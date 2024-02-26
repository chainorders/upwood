#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

use std::ops::Sub;
mod utils;
use crate::utils::{
    chain::{create_accounts, init_identity_contracts, init_security_token_contracts},
    security_nft::{nft_balance_of, nft_mint},
};
use concordium_cis2::{
    AdditionalData, BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, Receiver,
    TokenAmountU64, TokenAmountU8, TokenIdU8, TokenIdUnit, TokenIdVec, TransferParams,
};
use concordium_rwa_market::{
    event::{PaymentAmount, PaymentTokenUId},
    exchange::{Amounts, ExchangeParams},
    init::InitParams as MarketInitParams,
    list::{GetListedParam, ListParams},
    types::{ExchangeRate, Rate, TokenUId},
};
use concordium_smart_contract_testing::*;
use concordium_std::ExpectReport;
use euroe_stablecoin::{MintParams as EuroEMintParams, RoleTypes};
use utils::{consts::*, identity_registry::*};

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const IDENTITY_REGISTRY_AGENT: AccountAddress = AccountAddress([1; 32]);
const SELLER_ACC: AccountAddress = AccountAddress([2; 32]);
const BUYER_ACC: AccountAddress = AccountAddress([3; 32]);
const BUYER_ACC_NON_COMPLIANT: AccountAddress = AccountAddress([4; 32]);

#[test]
fn market_buy_via_transfer_of_cis2() {
    let mut chain = Chain::new();
    create_accounts(
        &mut chain,
        vec![
            DEFAULT_INVOKER,
            ADMIN,
            IDENTITY_REGISTRY_AGENT,
            SELLER_ACC,
            BUYER_ACC,
            BUYER_ACC_NON_COMPLIANT,
        ],
        DEFAULT_ACC_BALANCE,
    );

    let (ir_contract, _, compliance_contract) =
        init_identity_contracts(&mut chain, ADMIN, vec!["IN".to_owned(), "US".to_owned()]);
    add_identities(
        &mut chain,
        ir_contract,
        ADMIN,
        vec![
            (Address::Account(BUYER_ACC), "IN".to_string()),
            (Address::Account(SELLER_ACC), "US".to_string()),
            (Address::Account(BUYER_ACC_NON_COMPLIANT), "DK".to_string()),
        ],
    )
    .expect("Add Account identities");

    let (security_nft_contract, _) =
        init_security_token_contracts(&mut chain, ADMIN, ir_contract, compliance_contract, vec![]);
    let euroe_contract = euroe_deploy_and_init(&mut chain, ADMIN);
    let market_contract = market_deploy_and_init(
        &mut chain,
        ADMIN,
        vec![security_nft_contract],
        vec![TokenUId {
            // Euro E has a Unit Token Id
            id: TokenIdVec(Vec::new()),
            contract: euroe_contract,
        }],
        Rate {
            numerator: 1,
            denominator: 10,
        },
    );
    add_identities(
        &mut chain,
        ir_contract,
        ADMIN,
        vec![(Address::Contract(market_contract), "IN".to_string())],
    )
    .expect("Add Contract identities");

    let buy_token = nft_mint(
        &mut chain,
        security_nft_contract,
        ADMIN,
        Receiver::Account(SELLER_ACC),
        "ipfs:url1",
    )
    .expect_report("Security NFT: Mint token 1");

    let euroe_exchange_rate: ExchangeRate = ExchangeRate::Cis2((
        TokenUId {
            contract: euroe_contract,
            id: TokenIdVec(Vec::new()),
        },
        Rate {
            numerator: 2_000_000,
            denominator: 1,
        },
    ));

    // Listing Of Token
    market_transfer_and_list(
        &mut chain,
        security_nft_contract,
        SELLER_ACC,
        buy_token,
        market_contract,
        vec![euroe_exchange_rate.to_owned()],
    )
    .expect_report("Market: Transfer and List token 1");

    let balance_of_listed_buy_token =
        market_balance_of_listed(&mut chain, market_contract, security_nft_contract, buy_token);
    assert_eq!(balance_of_listed_buy_token, TokenAmountU64(1), "Token 1 listed");

    // Total amount of tokens to buy
    let buy_token_amount: TokenAmountU8 = TokenAmountU8(1);
    // Total amount of tokens to pay
    let pay_token_amount: TokenAmountU64 = TokenAmountU64(2_000_000);
    // Amount of Pay Token to be credited to the seller
    let token_owner_credited_amount = TokenAmountU64(1_800_000);
    // Amount of Pay Token to be credited to the market
    let market_commission_amount = TokenAmountU64(200_000);

    let amounts = market_calculate_amounts(
        &mut chain,
        market_contract,
        security_nft_contract,
        buy_token,
        buy_token_amount,
        SELLER_ACC,
        BUYER_ACC,
        euroe_exchange_rate.to_owned(),
    );
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
            contract: euroe_contract,
            id: TokenIdVec(Vec::new()),
        }),
        "Invalid Calculated Pay Token Id"
    );

    let init_euroe_balance = TokenAmountU64(400_000_000);
    chain
        .contract_update(
            Signer::with_one_key(),
            ADMIN,
            Address::Account(ADMIN),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("euroe_stablecoin.mint".to_string()),
                address: euroe_contract,
                message: OwnedParameter::from_serial(&EuroEMintParams {
                    owner: Address::Account(BUYER_ACC),
                    amount: init_euroe_balance,
                })
                .expect_report("Mint params"),
            },
        )
        .expect_report("Mint EuroE tokens buyer");
    // Buying of Token
    euroe_transfer_and_buy(
        &mut chain,
        security_nft_contract,
        buy_token,
        buy_token_amount,
        SELLER_ACC,
        euroe_contract,
        pay_token_amount,
        BUYER_ACC,
        market_contract,
        euroe_exchange_rate,
    )
    .expect_report("Euro E: Transfer and Buy Token 1");

    // Settlement Tests
    // Settlement of the Pay Token
    assert_eq!(
        market_balance_of_listed(&mut chain, market_contract, security_nft_contract, buy_token),
        TokenAmountU64(0),
        "Market Listed Balance"
    );
    assert_eq!(
        euroe_balance_of(&mut chain, euroe_contract, Address::Account(SELLER_ACC)),
        token_owner_credited_amount,
        "Seller EuroE balance"
    );
    assert_eq!(
        euroe_balance_of(&mut chain, euroe_contract, Address::Account(BUYER_ACC)),
        init_euroe_balance.sub(pay_token_amount),
        "Buyer EuroE balance"
    );
    assert_eq!(
        euroe_balance_of(&mut chain, euroe_contract, Address::Account(ADMIN)),
        market_commission_amount,
        "Commission EuroE balance"
    );
    // Settlement of the Buy Token
    assert_eq!(
        nft_balance_of(&mut chain, security_nft_contract, buy_token, Address::Account(SELLER_ACC)),
        TokenAmountU8(0),
        "Seller Security NFT balance"
    );
    assert_eq!(
        nft_balance_of(&mut chain, security_nft_contract, buy_token, Address::Account(BUYER_ACC)),
        TokenAmountU8(1),
        "Buyer Security NFT balance"
    );
}

