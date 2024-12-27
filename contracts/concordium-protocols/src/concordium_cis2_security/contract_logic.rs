use concordium_std::{
    ensure, Address, ContractAddress, DeserialWithState, ExternStateApi, HasHost, Host,
    MetadataUrl, Serial, Serialize, StateBuilder,
};

use super::{
    BurnedParam, CanTransferParam, SecurityParams, TokenAmountSecurity, TokenUId, TransferredParam,
};
use crate::concordium_cis2_ext::{IsTokenAmount, IsTokenId};
use crate::concordium_cis2_security::compliance_client::ComplianceClient;
use crate::concordium_cis2_security::identity_registry_client::IdentityRegistryClient;
use crate::concordium_cis2_security::MintedParam;
use crate::contract_client::ContractClientError;

pub enum Error {
    Unauthorized,
    UnVerifiedIdentity,
    InCompliantTransfer,
    ContractClientError,
}

impl From<ContractClientError<()>> for Error {
    fn from(_: ContractClientError<()>) -> Self { Error::ContractClientError }
}

#[allow(clippy::too_many_arguments)]
pub fn authorize_and_transfer<
    SE: From<Error>,
    T: IsTokenId+Copy,
    A: IsTokenAmount,
    State: Cis2SecurityState<SE, T, A>+Serial+DeserialWithState<ExternStateApi>,
