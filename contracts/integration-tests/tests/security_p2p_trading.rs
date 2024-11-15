#![cfg(test)]

use cis2_conversions::to_token_id_vec;
use compliance::init_nationalities;
use concordium_cis2::{
    BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, OperatorUpdate, TokenAmountU64,
    TokenIdUnit, UpdateOperator,
};
use concordium_protocols::concordium_cis2_security::TokenUId;
use concordium_protocols::rate::Rate;
use concordium_smart_contract_testing::*;
use concordium_std::{AccountAddress, Amount};
use integration_tests::*;
use security_p2p_trading::{
    Deposit, ExchangeParams, GetDepositParams, InitParam, TransferExchangeParams,
    TransferSellParams,
};
use security_sft_rewards::types::TRACKED_TOKEN_ID;
use security_sft_single::types::ContractMetadataUrl;

const TOKEN_ID: TokenIdUnit = TokenIdUnit();
const METADATA_URL_SFT_SINGLE: &str = "example.com";
const METADATA_URL_SFT_REWARDS: &str = "example2.com";
const MIN_REWARD_METADATA_URL: &str = "blank_reward.example.com";
const ADMIN: AccountAddress = AccountAddress([0; 32]);
const HOLDER: AccountAddress = AccountAddress([2; 32]);
const HOLDER_2: AccountAddress = AccountAddress([3; 32]);
const COMPLIANT_NATIONALITIES: [&str; 2] = ["IN", "US"];
const DEFAULT_ACC_BALANCE: Amount = Amount {
    micro_ccd: 1_000_000_000_u64,
};

#[test]
pub fn normal_flow_sft_single() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let holder = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(holder.clone());
    let holder_2 = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());

    let (euroe_contract, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    let token_contract =
        create_token_contract_sft_single(&mut chain, &admin, compliance_contract, ir_contract)
            .0
            .contract_address;
    let trading_contract = security_p2p_trading_client::init(&mut chain, &admin, &InitParam {
        currency: TokenUId {
            id:       to_token_id_vec(TokenIdUnit()),
            contract: euroe_contract,
        },
        token:    TokenUId {
            id:       to_token_id_vec(TOKEN_ID),
            contract: token_contract,
        },
    })
    .expect("init trading contract")
    .contract_address;

    identity_registry::register_nationalities(&mut chain, &admin, &ir_contract, vec![
        (Address::Account(holder.address), COMPLIANT_NATIONALITIES[1]),
        (
            Address::Account(holder_2.address),
            COMPLIANT_NATIONALITIES[1],
        ),
        (
            Address::Contract(trading_contract),
            COMPLIANT_NATIONALITIES[1],
        ),
    ]);

    security_sft_single_client::mint(
        &mut chain,
        &admin,
        &token_contract,
        &security_sft_single::types::MintParams {
            owners:   vec![security_sft_single::types::MintParam {
                amount:  TokenAmountU64(50),
                address: holder.address,
            }],
            token_id: TOKEN_ID,
        },
    )
    .expect("should mint");
    security_sft_single_client::update_operator_single(
        &mut chain,
        &holder,
        token_contract,
        UpdateOperator {
            update:   OperatorUpdate::Add,
            operator: trading_contract.into(),
        },
    )
    .expect("should update operator");

    let rate = Rate::new(1, 1000).unwrap();
    security_p2p_trading_client::transfer_sell(
        &mut chain,
        &holder,
        trading_contract,
        &TransferSellParams {
            amount: TokenAmountU64(10),
            rate,
        },
    )
    .expect("should transfer sell");
    assert_eq!(
        security_sft_single_client::balance_of_single(
            &mut chain,
            &admin,
            token_contract,
            TOKEN_ID,
            holder.address.into()
        )
        .expect("should get balance"),
        TokenAmountU64(40)
    );
    assert_eq!(
        security_p2p_trading_client::get_deposit(
            &mut chain,
            &admin,
            trading_contract,
            &GetDepositParams {
                from: holder.address,
            }
        )
        .expect("should get deposit"),
        Deposit {
            amount: TokenAmountU64(10),
            rate
        }
    );

    euroe::update_operator_single(&mut chain, &holder_2, euroe_contract, UpdateOperator {
        update:   OperatorUpdate::Add,
        operator: trading_contract.into(),
    });
    euroe::mint(&mut chain, &admin, euroe_contract, &euroe::MintParams {
        owner:  holder_2.address.into(),
        amount: TokenAmountU64(1000),
    })
    .expect("euroe mint");
    security_p2p_trading_client::transfer_exchange(
        &mut chain,
        &holder_2,
        trading_contract,
        &TransferExchangeParams {
            pay: TokenAmountU64(1000),
            get: ExchangeParams {
                from: holder.address,
                rate,
            },
        },
    )
    .expect("should transfer buy");
    assert_eq!(
        euroe::balance_of_single(
            &mut chain,
            &holder_2,
            euroe_contract,
            holder_2.address.into()
        ),
        TokenAmountU64(0)
    );
    assert_eq!(
        security_p2p_trading_client::get_deposit(
            &mut chain,
            &admin,
            trading_contract,
            &GetDepositParams {
                from: holder.address,
            }
        )
        .expect("should get deposit"),
        Deposit {
            amount: TokenAmountU64(9),
            rate
        }
    );
    assert_eq!(
        security_sft_single_client::balance_of(
            &mut chain,
            &admin,
            token_contract,
            &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: TOKEN_ID,
                        address:  holder.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: TOKEN_ID,
                        address:  holder_2.address.into(),
                    }
                ],
            }
        )
        .expect("should get balance"),
        BalanceOfQueryResponse(vec![TokenAmountU64(40), TokenAmountU64(1),])
    );
}

