use concordium_cis2::{
    BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, IsTokenAmount, IsTokenId,
    OperatorOfQuery, OperatorOfQueryParams, OperatorOfQueryResponse, TransferParams,
};
use concordium_smart_contract_testing::*;

use super::MAX_ENERGY;

pub fn balance_of<T, A>(
    chain: &mut Chain,
    invoker: &Account,
    contract: ContractAddress,
    payload: &BalanceOfQueryParams<T>,
) -> BalanceOfQueryResponse<A>
where
    T: IsTokenId,
    A: IsTokenAmount, {
    chain
        .contract_invoke(
            invoker.address,
            concordium_smart_contract_testing::Address::Account(invoker.address),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "balanceOf".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("balance of")
        .parse_return_value()
        .unwrap()
}

pub fn balance_of_single<T, A>(
    chain: &mut Chain,
    invoker: &Account,
    contract: ContractAddress,
    token_id: T,
    address: Address,
) -> A
where
    T: IsTokenId,
    A: IsTokenAmount + Copy, {
    let BalanceOfQueryResponse(amounts) =
        balance_of::<T, A>(chain, invoker, contract, &BalanceOfQueryParams {
            queries: vec![BalanceOfQuery {
                address,
                token_id,
            }],
        });
    amounts[0]
}

pub fn transfer<T, A>(
    chain: &mut Chain,
    sender: &Account,
    contract: &ContractAddress,
    payload: &TransferParams<T, A>,
) -> ContractInvokeSuccess
where
    T: IsTokenId,
    A: IsTokenAmount, {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      *contract,
                amount:       Amount::zero(),
                receive_name: "transfer".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("transfer")
}

pub fn transfer_single<T, A>(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: concordium_cis2::Transfer<T, A>,
) -> ContractInvokeSuccess
where
    T: IsTokenId,
    A: IsTokenAmount, {
    let payload = &TransferParams(vec![payload]);
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "transfer".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("transfer")
}

pub fn update_operator(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &concordium_cis2::UpdateOperatorParams,
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
                receive_name: "updateOperator".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("update_operator")
}

pub fn operator_of(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &concordium_cis2::OperatorOfQueryParams,
) -> ContractInvokeSuccess {
    chain
        .contract_invoke(
            sender.address,
            Address::Account(sender.address),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      contract,
                amount:       Amount::zero(),
                receive_name: "operatorOf".parse().unwrap(),
                message:      OwnedParameter::from_serial(payload).unwrap(),
            },
        )
        .expect("operator_of")
}

pub fn operator_of_single(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    owner: Address,
    address: Address,
) -> bool {
    let OperatorOfQueryResponse(operators) =
        operator_of(chain, sender, contract, &OperatorOfQueryParams {
            queries: vec![OperatorOfQuery {
                owner,
                address,
            }],
        })
        .parse_return_value()
        .unwrap();

    operators[0]
}
