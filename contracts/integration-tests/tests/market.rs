#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

pub mod utils;

use concordium_cis2::{
    AdditionalData, Receiver, TokenAmountU64, TokenAmountU8, TokenIdU32, TokenIdUnit, TokenIdVec,
};
use concordium_protocols::concordium_cis2_security::AgentWithRoles;
use concordium_rwa_market::event::{PaymentAmount, PaymentTokenUId};
use concordium_rwa_market::exchange::ExchangeParams;
use concordium_rwa_market::list::ListParams;
use concordium_rwa_market::types::{ExchangeRate, Rate, TokenUId};
use concordium_smart_contract_testing::*;
use concordium_std::ops::Sub;
use security_sft_rewards::types::{AgentRole, ContractMetadataUrl, InitParam, MintParam};
use utils::cis2_conversions::*;
use utils::*;
pub const DEFAULT_ACC_BALANCE: Amount = Amount {
    micro_ccd: 1_000_000_000_u64,
};
#[test]
fn market_buy_via_transfer_of_cis2() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let admin = chain
        .create_account(Account::new(AccountAddress([0; 32]), DEFAULT_ACC_BALANCE))
        .unwrap();
    let ir_agent = chain
        .create_account(Account::new(AccountAddress([1; 32]), DEFAULT_ACC_BALANCE))
        .unwrap();
    let seller = chain
        .create_account(Account::new(AccountAddress([2; 32]), DEFAULT_ACC_BALANCE))
        .unwrap();
    let buyer = chain
        .create_account(Account::new(AccountAddress([3; 32]), DEFAULT_ACC_BALANCE))
        .unwrap();
    let nft_agent = chain
        .create_account(Account::new(AccountAddress([4; 32]), DEFAULT_ACC_BALANCE))
        .unwrap();
    euroe::deploy_module(&mut chain, &admin);
    let euroe = euroe::init(&mut chain, &admin).contract_address;
    euroe::grant_role(&mut chain, &admin, euroe, &euroe::RoleTypes {
        mintrole:  Address::Account(admin.address),
        burnrole:  Address::Account(admin.address),
        blockrole: Address::Account(admin.address),
        pauserole: Address::Account(admin.address),
        adminrole: Address::Account(admin.address),
    });

    market::deploy_module(&mut chain, &admin);
    identity_registry::deploy_module(&mut chain, &admin);
    compliance::deploy_module(&mut chain, &admin);
    sft_security::deploy_module(&mut chain, &admin);
    let ir_contract = identity_registry::init(&mut chain, &admin).contract_address;
    let compliance_contract = compliance::init_all(
        &mut chain,
        &admin,
        ir_contract,
        compliant_nationalities.to_vec(),
    )
    .contract_address;
    identity_registry::add_agent(
        &mut chain,
        &admin,
        &ir_contract,
        &Address::Account(ir_agent.address),
    );

    let token_contract = sft_security::init(&mut chain, &admin, &InitParam {
        compliance:        compliance_contract,
        identity_registry: ir_contract,
        metadata_url:      ContractMetadataUrl {
            hash: None,
            url:  "example.com".to_string(),
        },
        sponsors:          vec![],
    })
    .contract_address;
    sft_security::add_agent(&mut chain, &admin, token_contract, &AgentWithRoles {
        address: Address::Account(nft_agent.address),
        roles:   vec![AgentRole::Mint],
    });
    let market = market::init(
        &mut chain,
        &admin,
        &concordium_rwa_market::init::InitParams {
            commission:      Rate {
                numerator:   1,
                denominator: 10,
            },
            token_contracts: vec![token_contract],
            exchange_tokens: vec![TokenUId {
                // Euro E has a Unit Token Id
                id:       to_token_id_vec(TokenIdUnit()),
                contract: euroe,
            }],
        },
    )
    .contract_address;
    identity_registry::register_nationalities(&mut chain, &ir_agent, &ir_contract, vec![(
        Address::Contract(market),
        "IN".to_owned(),
    )]);
    identity_registry::register_nationalities(&mut chain, &ir_agent, &ir_contract, vec![
        (
            Address::Account(buyer.address),
            compliant_nationalities[0].clone(),
        ),
        (
            Address::Account(seller.address),
            compliant_nationalities[1].clone(),
        ),
    ]);
    let buy_token = TokenIdU32(0);
    sft_security::mint(&mut chain, &nft_agent, &token_contract, &MintParam {
        amount:   TokenAmountU64(1),
        owner:    Receiver::Account(seller.address),
        token_id: buy_token,
    });

    let euroe_exchange_rate: ExchangeRate = ExchangeRate::Cis2((
        TokenUId {
            contract: euroe,
            id:       to_token_id_vec(TokenIdUnit()),
        },
        Rate {
            numerator:   2_000_000,
            denominator: 1,
        },
    ));
    let nft_token_uid = TokenUId {
        contract: token_contract,
        id:       to_token_id_vec(buy_token),
    };

    sft_security::transfer_single(
        &mut chain,
        &seller,
        token_contract,
        concordium_cis2::Transfer {
            from:     Address::Account(seller.address),
            amount:   1.into(),
            to:       Receiver::Contract(
                market,
                OwnedEntrypointName::new_unchecked(market::DEPOSIT_RECEIVE_NAME.to_string()),
            ),
            token_id: buy_token,
            data:     to_bytes(&ListParams {
                token_id:       nft_token_uid.clone(),
                supply:         TokenAmountU64(1),
                owner:          seller.address,
                exchange_rates: vec![euroe_exchange_rate.clone()],
            })
            .into(),
        },
    );

    assert_eq!(
        market::balance_of_listed(
            &mut chain,
            &seller,
            &market,
            &concordium_rwa_market::list::GetListedParam {
                token_id: nft_token_uid.clone(),
                owner:    seller.address,
            },
        ),
        TokenAmountU64(1),
        "Token 1 listed"
    );

    // Total amount of tokens to buy
    let buy_token_amount: TokenAmountU8 = TokenAmountU8(1);
    // Total amount of tokens to pay
    let pay_token_amount: TokenAmountU64 = TokenAmountU64(2_000_000);
    // Amount of Pay Token to be credited to the seller
    let token_owner_credited_amount = TokenAmountU64(1_800_000);
    // Amount of Pay Token to be credited to the market
    let market_commission_amount = TokenAmountU64(200_000);

    let amounts = market::calculate_amounts(
        &mut chain,
        &buyer,
        &market,
        &concordium_rwa_market::exchange::ExchangeParams {
            token_id: nft_token_uid.clone(),
            owner:    seller.address,
            amount:   to_token_amount_u64(buy_token_amount),
            rate:     euroe_exchange_rate.clone(),
            payer:    seller.address,
        },
    );

    assert_eq!(
        amounts.buy,
        to_token_amount_u64(buy_token_amount),
        "Invalid Calculated Buy Amount"
    );
    assert_eq!(
        amounts.pay,
        PaymentAmount::Cis2(token_owner_credited_amount),
        "Invalid Calculated Pay Amount"
    );
    assert_eq!(
        amounts.commission,
        PaymentAmount::Cis2(market_commission_amount),
        "Invalid Calculated Commission Amount"
    );
    assert_eq!(
        amounts.pay_token,
        PaymentTokenUId::Cis2(TokenUId {
            contract: euroe,
            id:       TokenIdVec(Vec::new()),
        }),
        "Invalid Calculated Pay Token Id"
    );

    let init_euroe_balance = TokenAmountU64(400_000_000);
    euroe::mint(&mut chain, &admin, euroe, &euroe::MintParams {
        owner:  Address::Account(buyer.address),
        amount: init_euroe_balance,
    });
    euroe::transfer_single(&mut chain, &buyer, euroe, concordium_cis2::Transfer {
        from:     Address::Account(buyer.address),
        amount:   pay_token_amount,
        to:       Receiver::Contract(
            market,
            OwnedEntrypointName::new_unchecked(market::DEPOSIT_RECEIVE_NAME.to_string()),
        ),
        token_id: TokenIdUnit(),
        data:     AdditionalData::from(
            to_bytes(&ExchangeParams {
                token_id: nft_token_uid.clone(),
                amount:   to_token_amount_u64(buy_token_amount),
                owner:    seller.address,
                payer:    buyer.address,
                rate:     euroe_exchange_rate,
            })
            .to_vec(),
        ),
    });

    // Settlement Tests
    // Settlement of the Pay Token
    assert_eq!(
        market::balance_of_listed(
            &mut chain,
            &admin,
            &market,
            &concordium_rwa_market::list::GetListedParam {
                owner:    seller.address,
                token_id: nft_token_uid,
            },
        ),
        TokenAmountU64(0),
        "Market Listed Balance"
    );

    assert_eq!(
        euroe::balance_of_single(&mut chain, &admin, euroe, Address::Account(seller.address)),
        token_owner_credited_amount,
        "Seller EuroE balance"
    );
    assert_eq!(
        euroe::balance_of_single(&mut chain, &admin, euroe, Address::Account(buyer.address)),
        init_euroe_balance.sub(pay_token_amount),
        "Buyer EuroE balance"
    );
    assert_eq!(
        euroe::balance_of_single(&mut chain, &admin, euroe, Address::Account(admin.address)),
        market_commission_amount,
        "Commission EuroE balance"
    );

    // Settlement of the Buy Token
    assert_eq!(
        cis2::balance_of_single::<TokenIdU32, TokenAmountU64>(
            &mut chain,
            &admin,
            token_contract,
            TokenIdU32(0),
            Address::Account(seller.address)
        ),
        TokenAmountU64(0),
        "Seller balance"
    );
    assert_eq!(
        cis2::balance_of_single::<TokenIdU32, TokenAmountU64>(
            &mut chain,
            &admin,
            token_contract,
            TokenIdU32(0),
            Address::Account(buyer.address)
        ),
        TokenAmountU64(1),
        "Buyer balance"
    );
}
