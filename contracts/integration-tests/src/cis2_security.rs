#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_cis2::{
    BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, IsTokenId, TransferParams,
    UpdateOperator, UpdateOperatorParams,
};
use concordium_protocols::concordium_cis2_ext::IsTokenAmount;
use concordium_protocols::concordium_cis2_security::{
    AddTokenParams, Agent, AgentWithRoles, BurnParams, FreezeParams, MintParams, PauseParams,
};
use concordium_smart_contract_testing::*;
use concordium_std::{Deserial, ParseError, Serial};
use nft_multi_rewarded::types::ContractMetadataUrl;

use super::MAX_ENERGY;
use crate::contract_base::{ContractInvokeErrorOrParseError, ContractPayloads, ContractTestClient};
pub trait Cis2Payloads<I: Serial, T: IsTokenId, A: IsTokenAmount>: ContractPayloads<I> {
    fn transfer_payload(&self, transfer: &TransferParams<T, A>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("transfer"),
            ),
            message:      OwnedParameter::from_serial(transfer).unwrap(),
        }
    }
    fn update_operator_payload(
        &self,
        update_operator: &UpdateOperatorParams,
    ) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("updateOperator"),
            ),
            message:      OwnedParameter::from_serial(update_operator).unwrap(),
        }
    }
    fn update_operator_single_payload(&self, operator: UpdateOperator) -> UpdateContractPayload {
        let payload = UpdateOperatorParams(vec![operator]);
        self.update_operator_payload(&payload)
    }
    fn balance_of_payload(&self, payload: &BalanceOfQueryParams<T>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("balanceOf"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
    fn balance_of_single_payload(&self, token_id: T, address: Address) -> UpdateContractPayload {
        let payload = BalanceOfQueryParams {
            queries: vec![BalanceOfQuery { token_id, address }],
        };
        self.balance_of_payload(&payload)
    }
}

pub trait Cis2Responses {
    fn parse_balance_of<A: IsTokenAmount>(&self) -> Result<BalanceOfQueryResponse<A>, ParseError>;
}
impl Cis2Responses for ContractInvokeSuccess {
    fn parse_balance_of<A: IsTokenAmount>(&self) -> Result<BalanceOfQueryResponse<A>, ParseError> {
        self.parse_return_value()
    }
}
pub trait Cis2TestClient<I: Serial, T: IsTokenId, A: IsTokenAmount>:
    Cis2Payloads<I, T, A>+ContractTestClient<I> {
    fn transfer(
        &self,
        chain: &mut Chain,
        sender: &Account,
        transfer: &TransferParams<T, A>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.transfer_payload(transfer),
        )
    }

    fn transfer_single(
        &self,
        chain: &mut Chain,
        sender: &Account,
        transfer: concordium_cis2::Transfer<T, A>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        self.transfer(chain, sender, &TransferParams(vec![transfer]))
    }

    fn update_operator(
        &self,
        chain: &mut Chain,
        sender: &Account,
        update_operator: &UpdateOperatorParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.update_operator_payload(update_operator),
        )
    }

    fn update_operator_single(
        &self,
        chain: &mut Chain,
        sender: &Account,
        operator: &UpdateOperator,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.update_operator_single_payload(operator.clone()),
        )
    }

    fn balance_of(
        &self,
        chain: &Chain,
        sender: &Account,
        payload: &BalanceOfQueryParams<T>,
    ) -> Result<BalanceOfQueryResponse<A>, ContractInvokeErrorOrParseError> {
        chain
            .contract_invoke(
                sender.address,
                sender.address.into(),
                MAX_ENERGY,
                self.balance_of_payload(payload),
            )
            .map_err(ContractInvokeErrorOrParseError::ContractInvokeError)?
            .parse_balance_of()
            .map_err(|_| ContractInvokeErrorOrParseError::ParseError)
    }

    fn balance_of_single(
        &self,
        chain: &Chain,
        sender: &Account,
        token_id: T,
        address: Address,
    ) -> Result<A, ContractInvokeErrorOrParseError> {
        chain
            .contract_invoke(
                sender.address,
                sender.address.into(),
                MAX_ENERGY,
                self.balance_of_single_payload(token_id, address),
            )
            .map_err(ContractInvokeErrorOrParseError::ContractInvokeError)?
            .parse_balance_of()
            .map_err(|_| ContractInvokeErrorOrParseError::ParseError)?
            .0
            .into_iter()
            .next()
            .ok_or({ ContractInvokeErrorOrParseError::ParseError })
    }
}
pub trait Cis2SecurityPayloads<I: Serial, R: Serial, T: IsTokenId, A: IsTokenAmount>:
    Cis2Payloads<I, T, A> {
    fn add_agent_payload(&self, agent: &AgentWithRoles<R>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("addAgent"),
            ),
            message:      OwnedParameter::from_serial(agent).unwrap(),
        }
    }
    fn agents_payload(&self) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("agents"),
            ),
            message:      OwnedParameter::empty(),
        }
    }
    fn is_agent_payload(&self, agent: &AgentWithRoles<R>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("isAgent"),
            ),
            message:      OwnedParameter::from_serial(agent).unwrap(),
        }
    }
    fn remove_agent_payload(&self, payload: &Address) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("removeAgent"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
    fn add_token_payload(
        &self,
        token: &AddTokenParams<T, ContractMetadataUrl>,
    ) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("addToken"),
            ),
            message:      OwnedParameter::from_serial(token).unwrap(),
        }
    }
    fn mint_payload(&self, mint: &MintParams<T, A>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("mint"),
            ),
            message:      OwnedParameter::from_serial(mint).unwrap(),
        }
    }
    fn burn_payload(&self, burn: &BurnParams<T, A>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("burn"),
            ),
            message:      OwnedParameter::from_serial(burn).unwrap(),
        }
    }
    fn set_identity_registry_payload(&self, payload: &ContractAddress) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("setIdentityRegistry"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
    fn set_compliance_payload(&self, payload: &ContractAddress) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("setCompliance"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
    fn freeze_payload(&self, payload: &FreezeParams<T, A>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("freeze"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
    fn un_freeze_payload(&self, payload: &FreezeParams<T, A>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("unFreeze"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
    fn pause_payload(&self, payload: &PauseParams<T>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("pause"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
    fn get_identity_registry_payload(&self) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("identityRegistry"),
            ),
            message:      OwnedParameter::empty(),
        }
    }
    fn get_compliance_payload(&self) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("compliance"),
            ),
            message:      OwnedParameter::empty(),
        }
    }
    fn balance_of_frozen_payload(
        &self,
        payload: &BalanceOfQueryParams<T>,
    ) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("balanceOfFrozen"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
    fn balance_of_un_frozen_payload(
        &self,
        payload: &BalanceOfQueryParams<T>,
    ) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("balanceOfUnFrozen"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        }
    }
}

pub trait Cis2SecurityResponses {
    fn agents<R: Deserial>(&self) -> Result<Vec<AgentWithRoles<R>>, ParseError>;
    fn is_agent(&self) -> Result<bool, ParseError>;
    fn balance_of_frozen<A: IsTokenAmount>(&self) -> Result<BalanceOfQueryResponse<A>, ParseError>;
    fn balance_of_un_frozen<A: IsTokenAmount>(
        &self,
    ) -> Result<BalanceOfQueryResponse<A>, ParseError>;
    fn balance_of<A: IsTokenAmount>(&self) -> Result<BalanceOfQueryResponse<A>, ParseError>;
    fn balance_of_single<A: IsTokenAmount>(&self) -> Result<A, ParseError>;
}
impl Cis2SecurityResponses for ContractInvokeSuccess {
    fn agents<R: Deserial>(&self) -> Result<Vec<AgentWithRoles<R>>, ParseError> {
        self.parse_return_value()
    }

    fn is_agent(&self) -> Result<bool, ParseError> { self.parse_return_value() }

    fn balance_of_frozen<A: IsTokenAmount>(&self) -> Result<BalanceOfQueryResponse<A>, ParseError> {
        self.parse_return_value()
    }

    fn balance_of_un_frozen<A: IsTokenAmount>(
        &self,
    ) -> Result<BalanceOfQueryResponse<A>, ParseError> {
        self.parse_return_value()
    }

    fn balance_of<A: IsTokenAmount>(&self) -> Result<BalanceOfQueryResponse<A>, ParseError> {
        self.parse_return_value()
    }

    fn balance_of_single<A: IsTokenAmount>(&self) -> Result<A, ParseError> {
        self.parse_return_value::<BalanceOfQueryResponse<A>>()?
            .0
            .into_iter()
            .next()
            .ok_or({ ParseError {} })
    }
}
pub trait Cis2SecurityTestClient<I: Serial, R: Serial+Deserial, T: IsTokenId, A: IsTokenAmount>:
    Cis2SecurityPayloads<I, R, T, A> {
    fn add_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        agent: &AgentWithRoles<R>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.add_agent_payload(agent),
        )
    }

    fn agents(
        &self,
        chain: &mut Chain,
        sender: &Account,
    ) -> Result<Vec<AgentWithRoles<R>>, ContractInvokeErrorOrParseError> {
        chain
            .contract_invoke(
                sender.address,
                sender.address.into(),
                MAX_ENERGY,
                self.agents_payload(),
            )
            .map_err(ContractInvokeErrorOrParseError::ContractInvokeError)?
            .agents()
            .map_err(|_| ContractInvokeErrorOrParseError::ParseError)
    }

    fn is_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        agent: &AgentWithRoles<R>,
    ) -> Result<bool, ContractInvokeErrorOrParseError> {
        chain
            .contract_invoke(
                sender.address,
                sender.address.into(),
                MAX_ENERGY,
                self.is_agent_payload(agent),
            )
            .map_err(ContractInvokeErrorOrParseError::ContractInvokeError)?
            .is_agent()
            .map_err(|_| ContractInvokeErrorOrParseError::ParseError)
    }

    fn remove_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        address: &Address,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.remove_agent_payload(address),
        )
    }

    fn add_token(
        &self,
        chain: &mut Chain,
        sender: &Account,
        token: &AddTokenParams<T, ContractMetadataUrl>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.add_token_payload(token),
        )
    }

    fn mint(
        &self,
        chain: &mut Chain,
        sender: &Account,
        mint: &MintParams<T, A>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.mint_payload(mint),
        )
    }

    fn burn(
        &self,
        chain: &mut Chain,
        sender: &Account,
        burn: &BurnParams<T, A>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.burn_payload(burn),
        )
    }

    fn set_identity_registry(
        &self,
        chain: &mut Chain,
        sender: &Account,
        address: &ContractAddress,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.set_identity_registry_payload(address),
        )
    }

    fn set_compliance(
        &self,
        chain: &mut Chain,
        sender: &Account,
        address: &ContractAddress,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.set_compliance_payload(address),
        )
    }

    fn freeze(
        &self,
        chain: &mut Chain,
        sender: &Account,
        freeze: &FreezeParams<T, A>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.freeze_payload(freeze),
        )
    }

    fn un_freeze(
        &self,
        chain: &mut Chain,
        sender: &Account,
        un_freeze: &FreezeParams<T, A>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.un_freeze_payload(un_freeze),
        )
    }

    fn pause(
        &self,
        chain: &mut Chain,
        sender: &Account,
        pause: &PauseParams<T>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.pause_payload(pause),
        )
    }

    fn get_identity_registry(
        &self,
        chain: &mut Chain,
        sender: &Account,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_invoke(
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.get_identity_registry_payload(),
        )
    }

    fn get_compliance(
        &self,
        chain: &mut Chain,
        sender: &Account,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_invoke(
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.get_compliance_payload(),
        )
    }

    fn balance_of_frozen(
        &self,
        chain: &mut Chain,
        sender: &Account,
        payload: &BalanceOfQueryParams<T>,
    ) -> Result<BalanceOfQueryResponse<A>, ContractInvokeErrorOrParseError> {
        chain
            .contract_invoke(
                sender.address,
                sender.address.into(),
                MAX_ENERGY,
                self.balance_of_frozen_payload(payload),
            )
            .map_err(ContractInvokeErrorOrParseError::ContractInvokeError)?
            .balance_of_frozen()
            .map_err(|_| ContractInvokeErrorOrParseError::ParseError)
    }

    fn balance_of_un_frozen(
        &self,
        chain: &mut Chain,
        sender: &Account,
        payload: &BalanceOfQueryParams<T>,
    ) -> Result<BalanceOfQueryResponse<A>, ContractInvokeErrorOrParseError> {
        chain
            .contract_invoke(
                sender.address,
                sender.address.into(),
                MAX_ENERGY,
                self.balance_of_un_frozen_payload(payload),
            )
            .map_err(ContractInvokeErrorOrParseError::ContractInvokeError)?
            .balance_of_un_frozen()
            .map_err(|_| ContractInvokeErrorOrParseError::ParseError)
    }
}