#[test]
pub fn normal_flow_sft_rewards() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);

    let mut chain = Chain::new();
    let seller = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(seller.clone());
    let buyer = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(buyer.clone());

    let (euroe_contract, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    let token_contract =
        create_token_contract_sft_rewards(&mut chain, &admin, compliance_contract, ir_contract);
    let trading_contract = security_p2p_trading_client::init(&mut chain, &admin, &InitParam {
        currency: TokenUId {
            id:       to_token_id_vec(TokenIdUnit()),
            contract: euroe_contract,
        },
        token:    TokenUId {
            id:       to_token_id_vec(TRACKED_TOKEN_ID),
            contract: token_contract,
        },
    })
    .expect("init trading contract")
    .contract_address;

    identity_registry::register_nationalities(&mut chain, &admin, &ir_contract, vec![
        (Address::Account(seller.address), COMPLIANT_NATIONALITIES[1]),
        (Address::Account(buyer.address), COMPLIANT_NATIONALITIES[1]),
        (
            Address::Contract(trading_contract),
            COMPLIANT_NATIONALITIES[1],
        ),
    ]);

    security_sft_rewards_client::mint_raw(
        &mut chain,
        &admin,
        &token_contract,
        &security_sft_rewards::types::MintParams {
            owners:   vec![security_sft_rewards::types::MintParam {
                amount:  TokenAmountU64(50),
                address: seller.address,
            }],
            token_id: TRACKED_TOKEN_ID,
        },
    )
    .expect("should mint");
    security_sft_rewards_client::update_operator_single(
        &mut chain,
        &seller,
        token_contract,
        UpdateOperator {
            update:   OperatorUpdate::Add,
            operator: trading_contract.into(),
        },
    )
    .expect("should update operator");

    let rate = Rate::new(1, 1000).unwrap();
    security_p2p_trading_client::transfer_sell(
        &mut chain,
        &seller,
        trading_contract,
        &TransferSellParams {
            amount: TokenAmountU64(10),
            rate,
        },
    )
    .expect("should transfer sell");
    assert_eq!(
        security_sft_rewards_client::balance_of_single(
            &mut chain,
            &admin,
            token_contract,
            TRACKED_TOKEN_ID,
            seller.address.into()
        )
        .expect("should get balance"),
        TokenAmountU64(40)
    );
    assert_eq!(
        security_p2p_trading_client::get_deposit(
            &mut chain,
            &admin,
            trading_contract,
            &GetDepositParams {
                from: seller.address,
            }
        )
        .expect("should get deposit"),
        Deposit {
            amount: TokenAmountU64(10),
            rate
        }
    );

    euroe::update_operator_single(&mut chain, &buyer, euroe_contract, UpdateOperator {
        update:   OperatorUpdate::Add,
        operator: trading_contract.into(),
    });
    euroe::mint(&mut chain, &admin, euroe_contract, &euroe::MintParams {
        owner:  buyer.address.into(),
        amount: TokenAmountU64(1000),
    })
    .expect("euroe mint");
    security_p2p_trading_client::transfer_exchange(
        &mut chain,
        &buyer,
        trading_contract,
        &TransferExchangeParams {
            pay: TokenAmountU64(1000),
            get: ExchangeParams {
                from: seller.address,
                rate,
            },
        },
    )
    .expect("should transfer buy");
    assert_eq!(
        euroe::balance_of_single(&mut chain, &buyer, euroe_contract, buyer.address.into()),
        TokenAmountU64(0)
    );
    assert_eq!(
        security_p2p_trading_client::get_deposit(
            &mut chain,
            &admin,
            trading_contract,
            &GetDepositParams {
                from: seller.address,
            }
        )
        .expect("should get deposit"),
        Deposit {
            amount: TokenAmountU64(9),
            rate
        }
    );
    assert_eq!(
        security_sft_rewards_client::balance_of(
            &mut chain,
            &admin,
            token_contract,
            &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: TRACKED_TOKEN_ID,
                        address:  seller.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: TRACKED_TOKEN_ID,
                        address:  buyer.address.into(),
                    }
                ],
            }
        )
        .expect("should get balance"),
        BalanceOfQueryResponse(vec![TokenAmountU64(40), TokenAmountU64(1),])
    );
}

