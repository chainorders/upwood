use concordium_cis2::IsTokenId;
use concordium_std::*;

use super::{
    AddTokenParams, Agent, Burn, BurnParams, FreezeParam, FreezeParams, IsPausedResponse,
    MintParam, MintParams, PauseParam, PauseParams, RecoverParam,
};
use crate::concordium_cis2_ext::cis2_client::{Cis2Client, Cis2ClientError};
use crate::concordium_cis2_ext::{ContractMetadataUrl, IsTokenAmount};
use crate::contract_client::{invoke_contract, invoke_contract_read_only};

pub type Cis2SecurityClientError = Cis2ClientError;

pub trait Cis2SecurityClient: Cis2Client {
    fn invoke_mint<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &MintParams<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_mint_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        token_id: T,
        mint_param: MintParam<A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_freeze<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &FreezeParams<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_freeze_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        owner: Address,
        freeze_param: FreezeParam<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_un_freeze<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &FreezeParams<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_un_freeze_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        owner: Address,
        freeze_param: FreezeParam<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_burn<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &BurnParams<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_burn_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: Burn<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_add_token<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        params: &AddTokenParams<T, ContractMetadataUrl>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_pause<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        params: &PauseParams<T>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_pause_single<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        param: PauseParam<T>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_un_pause<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        params: &PauseParams<T>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_un_pause_single<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        param: PauseParam<T>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_is_paused<T: IsTokenId>(
        &self,
        contract: &ContractAddress,
        params: &PauseParams<T>,
    ) -> Result<IsPausedResponse, Cis2ClientError>;

    fn invoke_add_agent(
        &mut self,
        contract: &ContractAddress,
        params: &Agent,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_remove_agent(
        &mut self,
        contract: &ContractAddress,
        address: &Address,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_is_agent(
        &self,
        contract: &ContractAddress,
        params: &Agent,
    ) -> Result<bool, Cis2ClientError>;

    fn invoke_set_identity_registry(
        &mut self,
        contract: &ContractAddress,
        identity_registry: &ContractAddress,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_identity_registry(
        &self,
        contract: &ContractAddress,
    ) -> Result<Option<ContractAddress>, Cis2ClientError>;

    fn invoke_set_compliance(
        &mut self,
        contract: &ContractAddress,
        compliance: &ContractAddress,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_compliance(
        &self,
        contract: &ContractAddress,
    ) -> Result<Option<ContractAddress>, Cis2ClientError>;

    fn invoke_recover(
        &mut self,
        contract: &ContractAddress,
        params: &RecoverParam,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_recovery_address(
        &self,
        contract: &ContractAddress,
        address: &Address,
    ) -> Result<Option<Address>, Cis2ClientError>;
}

impl<S> Cis2SecurityClient for Host<S>
where S: Serial+DeserialWithState<ExternStateApi>
{
    fn invoke_mint<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &MintParams<T, A>,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("mint"),
            params,
        )
    }

    fn invoke_mint_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        token_id: T,
        mint_param: MintParam<A>,
    ) -> Result<(), Cis2ClientError> {
        self.invoke_mint(contract, &MintParams {
            token_id,
            owners: vec![mint_param],
        })
    }

    fn invoke_freeze<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &FreezeParams<T, A>,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("freeze"),
            params,
        )
    }

    fn invoke_freeze_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        owner: Address,
        freeze_param: FreezeParam<T, A>,
    ) -> Result<(), Cis2ClientError> {
        self.invoke_freeze(contract, &FreezeParams {
            owner,
            tokens: vec![freeze_param],
        })
    }

    fn invoke_un_freeze<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &FreezeParams<T, A>,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("un_freeze"),
            params,
        )
    }

    fn invoke_un_freeze_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        owner: Address,
        freeze_param: FreezeParam<T, A>,
    ) -> Result<(), Cis2ClientError> {
        self.invoke_un_freeze(contract, &FreezeParams {
            owner,
            tokens: vec![freeze_param],
        })
    }

    fn invoke_burn<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &BurnParams<T, A>,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("burn"),
            params,
        )
    }

    fn invoke_burn_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: Burn<T, A>,
    ) -> Result<(), Cis2ClientError> {
        self.invoke_burn(contract, &BurnParams(vec![params]))
    }

    fn invoke_add_token<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        params: &AddTokenParams<T, ContractMetadataUrl>,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("addToken"),
            params,
        )
    }

    fn invoke_pause<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        params: &PauseParams<T>,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("pause"),
            params,
        )
    }

    fn invoke_pause_single<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        param: PauseParam<T>,
    ) -> Result<(), Cis2ClientError> {
        self.invoke_pause(contract, &PauseParams {
            tokens: vec![param],
        })
    }

    fn invoke_un_pause<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        params: &PauseParams<T>,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("unPause"),
            params,
        )
    }

    fn invoke_un_pause_single<T: IsTokenId>(
        &mut self,
        contract: &ContractAddress,
        param: PauseParam<T>,
    ) -> Result<(), Cis2ClientError> {
        self.invoke_un_pause(contract, &PauseParams {
            tokens: vec![param],
        })
    }

    fn invoke_is_paused<T: IsTokenId>(
        &self,
        contract: &ContractAddress,
        params: &PauseParams<T>,
    ) -> Result<IsPausedResponse, Cis2ClientError> {
        invoke_contract_read_only(
            self,
            contract,
            EntrypointName::new_unchecked("isPaused"),
            params,
        )
    }

    fn invoke_add_agent(
        &mut self,
        contract: &ContractAddress,
        params: &Agent,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("addAgent"),
            params,
        )
    }

    fn invoke_remove_agent(
        &mut self,
        contract: &ContractAddress,
        address: &Address,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("removeAgent"),
            address,
        )
    }

    fn invoke_is_agent(
        &self,
        contract: &ContractAddress,
        params: &Agent,
    ) -> Result<bool, Cis2ClientError> {
        invoke_contract_read_only(
            self,
            contract,
            EntrypointName::new_unchecked("isAgent"),
            params,
        )
    }

    fn invoke_set_identity_registry(
        &mut self,
        contract: &ContractAddress,
        identity_registry: &ContractAddress,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("setIdentityRegistry"),
            identity_registry,
        )
    }

    fn invoke_identity_registry(
        &self,
        contract: &ContractAddress,
    ) -> Result<Option<ContractAddress>, Cis2ClientError> {
        invoke_contract_read_only(
            self,
            contract,
            EntrypointName::new_unchecked("identityRegistry"),
            &(),
        )
    }

    fn invoke_set_compliance(
        &mut self,
        contract: &ContractAddress,
        compliance: &ContractAddress,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("setCompliance"),
            compliance,
        )
    }

    fn invoke_compliance(
        &self,
        contract: &ContractAddress,
    ) -> Result<Option<ContractAddress>, Cis2ClientError> {
        invoke_contract_read_only(
            &self,
            contract,
            EntrypointName::new_unchecked("compliance"),
            &(),
        )
    }

    fn invoke_recover(
        &mut self,
        contract: &ContractAddress,
        params: &RecoverParam,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("recover"),
            params,
        )
    }

    fn invoke_recovery_address(
        &self,
        contract: &ContractAddress,
        address: &Address,
    ) -> Result<Option<Address>, Cis2ClientError> {
        invoke_contract_read_only(
            self,
            contract,
            EntrypointName::new_unchecked("recoveryAddress"),
            address,
        )
    }
}
