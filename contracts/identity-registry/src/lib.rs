pub mod error;
mod state;
pub mod types;

use concordium_cis2::{
    StandardIdentifier, SupportResult, SupportsQueryParams, SupportsQueryResponse,
    CIS0_STANDARD_IDENTIFIER,
};
use concordium_protocols::concordium_cis2_security::IDENTITY_REGISTRY_STANDARD_IDENTIFIER;
use concordium_protocols::concordium_cis4::{self, cis4_client};
use concordium_std::*;
use error::*;
use state::*;
use types::*;
const SUPPORTS_STANDARDS: [StandardIdentifier<'static>; 2] = [
    CIS0_STANDARD_IDENTIFIER,
    IDENTITY_REGISTRY_STANDARD_IDENTIFIER,
];

/// Initializes the contract.
#[init(contract = "rwa_identity_registry", event = "Event", error = "Error")]
pub fn init(ctx: &InitContext, state_builder: &mut StateBuilder) -> ContractResult<State> {
    let owner = Address::Account(ctx.init_origin());

    Ok(State::new(vec![owner], state_builder))
}

/// Handles the `supports` contract call in the `rwa_identity_registry`
/// contract.
///
/// This function is called to check if the contract supports a given standard.
/// It iterates over all standards in the `SUPPORTS_STANDARDS` array, and checks
/// if the queried standard is in the array.
///
/// # Errors
///
/// Returns `Error::ParseError` if the parameters could not be parsed.
#[receive(
    contract = "rwa_identity_registry",
    name = "supports",
    parameter = "SupportsQueryParams",
    return_value = "SupportsQueryResponse",
    error = "Error"
)]
fn supports(ctx: &ReceiveContext, _: &Host<State>) -> ContractResult<SupportsQueryResponse> {
    let params: SupportsQueryParams = ctx.parameter_cursor().get()?;
    let mut response = Vec::with_capacity(params.queries.len());
    for std_id in params.queries {
        if SUPPORTS_STANDARDS.contains(&std_id.as_standard_identifier()) {
            response.push(SupportResult::Support);
        } else {
            response.push(SupportResult::NoSupport)
        }
    }

    Ok(SupportsQueryResponse::from(response))
}

/// Returns true if the given address is an agent.
///
/// # Returns
///
/// Returns `ContractResult<Vec<Address>>` containing the list of agents.
#[receive(
    contract = "rwa_identity_registry",
    name = "isAgent",
    return_value = "bool",
    parameter = "Address",
    error = "Error"
)]
pub fn is_agent(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let address: Address = ctx.parameter_cursor().get()?;
    Ok(host.state().agents.contains(&address))
}

#[receive(
    contract = "rwa_identity_registry",
    name = "agents",
    return_value = "Vec<Address>",
    error = "Error"
)]
/// Returns the list of agents.
pub fn agents(_ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<Vec<Address>> {
    Ok(host.state.agents.iter().map(|a| *a).collect())
}

/// Adds the given address as an agent.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender does not match the owner.
#[receive(
    contract = "rwa_identity_registry",
    name = "addAgent",
    mutable,
    enable_logger,
    parameter = "Address",
    error = "Error"
)]
pub fn add_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let agent: Address = ctx.parameter_cursor().get()?;
    ensure!(
        host.state_mut().agents.insert(agent),
        Error::AgentAlreadyExists
    );
    logger.log(&Event::AgentAdded(AgentUpdatedEvent { agent }))?;

    Ok(())
}

/// Removes the given address as an agent.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender does not match the owner.
#[receive(
    contract = "rwa_identity_registry",
    name = "removeAgent",
    mutable,
    enable_logger,
    parameter = "Address",
    error = "Error"
)]
pub fn remove_agent(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let agent: Address = ctx.parameter_cursor().get()?;
    ensure!(host.state_mut().agents.remove(&agent), Error::AgentNotFound);
    logger.log(&Event::AgentRemoved(AgentUpdatedEvent { agent }))?;

    Ok(())
}

/// Register Identity.
#[receive(
    contract = "rwa_identity_registry",
    name = "registerIdentity",
    mutable,
    enable_logger,
    parameter = "RegisterIdentityParams",
    error = "Error"
)]
pub fn register_identity(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    // Check if the sender is authorized to register identities.
    ensure!(
        host.state().agents.contains(&ctx.sender()),
        Error::Unauthorized
    );
    let RegisterIdentityParams { identity, address }: RegisterIdentityParams =
        ctx.parameter_cursor().get()?;
    let (state, state_builder) = host.state_and_builder();

    // Register the identity and log the event.
    let _ = state
        .identities
        .insert(address, IdentityState::new(identity, state_builder));
    logger.log(&Event::IdentityRegistered(IdentityUpdatedEvent { address }))?;

    Ok(())
}

