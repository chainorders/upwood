use concordium_cis2::TokenAmountU64;
use concordium_rwa_market::deposit::DepositParams;
use concordium_rwa_market::exchange::{Amounts, ExchangeParams};
use concordium_rwa_market::init::InitParams;
use concordium_rwa_market::list::GetListedParam;
use concordium_smart_contract_testing::*;

use super::MAX_ENERGY;
const MODULE_PATH: &str = "../market/contract.wasm.v1";

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    chain
        .module_deploy_v1(
            Signer::with_one_key(),
            sender.address,
            module_load_v1(MODULE_PATH).unwrap(),
        )
        .expect("deploying module")
}

pub fn init(chain: &mut Chain, sender: &Account, param: &InitParams) -> ContractInitSuccess {
    chain
        .contract_init(
            Signer::with_one_key(),
            sender.address,
            MAX_ENERGY,
            InitContractPayload {
                amount:    Amount::zero(),
                init_name: OwnedContractName::new_unchecked("init_rwa_market".to_string()),
                mod_ref:   module_load_v1(MODULE_PATH).unwrap().get_module_ref(),
                param:     OwnedParameter::from_serial(param).unwrap(),
            },
        )
        .expect("init")
}

pub const DEPOSIT_RECEIVE_NAME: &str = "deposit";

pub fn deposit(
    chain: &mut Chain,
    sender: &Account,
    contract: &ContractAddress,
    params: &DepositParams,
) -> ContractInvokeSuccess {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      *contract,
                amount:       Amount::zero(),
                receive_name: DEPOSIT_RECEIVE_NAME.parse().unwrap(),
                message:      OwnedParameter::from_serial(params).unwrap(),
            },
        )
        .expect("deposit")
}

pub fn balance_of_listed(
    chain: &mut Chain,
    invoker: &Account,
    contract: &ContractAddress,
    payload: &GetListedParam,
) -> TokenAmountU64 {
    chain
        .contract_invoke(
            invoker.address,
            concordium_smart_contract_testing::Address::Account(invoker.address),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      *contract,
                amount:       Amount::zero(),
                receive_name: "balanceOfDeposited".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("balance of listed")
        .parse_return_value()
        .unwrap()
}

pub fn calculate_amounts(
    chain: &mut Chain,
    invoker: &Account,
    contract: &ContractAddress,
    payload: &ExchangeParams,
) -> Amounts {
    chain
        .contract_invoke(
            invoker.address,
            concordium_smart_contract_testing::Address::Account(invoker.address),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      *contract,
                amount:       Amount::zero(),
                receive_name: "calculateAmounts".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("calculate amounts")
        .parse_return_value()
        .unwrap()
}
