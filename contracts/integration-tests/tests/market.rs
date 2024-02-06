#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

use std::ops::Sub;

use concordium_cis2::{
    AdditionalData, BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, Cis2Event,
    IsTokenAmount, IsTokenId, Receiver, TokenAmountU64, TokenAmountU8, TokenIdU8, TokenIdUnit,
    TokenIdVec, TransferParams,
};
use concordium_rwa_compliance::{
    compliance::init::InitParams as ComplianceInitParams,
    compliance_modules::allowed_nationalities::init::InitParams as ComplianceModuleInitParams,
};
use concordium_rwa_identity_registry::identities::RegisterIdentityParams;
use concordium_rwa_market::{
    event::{PaymentAmount, PaymentTokenUId},
    exchange::{Amounts, ExchangeParams},
    init::InitParams as MarketInitParams,
    list::{GetListedParam, ListParams},
    types::{ExchangeRate, Rate, TokenUId},
};
use concordium_rwa_security_nft::{
    event::Event as SecurityNftEvent,
    init::InitParam as SecurityNftInitParams,
    mint::{MintParam, MintParams as SecurityNftMintParams},
    types::{ContractMetadataUrl, TokenId},
};
use concordium_rwa_utils::common_types::{Identity, IdentityAttribute};
use concordium_smart_contract_testing::*;
use concordium_std::{Cursor, Deserial, ExpectReport, Serial};
use euroe_stablecoin::{MintParams as EuroEMintParams, RoleTypes};

const NATIONALITY_ATTRIBUTE_TAG: u8 = 5;
const DEFAULT_ACC_BALANCE: Amount = Amount {
    micro_ccd: 1_000_000_000_u64,
};
const ADMIN: AccountAddress = AccountAddress([0; 32]);
const IDENTITY_REGISTRY_AGENT: AccountAddress = AccountAddress([1; 32]);
const SELLER_ACC: AccountAddress = AccountAddress([2; 32]);
const BUYER_ACC: AccountAddress = AccountAddress([3; 32]);
const BUYER_ACC_NON_COMPLIANT: AccountAddress = AccountAddress([4; 32]);

const IDENTITY_REGISTRY_MODULE: &str = "../identity-registry/contract.wasm.v1";
const IR_CONTRACT_NAME: &str = "init_rwa_identity_registry";
const COMPLIANCE_MODULE: &str = "../compliance/contract.wasm.v1";
const COMPLIANCE_CONTRACT_NAME: &str = "init_rwa_compliance";
const COMPLIANCE_MODULE_CONTRACT_NAME: &str = "init_rwa_compliance_module_allowed_nationalities";
const SECURITY_NFT_MODULE: &str = "../security-nft/contract.wasm.v1";
const SECURITY_NFT_CONTRACT_NAME: &str = "init_rwa_security_nft";
const MARKET_MODULE: &str = "../market/contract.wasm.v1";
const MARKET_CONTRACT_NAME: &str = "init_rwa_market";
const EUROE_MODULE: &str = "../euroe/dist/module.wasm.v1";
const EUROE_CONTRACT_NAME: &str = "init_euroe_stablecoin";