fn euroe_balance_of(
    chain: &mut Chain,
    euroe_contract: ContractAddress,
    owner: Address,
) -> TokenAmountU64 {
    let balances = chain
        .contract_invoke(
            ADMIN,
            Address::Account(ADMIN),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "euroe_stablecoin.balanceOf".to_string(),
                ),
                address: euroe_contract,
                message: OwnedParameter::from_serial(&BalanceOfQueryParams {
                    queries: vec![BalanceOfQuery {
                        address: owner,
                        token_id: TokenIdUnit(),
                    }],
                })
                .expect_report("Serialize Balance Of Query Params"),
            },
        )
        .expect_report("EuroE: Balance Of")
        .parse_return_value::<BalanceOfQueryResponse<TokenAmountU64>>()
        .expect_report("Parsed Balance of EuroE");

    let balance = balances.0.first().expect_report("Balance of EuroE");
    *balance
}

fn market_deploy_and_init(
    chain: &mut Chain,
    owner: AccountAddress,
    token_contracts: Vec<ContractAddress>,
    exchange_tokens: Vec<TokenUId>,
    commission: Rate,
) -> ContractAddress {
    let market_module = chain
        .module_deploy_v1(Signer::with_one_key(), owner, module_load_v1(MARKET_MODULE).unwrap())
        .unwrap()
        .module_reference;

    chain
        .contract_init(
            Signer::with_one_key(),
            owner,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: market_module,
                amount: Amount::zero(),
                init_name: OwnedContractName::new_unchecked(MARKET_CONTRACT_NAME.to_owned()),
                param: OwnedParameter::from_serial(&MarketInitParams {
                    token_contracts,
                    commission,
                    exchange_tokens,
                })
                .unwrap(),
            },
        )
        .expect_report("Market: Init")
        .contract_address
}

fn euroe_deploy_and_init(chain: &mut Chain, owner: AccountAddress) -> ContractAddress {
    let euroe_module = chain
        .module_deploy_v1(Signer::with_one_key(), owner, module_load_v1(EUROE_MODULE).unwrap())
        .unwrap()
        .module_reference;
    let euroe_contract = chain
        .contract_init(
            Signer::with_one_key(),
            owner,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: euroe_module,
                amount: Amount::zero(),
                init_name: OwnedContractName::new_unchecked(EUROE_CONTRACT_NAME.to_owned()),
                param: OwnedParameter::empty(),
            },
        )
        .expect_report("EuroE: Init")
        .contract_address;
    chain
        .contract_update(
            Signer::with_one_key(),
            owner,
            Address::Account(owner),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "euroe_stablecoin.grantRole".to_string(),
                ),
                address: euroe_contract,
                message: OwnedParameter::from_serial(&RoleTypes {
                    mintrole: Address::Account(owner),
                    pauserole: Address::Account(owner),
                    burnrole: Address::Account(owner),
                    blockrole: Address::Account(owner),
                    adminrole: Address::Account(owner),
                })
                .expect_report("Grant roles"),
            },
        )
        .expect_report("Grant roles");
    euroe_contract
}

