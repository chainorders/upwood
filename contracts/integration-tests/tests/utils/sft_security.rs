use concordium_cis2::TransferParams;
use concordium_smart_contract_testing::*;
use security_sft_rewards::types::*;

use super::{cis2, cis2_security, MAX_ENERGY};
pub const MODULE_PATH: &str = "../security-sft-single/contract.wasm.v1";

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    chain
        .module_deploy_v1(
            Signer::with_one_key(),
            sender.address,
            module_load_v1(MODULE_PATH).unwrap(),
        )
        .expect("deploying module")
}

pub fn init(chain: &mut Chain, sender: &Account, param: &InitParam) -> ContractInitSuccess {
    chain
        .contract_init(
            Signer::with_one_key(),
            sender.address,
            MAX_ENERGY,
            InitContractPayload {
                amount:    Amount::zero(),
                init_name: OwnedContractName::new_unchecked(
                    "init_rwa_identity_registry".to_string(),
                ),
                mod_ref:   module_load_v1(MODULE_PATH).unwrap().get_module_ref(),
                param:     OwnedParameter::from_serial(param).unwrap(),
            },
        )
        .expect("init")
}

pub fn identity_registry(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
) -> ContractAddress {
    cis2_security::identity_registry(chain, sender, contract)
}

pub fn add_agent(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &Agent,
) -> ContractInvokeSuccess {
    cis2_security::add_agent(chain, sender, contract, payload)
}

pub fn is_agent(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &Agent,
) -> bool {
    cis2_security::is_agent(chain, sender, contract, payload)
}

pub fn agents(chain: &mut Chain, sender: &Account, contract: ContractAddress) -> Vec<Agent> {
    cis2_security::agents(chain, sender, contract)
}

pub fn remove_agent(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &Address,
) -> ContractInvokeSuccess {
    cis2_security::remove_agent(chain, sender, contract, payload)
}

pub fn mint(
    chain: &mut Chain,
    sender: &Account,
    contract: &ContractAddress,
    payload: &MintParam,
) -> ContractInvokeSuccess
where {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      *contract,
                amount:       Amount::zero(),
                receive_name: "mint".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("mint")
}

pub fn transfer_single(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: concordium_cis2::Transfer<TokenId, TokenAmount>,
) -> ContractInvokeSuccess {
    cis2::transfer_single(chain, sender, contract, payload)
}

pub fn forced_transfer(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &concordium_cis2::TransferParams<TokenId, TokenAmount>,
) -> ContractInvokeSuccess {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "forcedTransfer".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("forced_transfer")
}

pub fn forced_transfer_single(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: concordium_cis2::Transfer<TokenId, TokenAmount>,
) -> ContractInvokeSuccess {
    forced_transfer(chain, sender, contract, &TransferParams(vec![payload]))
}

pub fn balance_of(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &concordium_cis2::BalanceOfQueryParams<TokenId>,
) -> concordium_cis2::BalanceOfQueryResponse<TokenAmount> {
    cis2::balance_of(chain, sender, contract, payload)
}

pub fn balance_of_single(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    token_id: TokenId,
    address: Address,
) -> TokenAmount {
    cis2::balance_of_single(chain, sender, contract, token_id, address)
}

pub fn burn(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &BurnParams,
) -> ContractInvokeSuccess {
    cis2_security::burn(chain, sender, contract, payload)
}

pub fn forced_burn(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &BurnParams,
) -> ContractInvokeSuccess {
    cis2_security::forced_burn(chain, sender, contract, payload)
}

pub fn freeze(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &FreezeParams,
) -> ContractInvokeSuccess {
    cis2_security::freeze(chain, sender, contract, payload)
}

pub fn un_freeze(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &FreezeParams,
) -> ContractInvokeSuccess {
    cis2_security::un_freeze(chain, sender, contract, payload)
}

pub fn balance_of_frozen(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &FrozenParams,
) -> FrozenResponse {
    cis2_security::balance_of_frozen(chain, sender, contract, payload)
}

pub fn balance_of_un_frozen(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &FrozenParams,
) -> FrozenResponse {
    cis2_security::balance_of_un_frozen(chain, sender, contract, payload)
}