#[test]
fn market_buy_via_transfer_of_cis2() {
    let mut chain = Chain::new();
    chain.create_account(Account::new_with_balance(
        ADMIN,
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
    ));
    chain.create_account(Account::new_with_balance(
        IDENTITY_REGISTRY_AGENT,
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
    ));
    chain.create_account(Account::new_with_balance(
        SELLER_ACC,
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
    ));
    chain.create_account(Account::new_with_balance(
        BUYER_ACC,
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
    ));
    chain.create_account(Account::new_with_balance(
        BUYER_ACC_NON_COMPLIANT,
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
    ));
    let ir_contract = identity_registry_deploy_and_init(&mut chain, ADMIN);
    let compliance_contract = compliance_deploy_and_init(&mut chain, ir_contract, ADMIN, vec![
        "IN".to_owned(),
        "US".to_owned(),
    ]);
    let security_nft_contract =
        security_nft_deploy_and_init(&mut chain, ADMIN, compliance_contract, ir_contract);
    let euroe_contract = euroe_deploy_and_init(&mut chain, ADMIN);
    let market_contract = market_deploy_and_init(
        &mut chain,
        ADMIN,
        vec![security_nft_contract],
        vec![TokenUId {
            // Euro E has a Unit Token Id
            id:       TokenIdVec(Vec::new()),
            contract: euroe_contract,
        }],
        Rate {
            numerator:   1,
            denominator: 10,
        },
    );

    let init_euroe_balance = TokenAmountU64(400_000_000);
    chain
        .contract_update(
            Signer::with_one_key(),
            ADMIN,
            Address::Account(ADMIN),
            Energy::from(10000),
            UpdateContractPayload {
                amount:       Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("euroe_stablecoin.mint".to_string()),
                address:      euroe_contract,
                message:      OwnedParameter::from_serial(&EuroEMintParams {
                    owner:  Address::Account(BUYER_ACC),
                    amount: init_euroe_balance,
                })
                .expect_report("Mint params"),
            },
        )
        .expect_report("Mint EuroE tokens buyer");
    add_identity_nationality(
        &mut chain,
        ir_contract,
        ADMIN,
        Address::Contract(market_contract),
        "IN",
    )
    .expect_report("Add market Identity");
    add_identity_nationality(&mut chain, ir_contract, ADMIN, Address::Account(BUYER_ACC), "IN")
        .expect_report("Add buyer Identity");
    add_identity_nationality(
        &mut chain,
        ir_contract,
        ADMIN,
        Address::Account(BUYER_ACC_NON_COMPLIANT),
        "DK",
    )
    .expect_report("Add non compliant buyer Identity");
    add_identity_nationality(&mut chain, ir_contract, ADMIN, Address::Account(SELLER_ACC), "US")
        .expect_report("Add seller Identity");

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
            id:       TokenIdVec(Vec::new()),
        },
        Rate {
            numerator:   2_000_000,
            denominator: 1,
        },
    ));

    // Listing Of Token
    nft_transfer_and_list(
        &mut chain,
        security_nft_contract,
        SELLER_ACC,
        buy_token,
        market_contract,
        vec![euroe_exchange_rate.to_owned()],
    )
    .expect_report("Security NFT: Transfer and List token 1");

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
            id:       TokenIdVec(Vec::new()),
        }),
        "Invalid Calculated Pay Token Id"
    );

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