fn euroe_transfer_and_buy(
    chain: &mut Chain,
    buy_token_contract: ContractAddress,
    buy_token_id: TokenIdU8,
    buy_token_amount: TokenAmountU8,
    seller_acc: AccountAddress,
    euroe_contract: ContractAddress,
    euroe_pay_amount: TokenAmountU64,
    buyer_acc: AccountAddress,
    market_contract: ContractAddress,
    rate: ExchangeRate,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    let exchange_params = ExchangeParams {
        token_id: TokenUId {
            contract: buy_token_contract,
            id: to_token_id_vec(buy_token_id),
        },
        amount: to_token_amount_u64(buy_token_amount),
        owner: seller_acc,
        payer: buyer_acc,
        rate,
    };
    chain.contract_update(
        Signer::with_one_key(),
        buyer_acc,
        Address::Account(buyer_acc),
        Energy::from(30000),
        UpdateContractPayload {
            amount: Amount::zero(),
            receive_name: OwnedReceiveName::new_unchecked("euroe_stablecoin.transfer".to_string()),
            address: euroe_contract,
            message: OwnedParameter::from_serial(&TransferParams(vec![
                concordium_cis2::Transfer {
                    token_id: TokenIdUnit(),
                    amount: euroe_pay_amount,
                    from: Address::Account(buyer_acc),
                    to: Receiver::Contract(
                        market_contract,
                        OwnedEntrypointName::new_unchecked("deposit".to_string()),
                    ),
                    data: AdditionalData::from(to_bytes(&exchange_params)),
                },
            ]))
            .unwrap(),
        },
    )
}

fn market_balance_of_listed(
    chain: &mut Chain,
    market_contract: ContractAddress,
    token_contract: ContractAddress,
    token_id: TokenIdU8,
) -> TokenAmountU64 {
    chain
        .contract_invoke(
            ADMIN,
            Address::Account(ADMIN),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "rwa_market.balanceOfListed".to_string(),
                ),
                address: market_contract,
                message: OwnedParameter::from_serial(&GetListedParam {
                    owner: SELLER_ACC,
                    token_id: TokenUId {
                        contract: token_contract,
                        id: to_token_id_vec(token_id),
                    },
                })
                .expect_report("Serialize Get Listed Params"),
            },
        )
        .expect_report("Market: Balance Of Listed")
        .parse_return_value()
        .expect_report("Parsed Balance of listed token")
}

fn market_calculate_amounts(
    chain: &mut Chain,
    market_contract: ContractAddress,
    buy_token_contract: ContractAddress,
    buy_token_id: TokenIdU8,
    buy_token_amount: TokenAmountU8,
    seller_acc: AccountAddress,
    buyer_acc: AccountAddress,
    rate: ExchangeRate,
) -> Amounts {
    chain
        .contract_invoke(
            ADMIN,
            Address::Account(ADMIN),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "rwa_market.calculateAmounts".to_string(),
                ),
                address: market_contract,
                message: OwnedParameter::from_serial(&ExchangeParams {
                    token_id: TokenUId {
                        contract: buy_token_contract,
                        id: to_token_id_vec(buy_token_id),
                    },
                    amount: to_token_amount_u64(buy_token_amount),
                    owner: seller_acc,
                    payer: buyer_acc,
                    rate,
                })
                .expect_report("Serialize Exchange Params"),
            },
        )
        .expect_report("Market: Calculate Amounts")
        .parse_return_value()
        .expect_report("Parsed Amounts")
}

fn market_transfer_and_list(
    chain: &mut Chain,
    security_nft_contract: ContractAddress,
    from: AccountAddress,
    token_id: TokenIdU8,
    market_contract: ContractAddress,
    exchange_rates: Vec<ExchangeRate>,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    let token_1_list_params = ListParams {
        token_id: TokenUId {
            contract: security_nft_contract,
            id: to_token_id_vec(token_id),
        },
        supply: TokenAmountU64(1),
        owner: from,
        exchange_rates,
    };
    chain.contract_update(
        Signer::with_one_key(),
        from,
        Address::Account(from),
        Energy::from(30000),
        UpdateContractPayload {
            amount: Amount::zero(),
            receive_name: OwnedReceiveName::new_unchecked("rwa_security_nft.transfer".to_string()),
            address: security_nft_contract,
            message: OwnedParameter::from_serial(&TransferParams(vec![
                concordium_cis2::Transfer {
                    token_id,
                    amount: TokenAmountU8(1),
                    from: Address::Account(from),
                    to: Receiver::Contract(
                        market_contract,
                        OwnedEntrypointName::new_unchecked("deposit".to_string()),
                    ),
                    data: AdditionalData::from(to_bytes(&token_1_list_params)),
                },
            ]))
            .unwrap(),
        },
    )
}