/// Handles the `isVerified` contract call in the `rwa_identity_registry`
/// contract.
///
/// This function is called to check if an address is associated with a verified
/// identity. It retrieves the identity associated with the address from the
/// state, and checks the status of all credentials associated with the
/// identity. If all credentials are active, the identity is considered
/// verified.
///
/// # Errors
///
/// Returns `Error::IdentityNotFound` if the identity associated with the
/// address could not be found. Returns `Error::ParseError` if the parameters
/// could not be parsed.
#[receive(
    contract = "rwa_identity_registry",
    name = "isVerified",
    parameter = "Address",
    return_value = "bool",
    error = "Error"
)]
pub fn is_verified(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let address: Address = ctx.parameter_cursor().get()?;

    // Check that the identity exists.
    let identity = host.state().identities.get(&address);
    let identity = match identity {
        Some(identity) => identity,
        None => return Ok(false),
    };

    let issuers = host.state().issuers.iter().map(|i| *i);
    for issuer in issuers {
        let credential_id = identity.credential_id(&issuer);
        let credential_status = match credential_id {
            Some(credential_holder_id) => {
                cis4_client::credential_status(host, &issuer, credential_holder_id)?
            }
            None => return Ok(false),
        };

        // If the credential is not active, the identity is not verified.
        if credential_status.ne(&concordium_cis4::CredentialStatus::Active) {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Delete multiple identities.
#[receive(
    contract = "rwa_identity_registry",
    name = "deleteIdentity",
    mutable,
    enable_logger,
    parameter = "Address",
    error = "Error"
)]
pub fn delete_identity(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    // Check if the sender is authorized to delete identities.
    ensure!(
        host.state().agents.contains(&ctx.sender()),
        Error::Unauthorized
    );

    let address: Address = ctx.parameter_cursor().get()?;
    let state = host.state_mut();

    ensure!(
        state
            .identities
            .remove_and_get(&address)
            .map(|i| i.delete())
            .is_some(),
        Error::IdentityNotFound
    );

    logger.log(&Event::IdentityRemoved(IdentityUpdatedEvent { address }))?;

    Ok(())
}

/// Return true if the input address has a registered Identity.
#[receive(
    contract = "rwa_identity_registry",
    name = "hasIdentity",
    parameter = "Address",
    return_value = "bool",
    error = "Error"
)]
pub fn has_identity(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let address: Address = ctx.parameter_cursor().get()?;
    let state = host.state();
    Ok(state.identities.get(&address).is_some())
}

/// Return the identity of the input address.
#[receive(
    contract = "rwa_identity_registry",
    name = "getIdentity",
    parameter = "Address",
    return_value = "Identity",
    error = "Error"
)]
pub fn get_identity(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<Identity> {
    let address: Address = ctx.parameter_cursor().get()?;
    let state = host.state();
    state
        .identities
        .get(&address)
        .map(|i| i.to_identity())
        .ok_or(Error::IdentityNotFound)
}

/// Returns true if the given address is an issuer.
///
/// # Returns
///
/// Returns `ContractResult<Vec<ContractAddress>>` containing the list of
/// issuers.
#[receive(
    contract = "rwa_identity_registry",
    name = "isIssuer",
    parameter = "ContractAddress",
    error = "Error",
    return_value = "bool"
)]
pub fn is_issuer(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<bool> {
    let address: ContractAddress = ctx.parameter_cursor().get()?;
    Ok(host.state().issuers.contains(&address))
}

#[receive(
    contract = "rwa_identity_registry",
    name = "issuers",
    return_value = "Vec<Issuer>",
    error = "Error"
)]
/// Returns the list of issuers.
pub fn issuers(_ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<Vec<Issuer>> {
    Ok(host.state().issuers.iter().map(|a| *a).collect())
}

/// Adds the given address as an issuer.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender does not match the owner.
#[receive(
    contract = "rwa_identity_registry",
    name = "addIssuer",
    mutable,
    enable_logger,
    parameter = "Issuer",
    error = "Error"
)]
pub fn add_issuer(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let issuer: Issuer = ctx.parameter_cursor().get()?;
    ensure!(
        cis4_client::supports_cis4(host, &issuer)?,
        Error::InvalidIssuer
    );
    ensure!(
        host.state_mut().issuers.insert(issuer),
        Error::IssuerAlreadyExists
    );
    logger.log(&Event::IssuerAdded(IssuerUpdatedEvent { issuer }))?;

    Ok(())
}

/// Removes the given address as an issuer.
///
/// # Returns
///
/// Returns `ContractResult<()>` indicating whether the operation was
/// successful.
///
/// # Errors
///
/// Returns `Error::Unauthorized` if the sender does not match the owner.
#[receive(
    contract = "rwa_identity_registry",
    name = "removeIssuer",
    mutable,
    enable_logger,
    parameter = "Issuer",
    error = "Error"
)]
pub fn remove_issuer(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::Unauthorized
    );
    let issuer: ContractAddress = ctx.parameter_cursor().get()?;
    ensure!(
        host.state_mut().issuers.remove(&issuer),
        Error::IssuerNotFound
    );
    logger.log(&Event::IssuerRemoved(IssuerUpdatedEvent { issuer }))?;

    Ok(())
}
