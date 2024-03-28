#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

pub mod utils;

use crate::utils::{
    common::{init_identity_contracts, init_security_token_contracts},
    consts::*,
};
use concordium_cis2::{
    AdditionalData, Receiver, TokenAmountU64, TokenAmountU8, TokenIdUnit, TokenIdVec,
    TransferParams,
};
use concordium_rwa_market::{
    event::{PaymentAmount, PaymentTokenUId},
    exchange::ExchangeParams,
    list::{GetListedParam, ListParams},
    types::{ExchangeRate, Rate, TokenUId},
};
use concordium_smart_contract_testing::{ed25519::PublicKey, *};
use concordium_std::{ops::Sub, ACCOUNT_ADDRESS_SIZE};
use euroe_stablecoin::RoleTypes;
use integration_tests::{
    cis2_test_contract::{ICis2Contract, ICis2ContractExt, ICis2ContractUnitTokenExt},
    euroe::{EuroeContract, EuroeModule, IEuroeContract, IEuroeModule},
    identity_registry::IIdentityRegistryContract,
    market::{IMarketContract, IMarketModule, MarketContract, MarketModule},
    security_nft::{ISecurityNftContract, ISecurityNftContractExt},
    test_contract_client::{ITestContract, ITestModule},
    verifier::Verifier,
};