fn nft_balance_of(
    chain: &mut Chain,
    security_nft_contract: ContractAddress,
    security_nft_token_id: TokenIdU8,
    owner: Address,
) -> TokenAmountU8 {
    let balances = chain
        .contract_invoke(
            ADMIN,
            Address::Account(ADMIN),
            Energy::from(10000),
            UpdateContractPayload {
                amount:       Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "rwa_security_nft.balanceOf".to_string(),
                ),
                address:      security_nft_contract,
                message:      OwnedParameter::from_serial(&BalanceOfQueryParams {
                    queries: vec![BalanceOfQuery {
                        address:  owner,
                        token_id: security_nft_token_id,
                    }],
                })
                .expect_report("Serialize Balance Of Query Params"),
            },
        )
        .expect_report("Security Nft: Balance Of")
        .parse_return_value::<BalanceOfQueryResponse<TokenAmountU8>>()
        .expect_report("Parsed Balance of Security Nft Token");

    let balance = balances.0.first().expect_report("Balance of Security Nft Token");
    *balance
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
                amount:       Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "euroe_stablecoin.balanceOf".to_string(),
                ),
                address:      euroe_contract,
                message:      OwnedParameter::from_serial(&BalanceOfQueryParams {
                    queries: vec![BalanceOfQuery {
                        address:  owner,
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
        .contract_init(Signer::with_one_key(), owner, Energy::from(30000), InitContractPayload {
            mod_ref:   market_module,
            amount:    Amount::zero(),
            init_name: OwnedContractName::new_unchecked(MARKET_CONTRACT_NAME.to_owned()),
            param:     OwnedParameter::from_serial(&MarketInitParams {
                token_contracts,
                commission,
                exchange_tokens,
            })
            .unwrap(),
        })
        .expect_report("Market: Init")
        .contract_address
}

fn euroe_deploy_and_init(chain: &mut Chain, owner: AccountAddress) -> ContractAddress {
    let euroe_module = chain
        .module_deploy_v1(Signer::with_one_key(), owner, module_load_v1(EUROE_MODULE).unwrap())
        .unwrap()
        .module_reference;
    let euroe_contract = chain
        .contract_init(Signer::with_one_key(), owner, Energy::from(30000), InitContractPayload {
            mod_ref:   euroe_module,
            amount:    Amount::zero(),
            init_name: OwnedContractName::new_unchecked(EUROE_CONTRACT_NAME.to_owned()),
            param:     OwnedParameter::empty(),
        })
        .expect_report("EuroE: Init")
        .contract_address;
    chain
        .contract_update(
            Signer::with_one_key(),
            owner,
            Address::Account(owner),
            Energy::from(10000),
            UpdateContractPayload {
                amount:       Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "euroe_stablecoin.grantRole".to_string(),
                ),
                address:      euroe_contract,
                message:      OwnedParameter::from_serial(&RoleTypes {
                    mintrole:  Address::Account(owner),
                    pauserole: Address::Account(owner),
                    burnrole:  Address::Account(owner),
                    blockrole: Address::Account(owner),
                    adminrole: Address::Account(owner),
                })
                .expect_report("Grant roles"),
            },
        )
        .expect_report("Grant roles");
    euroe_contract
}

fn security_nft_deploy_and_init(
    chain: &mut Chain,
    owner: AccountAddress,
    compliance_contract: ContractAddress,
    ir_contract: ContractAddress,
) -> ContractAddress {
    let security_nft_module = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            owner,
            module_load_v1(SECURITY_NFT_MODULE).unwrap(),
        )
        .unwrap()
        .module_reference;

    chain
        .contract_init(Signer::with_one_key(), owner, Energy::from(30000), InitContractPayload {
            mod_ref:   security_nft_module,
            amount:    Amount::zero(),
            init_name: OwnedContractName::new_unchecked(SECURITY_NFT_CONTRACT_NAME.to_owned()),
            param:     OwnedParameter::from_serial(&SecurityNftInitParams {
                compliance:        compliance_contract,
                identity_registry: ir_contract,
                sponsors:          vec![],
            })
            .expect_report("Security NFT: Init"),
        })
        .expect_report("Security NFT: Init")
        .contract_address
}

fn compliance_deploy_and_init(
    chain: &mut Chain,
    ir_contract: ContractAddress,
    owner: AccountAddress,
    nationalities: Vec<String>,
) -> ContractAddress {
    let compliance_module = chain
        .module_deploy_v1(Signer::with_one_key(), owner, module_load_v1(COMPLIANCE_MODULE).unwrap())
        .unwrap()
        .module_reference;
    let compliance_module_contract = chain
        .contract_init(Signer::with_one_key(), owner, Energy::from(30000), InitContractPayload {
            mod_ref:   compliance_module,
            amount:    Amount::zero(),
            init_name: OwnedContractName::new_unchecked(COMPLIANCE_MODULE_CONTRACT_NAME.to_owned()),
            param:     OwnedParameter::from_serial(&ComplianceModuleInitParams {
                identity_registry: ir_contract,
                nationalities,
            })
            .unwrap(),
        })
        .expect_report("Compliance Module: Init")
        .contract_address;

    chain
        .contract_init(Signer::with_one_key(), owner, Energy::from(30000), InitContractPayload {
            mod_ref:   compliance_module,
            amount:    Amount::zero(),
            init_name: OwnedContractName::new_unchecked(COMPLIANCE_CONTRACT_NAME.to_owned()),
            param:     OwnedParameter::from_serial(&ComplianceInitParams {
                modules: vec![compliance_module_contract],
            })
            .unwrap(),
        })
        .expect_report("Compliance: Init")
        .contract_address
}

fn identity_registry_deploy_and_init(chain: &mut Chain, owner: AccountAddress) -> ContractAddress {
    let ir_module = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            owner,
            module_load_v1(IDENTITY_REGISTRY_MODULE).unwrap(),
        )
        .expect_report("Identity Registry: Deploy")
        .module_reference;

    chain
        .contract_init(Signer::with_one_key(), owner, Energy::from(30000), InitContractPayload {
            mod_ref:   ir_module,
            amount:    Amount::zero(),
            init_name: OwnedContractName::new_unchecked(IR_CONTRACT_NAME.to_owned()),
            param:     OwnedParameter::empty(),
        })
        .expect_report("Identity Registry: Init")
        .contract_address
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
            id:       to_token_id_vec(buy_token_id),
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
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::new_unchecked("euroe_stablecoin.transfer".to_string()),
            address:      euroe_contract,
            message:      OwnedParameter::from_serial(&TransferParams(vec![
                concordium_cis2::Transfer {
                    token_id: TokenIdUnit(),
                    amount:   euroe_pay_amount,
                    from:     Address::Account(buyer_acc),
                    to:       Receiver::Contract(
                        market_contract,
                        OwnedEntrypointName::new_unchecked("deposit".to_string()),
                    ),
                    data:     AdditionalData::from(to_bytes(&exchange_params)),
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
                amount:       Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "rwa_market.balanceOfListed".to_string(),
                ),
                address:      market_contract,
                message:      OwnedParameter::from_serial(&GetListedParam {
                    owner:    SELLER_ACC,
                    token_id: TokenUId {
                        contract: token_contract,
                        id:       to_token_id_vec(token_id),
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
                amount:       Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "rwa_market.calculateAmounts".to_string(),
                ),
                address:      market_contract,
                message:      OwnedParameter::from_serial(&ExchangeParams {
                    token_id: TokenUId {
                        contract: buy_token_contract,
                        id:       to_token_id_vec(buy_token_id),
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

fn nft_transfer_and_list(
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
            id:       to_token_id_vec(token_id),
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
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::new_unchecked("rwa_security_nft.transfer".to_string()),
            address:      security_nft_contract,
            message:      OwnedParameter::from_serial(&TransferParams(vec![
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

fn nft_mint(
    chain: &mut Chain,
    security_nft_contract: ContractAddress,
    nft_agent: AccountAddress,
    owner: Receiver,
    metadata_url: &str,
) -> Result<TokenId, ContractInvokeError> {
    chain
        .contract_update(
            Signer::with_one_key(),
            nft_agent,
            Address::Account(nft_agent),
            Energy::from(30000),
            UpdateContractPayload {
                amount:       Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("rwa_security_nft.mint".to_string()),
                address:      security_nft_contract,
                message:      OwnedParameter::from_serial(&SecurityNftMintParams {
                    owner,
                    tokens: vec![MintParam {
                        metadata_url: ContractMetadataUrl {
                            url:  metadata_url.to_string(),
                            hash: None,
                        },
                    }],
                })
                .unwrap(),
            },
        )
        .map(|res| {
            res.events()
                .filter_map(|(contract_address, events)| {
                    if contract_address != security_nft_contract {
                        return None;
                    }

                    let binding = events
                        .iter()
                        .filter_map(|e| {
                            let e: SecurityNftEvent = e.parse().unwrap();
                            if let SecurityNftEvent::Cis2(Cis2Event::Mint(e)) = e {
                                Some(e.token_id)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    Some(binding)
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .map(|token_ids| *token_ids.first().expect_report("Minted Token"))
}

fn add_identity_nationality(
    chain: &mut Chain,
    ir_contract: ContractAddress,
    ir_agent: AccountAddress,
    identity_address: Address,
    identity_nationality: &str,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    chain.contract_update(
        Signer::with_one_key(),
        ir_agent,
        Address::Account(ir_agent),
        Energy::from(10000),
        UpdateContractPayload {
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::new_unchecked(
                "rwa_identity_registry.registerIdentity".to_string(),
            ),
            address:      ir_contract,
            message:      OwnedParameter::from_serial(&RegisterIdentityParams {
                identity: Identity {
                    attributes:  vec![IdentityAttribute {
                        tag:   NATIONALITY_ATTRIBUTE_TAG,
                        value: identity_nationality.to_owned(),
                    }],
                    credentials: vec![],
                },
                address:  identity_address,
            })
            .unwrap(),
        },
    )
}

fn to_token_id_vec<T: IsTokenId + Serial>(token_id: T) -> TokenIdVec {
    TokenIdVec(to_bytes(&token_id)[1..].to_vec())
}

fn to_token_amount_u64<A: IsTokenAmount + Serial>(token_amount: A) -> TokenAmountU64 {
    let mut token_amount_bytes = to_bytes(&token_amount);
    let mut cursor = Cursor::new(&mut token_amount_bytes);
    TokenAmountU64::deserial(&mut cursor).unwrap()
}