>(
    host: &mut Host<State>,
    security: Option<SecurityParams>,
    sender_can_operate: bool,
    sender_is_operator_agent: bool,
    sender_is_forced_transfer_agent: bool,
    self_address: ContractAddress,
    token_id: T,
    from: Address,
    to_address: Address,
    amount: A,
) -> Result<A, SE> {
    let compliance_token = TokenUId::new(token_id, self_address);
    match (
        security,
        sender_can_operate,
        sender_is_operator_agent,
        sender_is_forced_transfer_agent,
    ) {
        // The sender is authorized to perform simple security transfer
        (Some(security), true, ..) | (Some(security), false, true, false) => {
            ensure!(
                host.invoke_identity_registry_is_verified(&security.identity_registry, &to_address)
                    .map_err(|_| Error::ContractClientError)?,
                Error::UnVerifiedIdentity.into()
            );
            ensure!(
                host.invoke_compiliance_can_transfer(&security.compliance, &CanTransferParam {
                    token_id: compliance_token,
                    to: to_address,
                    amount,
                })
                .map_err(|_| Error::ContractClientError)?,
                Error::InCompliantTransfer.into()
            );
            let (state, state_builder) = host.state_and_builder();
            let unfrozen_amount =
                state.transfer(state_builder, token_id, from, to_address, amount, false)?;
            host.invoke_compiliance_transferred(&security.compliance, &TransferredParam {
                token_id: compliance_token,
                from,
                to: to_address,
                amount,
            })
            .map_err(|_| Error::ContractClientError)?;
            Ok(unfrozen_amount)
        }
        // The sender is authorized to perform security forced transfer
        (Some(security), false, _, true) => {
            ensure!(
                host.invoke_identity_registry_is_verified(&security.identity_registry, &to_address)
                    .map_err(|_| Error::ContractClientError)?,
                Error::UnVerifiedIdentity.into()
            );
            let (state, state_builder) = host.state_and_builder();
            let unfrozen_amount =
                state.transfer(state_builder, token_id, from, to_address, amount, true)?;
            host.invoke_compiliance_transferred(&security.compliance, &TransferredParam {
                token_id: compliance_token,
                from,
                to: to_address,
                amount,
            })
            .map_err(|_| Error::ContractClientError)?;
            Ok(unfrozen_amount)
        }
        // The sender is not authorized to perform the transfer
        (_, false, false, false) => Err(Error::Unauthorized.into()),
        // The sender is authorized to perform non security simple transfer
        (None, true, ..) | (None, false, true, false) => {
            let (state, state_builder) = host.state_and_builder();
            state.transfer(state_builder, token_id, from, to_address, amount, false)
        }
        // The sender is authorized to perform non security forced transfer
        (None, false, _, true) => {
            let (state, state_builder) = host.state_and_builder();
            state.transfer(state_builder, token_id, from, to_address, amount, true)
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn authorize_and_burn<
    SE: From<Error>,
    T: IsTokenId+Copy,
    A: IsTokenAmount,
    State: Cis2SecurityState<SE, T, A>+Serial+DeserialWithState<ExternStateApi>,
>(
    host: &mut Host<State>,
    security: Option<SecurityParams>,
    sender_can_operate: bool,
    sender_is_operator_agent: bool,
    sender_is_forced_burn_agent: bool,
    self_address: ContractAddress,
    token_id: T,
    owner: Address,
    amount: A,
) -> Result<A, SE> {
    let compliance_token = TokenUId::new(token_id, self_address);
    match (
        security,
        sender_can_operate,
        sender_is_operator_agent,
        sender_is_forced_burn_agent,
    ) {
        // The sender is authorized to perform simple security burn
        (Some(security), true, ..) | (Some(security), false, true, false) => {
            let unfrozen_amount = host.state_mut().burn(token_id, amount, owner, false)?;
            host.invoke_compiliance_burned(&security.compliance, &BurnedParam {
                token_id: compliance_token,
                amount,
                owner,
            })
            .map_err(|_| Error::ContractClientError)?;
            Ok(unfrozen_amount)
        }
        // The sender is authorized to perform forced security burn
        (Some(security), false, _, true) => {
            let unfrozen_amount = host.state_mut().burn(token_id, amount, owner, true)?;
            host.invoke_compiliance_burned(&security.compliance, &BurnedParam {
                token_id: compliance_token,
                amount,
                owner,
            })
            .map_err(|_| Error::ContractClientError)?;
            Ok(unfrozen_amount)
        }
        // The sender is not authorized to perform the burn
        (_, false, false, false) => Err(Error::Unauthorized.into()),
        // The sender is authorized to perform non security simple burn
        (None, true, ..) | (None, false, true, false) => {
            host.state_mut().burn(token_id, amount, owner, false)
        }
        // The sender is authorized to perform non security forced burn
        (None, false, _, true) => host.state_mut().burn(token_id, amount, owner, true),
    }
}

pub fn authorize_and_mint<
    SE: From<Error>,
    T: IsTokenId+Copy,
    A: IsTokenAmount,
    State: Cis2SecurityState<SE, T, A>+Serial+DeserialWithState<ExternStateApi>,
>(
    host: &mut Host<State>,
    security: Option<SecurityParams>,
    token_id: T,
    owner: Address,
    amount: TokenAmountSecurity<A>,
    self_address: ContractAddress,
) -> Result<(), SE> {
    let compliance_token = TokenUId::new(token_id, self_address);
    match security {
        Some(security) => {
            ensure!(
                host.invoke_identity_registry_is_verified(&security.identity_registry, &owner)
                    .map_err(|_| Error::ContractClientError)?,
                Error::UnVerifiedIdentity.into()
            );
            ensure!(
                host.invoke_compiliance_can_transfer(&security.compliance, &CanTransferParam {
                    token_id: compliance_token,
                    amount:   amount.total(),
                    to:       owner,
                })
                .map_err(|_| Error::ContractClientError)?,
                Error::InCompliantTransfer.into()
            );
            let (state, state_builder) = host.state_and_builder();
            state.mint(state_builder, token_id, owner, amount)?;
            host.invoke_compiliance_minted(&security.compliance, &MintedParam {
                token_id: compliance_token,
                amount: amount.total(),
                owner,
            })
            .map_err(|_| Error::ContractClientError)?;
        }
        None => {
            let (state, state_builder) = host.state_and_builder();
            state.mint(state_builder, token_id, owner, amount)?;
        }
    }
    Ok(())
}

pub trait Cis2SecurityState<E, T: IsTokenId, A: IsTokenAmount> {
    fn mint(
        &mut self,
        state_builder: &mut StateBuilder,
        token_id: T,
        owner: Address,
        amount: TokenAmountSecurity<A>,
    ) -> Result<(), E>;

    fn transfer(
        &mut self,
        state_builder: &mut StateBuilder,
        token_id: T,
        from: Address,
        to: Address,
        amount: A,
        forced: bool,
    ) -> Result<A, E>;

    fn burn(&mut self, token_id: T, amount: A, owner: Address, forced: bool) -> Result<A, E>;

    fn recover(&mut self, lost_account: Address, new_account: Address) -> Result<(), E>;
}

pub enum SecurityTokenStateError {
    InsufficientFunds,
    PausedToken,
}

#[derive(Serialize, Clone)]
pub struct SecurityTokenState<A> {
    pub metadata_url: MetadataUrl,
    pub paused:       bool,
    pub supply:       A,
}

impl<A: IsTokenAmount> SecurityTokenState<A> {
    pub fn pause(&mut self) { self.paused = true; }

    pub fn un_pause(&mut self) { self.paused = false; }

    pub fn sub_assign_supply(&mut self, amount: A) -> Result<A, SecurityTokenStateError> {
        ensure!(!self.paused, SecurityTokenStateError::PausedToken);
        ensure!(
            self.supply >= amount,
            SecurityTokenStateError::InsufficientFunds
        );
        self.supply -= amount;
        Ok(self.supply)
    }

    pub fn add_assign_supply(&mut self, amount: A) -> Result<(), SecurityTokenStateError> {
        ensure!(!self.paused, SecurityTokenStateError::PausedToken);
        self.supply += amount;
        Ok(())
    }

    pub fn metadata_url(&self) -> &MetadataUrl { &self.metadata_url }
}
