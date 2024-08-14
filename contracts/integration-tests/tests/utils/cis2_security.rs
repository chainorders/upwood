use concordium_cis2::{IsTokenAmount, IsTokenId};
use concordium_protocols::concordium_cis2_security::{
    AgentWithRoles, BurnParams, FreezeParams, FrozenParams, FrozenResponse,
};
use concordium_smart_contract_testing::*;
use concordium_std::{Deserial, Serial};

use super::MAX_ENERGY;

pub fn add_agent<R>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &AgentWithRoles<R>,
) -> ContractInvokeSuccess
where
    R: Serial,
{
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "addAgent".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("Add Agent")
}

pub fn is_agent<R>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &AgentWithRoles<R>,
) -> bool
where
    R: Serial,
{
    chain
        .contract_invoke(
            sender.address,
            Address::Account(sender.address),
            MAX_ENERGY,
            UpdateContractPayload {
                amount:       Amount::zero(),
                address:      contract,
                receive_name: "isAgent".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("Is Agent")
        .parse_return_value()
        .unwrap()
}

pub fn agents<R>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
) -> Vec<AgentWithRoles<R>>
where
    R: Deserial,
{
    chain
        .contract_invoke(
            sender.address,
            Address::Account(sender.address),
            MAX_ENERGY,
            UpdateContractPayload {
                amount:       Amount::zero(),
                address:      contract,
                receive_name: "agents".parse().unwrap(),
                message:      OwnedParameter::from_serial(&()).unwrap(),
            },
        )
        .expect("Agents")
        .parse_return_value()
        .unwrap()
}

pub fn remove_agent(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &Address,
) -> ContractInvokeSuccess
where {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "removeAgent".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("Remove Agent")
}

pub fn set_identity_registry(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &ContractAddress,
) -> ContractInvokeSuccess
where {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "setIdentityRegistry".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("Set Identity Registry")
}

pub fn set_compliance_registry(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &ContractAddress,
) -> ContractInvokeSuccess
where {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "setCompliance".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("Set Compliance")
}

pub fn identity_registry(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
) -> ContractAddress {
    chain
        .contract_invoke(
            sender.address,
            Address::Account(sender.address),
            MAX_ENERGY,
            UpdateContractPayload {
                amount:       Amount::zero(),
                address:      contract,
                receive_name: "identityRegistry".parse().unwrap(),
                message:      OwnedParameter::empty(),
            },
        )
        .expect("Identity Registry")
        .parse_return_value()
        .unwrap()
}

pub fn compliance(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
) -> ContractAddress {
    chain
        .contract_invoke(
            sender.address,
            Address::Account(sender.address),
            MAX_ENERGY,
            UpdateContractPayload {
                amount:       Amount::zero(),
                address:      contract,
                receive_name: "compliance".parse().unwrap(),
                message:      OwnedParameter::empty(),
            },
        )
        .expect("Compliance")
        .parse_return_value()
        .unwrap()
}

pub fn burn<T, A>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &BurnParams<T, A>,
) -> ContractInvokeSuccess
where
    T: IsTokenId,
    A: IsTokenAmount,
{
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "burn".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("burn")
}

pub fn forced_burn<T, A>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &BurnParams<T, A>,
) -> ContractInvokeSuccess
where
    T: IsTokenId,
    A: IsTokenAmount,
{
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "forced_burn".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("forced_burn")
}

pub fn freeze<T, A>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &FreezeParams<T, A>,
) -> ContractInvokeSuccess
where
    T: IsTokenId,
    A: IsTokenAmount,
{
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "freeze".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("freeze")
}

pub fn un_freeze<T, A>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &FreezeParams<T, A>,
) -> ContractInvokeSuccess
where
    T: IsTokenId,
    A: IsTokenAmount,
{
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "un_freeze".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("un_freeze")
}

pub fn balance_of_frozen<T, A>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &FrozenParams<T>,
) -> FrozenResponse<A>
where
    T: IsTokenId,
    A: IsTokenAmount,
{
    chain
        .contract_invoke(
            sender.address,
            Address::Account(sender.address),
            MAX_ENERGY,
            UpdateContractPayload {
                amount:       Amount::zero(),
                address:      contract,
                receive_name: "balanceOfFrozen".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("Balance of frozen")
        .parse_return_value()
        .unwrap()
}

pub fn balance_of_un_frozen<T, A>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &FrozenParams<T>,
) -> FrozenResponse<A>
where
    T: IsTokenId,
    A: IsTokenAmount,
{
    chain
        .contract_invoke(
            sender.address,
            Address::Account(sender.address),
            MAX_ENERGY,
            UpdateContractPayload {
                amount:       Amount::zero(),
                address:      contract,
                receive_name: "balanceOfUnFrozen".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("Balance of un frozen")
        .parse_return_value()
        .unwrap()
}