fn create_token_contract_sft_single(
    chain: &mut Chain,
    admin: &Account,
    compliance_contract: ContractAddress,
    ir_contract: ContractAddress,
) -> (ContractInitSuccess, ModuleReference, OwnedContractName) {
    security_sft_single_client::init(chain, admin, &security_sft_single::types::InitParam {
        compliance:        compliance_contract,
        identity_registry: ir_contract,
        metadata_url:      ContractMetadataUrl {
            hash: None,
            url:  METADATA_URL_SFT_SINGLE.to_string(),
        },
        sponsors:          None,
    })
    .expect("init sft single")
}

fn create_token_contract_sft_rewards(
    chain: &mut Chain,
    admin: &Account,
    compliance_contract: ContractAddress,
    ir_contract: ContractAddress,
) -> ContractAddress {
    security_sft_rewards_client::init(chain, admin, &security_sft_rewards::types::InitParam {
        compliance:                compliance_contract,
        identity_registry:         ir_contract,
        metadata_url:              ContractMetadataUrl {
            hash: None,
            url:  METADATA_URL_SFT_REWARDS.to_string(),
        },
        sponsors:                  None,
        blank_reward_metadata_url: ContractMetadataUrl {
            hash: None,
            url:  MIN_REWARD_METADATA_URL.to_string(),
        },
    })
    .contract_address
}

fn setup_chain(
    chain: &mut Chain,
    admin: &Account,
    compliant_nationalities: &[&str],
) -> (ContractAddress, ContractAddress, ContractAddress) {
    chain.create_account(admin.clone());

    euroe::deploy_module(chain, admin);
    identity_registry::deploy_module(chain, admin);
    compliance::deploy_module(chain, admin);
    security_sft_single_client::deploy_module(chain, admin);
    security_sft_rewards_client::deploy_module(chain, admin);
    security_p2p_trading_client::deploy_module(chain, admin);
    security_mint_fund_client::deploy_module(chain, admin);

    let euroe_contract = euroe::init(chain, admin)
        .expect("euroe init")
        .0
        .contract_address;
    euroe::grant_role(chain, admin, euroe_contract, &euroe::RoleTypes {
        adminrole: admin.address.into(),
        blockrole: admin.address.into(),
        burnrole:  admin.address.into(),
        mintrole:  admin.address.into(),
        pauserole: admin.address.into(),
    })
    .expect("grant role euroe");
    let ir_contract = identity_registry::init(chain, admin)
        .expect("identity registry init")
        .0
        .contract_address;

    let (compliance_module, ..) = init_nationalities(
        chain,
        admin,
        &concordium_rwa_compliance::compliance_modules::allowed_nationalities::init::InitParams {
            nationalities:     compliant_nationalities
                .iter()
                .map(|n| n.to_string())
                .collect(),
            identity_registry: ir_contract,
        },
    )
    .expect("init nationalities module");
    let (compliance, ..) = compliance::init(chain, admin, vec![compliance_module.contract_address])
        .expect("init compliance module");

    (euroe_contract, ir_contract, compliance.contract_address)
}