#[test]
fn market_buy_via_transfer_of_cis2() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let admin = Account::new_with_keys(
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let ir_agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let seller = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let buyer = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let nft_agent = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    vec![admin.clone(), ir_agent.clone(), seller.clone(), buyer.clone(), nft_agent.clone()]
        .iter()
        .for_each(|a| {
            chain.create_account(a.clone());
        });

    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, &admin, compliant_nationalities.to_vec());
    ir_contract
        .add_agent()
        .update(&mut chain, &admin, &Address::Account(ir_agent.address))
        .expect("Identity Register : Add Agent");
    let market_module = MarketModule {
        module_path: MARKET_MODULE.to_owned(),
    };
    market_module.deploy(&mut chain, &admin).expect("market: deploy");
    let euroe_module = EuroeModule {
        module_path: EUROE_MODULE.to_owned(),
    };
    euroe_module.deploy(&mut chain, &admin).expect("euroe: deploy");

    let verifier = Verifier {
        account:           ir_agent.clone(),
        identity_registry: ir_contract.clone(),
    };
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(buyer.address), compliant_nationalities[0].clone()),
            (Address::Account(seller.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let (nft, _) = init_security_token_contracts(
        &mut chain,
        &admin,
        &ir_contract,
        &compliance_contract,
        vec![],
    )
    .expect("Init Security Token Contracts");
    nft.add_agent()
        .update(&mut chain, &admin, &Address::Account(nft_agent.address))
        .expect("Security NFT: Add Agent");

    let euroe = euroe_module
        .euroe()
        .init(&mut chain, &admin, &())
        .map(|r| EuroeContract(r.contract_address))
        .expect("euroe init");

    let market = market_module
        .rwa_market()
        .init(&mut chain, &admin, &concordium_rwa_market::init::InitParams {
            commission:      Rate {
                numerator:   1,
                denominator: 10,
            },
            token_contracts: vec![nft.contract_address()],
            exchange_tokens: vec![TokenUId {
                // Euro E has a Unit Token Id
                id:       TokenIdVec(Vec::new()),
                contract: euroe.contract_address(),
            }],
        })
        .map(|r| MarketContract(r.contract_address))
        .expect("Market init");

    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Contract(market.contract_address()),
            "IN".to_owned(),
        )])
        .expect("verifier: add market contract identities");

    let buy_token = nft
        .mint_single_update(
            &mut chain,
            &nft_agent,
            Receiver::Account(seller.address),
            concordium_rwa_security_nft::types::ContractMetadataUrl {
                url:  "ipfs:url1".to_owned(),
                hash: None,
            },
        )
        .expect("nft: mint buy token");

    let euroe_exchange_rate: ExchangeRate = ExchangeRate::Cis2((
        TokenUId {
            contract: euroe.contract_address(),
            id:       TokenIdVec(Vec::new()),
        },
        Rate {
            numerator:   2_000_000,
            denominator: 1,
        },
    ));
    let nft_token_uid = TokenUId {
        contract: nft.contract_address(),
        id:       to_token_id_vec(buy_token),
    };

    nft.transfer()
        .update(
            &mut chain,
            &seller,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(seller.address),
                amount:   1.into(),
                to:       Receiver::Contract(
                    market.contract_address(),
                    market.deposit().entrypoint_name,
                ),
                token_id: buy_token,
                data:     AdditionalData::from(
                    to_bytes(&ListParams {
                        token_id:       nft_token_uid.clone(),
                        supply:         TokenAmountU64(1),
                        owner:          seller.address,
                        exchange_rates: vec![euroe_exchange_rate.clone()],
                    })
                    .to_vec(),
                ),
            }]),
        )
        .expect("nft transfer to market with list info");
    let balance_of_listed_buy_token = market
        .balance_of_listed()
        .invoke(&mut chain, &seller, &concordium_rwa_market::list::GetListedParam {
            token_id: nft_token_uid.clone(),
            owner:    seller.address,
        })
        .map(|r| market.balance_of_listed().parse_return_value(&r).expect("parsing error"))
        .expect("market: balance of listed");
    assert_eq!(balance_of_listed_buy_token, TokenAmountU64(1), "Token 1 listed");

    // Total amount of tokens to buy
    let buy_token_amount: TokenAmountU8 = TokenAmountU8(1);
    // Total amount of tokens to pay
    let pay_token_amount: TokenAmountU64 = TokenAmountU64(2_000_000);
    // Amount of Pay Token to be credited to the seller
    let token_owner_credited_amount = TokenAmountU64(1_800_000);
    // Amount of Pay Token to be credited to the market
    let market_commission_amount = TokenAmountU64(200_000);

    let amounts = market
        .calculate_amounts()
        .invoke(&mut chain, &buyer, &concordium_rwa_market::exchange::ExchangeParams {
            token_id: nft_token_uid.clone(),
            owner:    seller.address,
            amount:   to_token_amount_u64(buy_token_amount),
            rate:     euroe_exchange_rate.clone(),
            payer:    seller.address,
        })
        .map(|r| {
            market
                .calculate_amounts()
                .parse_return_value(&r)
                .expect("Calculate Amounts Parsing return value")
        })
        .expect("market: calculate amounts");

    assert_eq!(amounts.buy, to_token_amount_u64(buy_token_amount), "Invalid Calculated Buy Amount");
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
            contract: euroe.contract_address(),
            id:       TokenIdVec(Vec::new()),
        }),
        "Invalid Calculated Pay Token Id"
    );

    euroe
        .grant_role()
        .update(&mut chain, &admin, &RoleTypes {
            mintrole:  Address::Account(admin.address),
            burnrole:  Address::Account(admin.address),
            blockrole: Address::Account(admin.address),
            pauserole: Address::Account(admin.address),
            adminrole: Address::Account(admin.address),
        })
        .expect("euroe: roles");
    let init_euroe_balance = TokenAmountU64(400_000_000);
    euroe
        .mint()
        .update(&mut chain, &admin, &euroe_stablecoin::MintParams {
            owner:  Address::Account(buyer.address),
            amount: init_euroe_balance,
        })
        .expect("Mint EuroE tokens buyer");

    euroe
        .transfer()
        .update(
            &mut chain,
            &buyer,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(buyer.address),
                amount:   pay_token_amount,
                to:       Receiver::Contract(
                    market.contract_address(),
                    market.deposit().entrypoint_name,
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
            }]),
        )
        .expect("euroe transfer to market with exchange params");

    // Settlement Tests
    // Settlement of the Pay Token
    let market_balance_of_listed = market
        .balance_of_listed()
        .invoke(&mut chain, &admin, &GetListedParam {
            owner:    seller.address,
            token_id: nft_token_uid,
        })
        .map(|r| {
            market
                .balance_of_listed()
                .parse_return_value(&r)
                .expect("market: balance of listed - parsing return value")
        })
        .expect("market: balance of listed");
    assert_eq!(market_balance_of_listed, TokenAmountU64(0), "Market Listed Balance");

    let euroe_balance_seller = euroe
        .balance_of_single_invoke(&mut chain, &admin, Address::Account(seller.address))
        .expect("euroe: balance of buyer");
    assert_eq!(euroe_balance_seller, token_owner_credited_amount, "Seller EuroE balance");

    let euroe_balance_buyer = euroe
        .balance_of_single_invoke(&mut chain, &admin, Address::Account(buyer.address))
        .expect("euroe: balance of buyer");

    assert_eq!(
        euroe_balance_buyer,
        init_euroe_balance.sub(pay_token_amount),
        "Buyer EuroE balance"
    );
    let euroe_balance_contract_owner = euroe
        .balance_of_single_invoke(&mut chain, &admin, Address::Account(admin.address))
        .expect("euroe: balance of buyer");
    assert_eq!(euroe_balance_contract_owner, market_commission_amount, "Commission EuroE balance");

    // Settlement of the Buy Token
    let buy_token_balance_of_seller = nft
        .balance_of_single_invoke(&mut chain, &admin, buy_token, Address::Account(seller.address))
        .expect("Seller NFT balance");
    assert_eq!(buy_token_balance_of_seller, TokenAmountU8(0), "Seller Security NFT balance");

    let buy_token_balance_of_buyer = nft
        .balance_of_single_invoke(&mut chain, &admin, buy_token, Address::Account(buyer.address))
        .expect("Buyer NFT balance");
    assert_eq!(buy_token_balance_of_buyer, TokenAmountU8(1), "Buyer Security NFT balance");
}
