use super::{event::*, state::State, utils::calculate_message_hash};
use concordium_rwa_utils::{concordium_cis3::*, sponsor_types::SponsoredParamsRaw};
use concordium_std::*;

#[derive(SchemaType, Serialize, Reject)]
enum Error {
    Parse,
    Log,
    WrongContract,
    Expired,
    NonceMismatch,
    WrongSignature,
    Serialization,
    AccountMissing,
    CallContractAmountTooLarge,
    CallContractMissingAccount,
    CallContractMissingContract,
    CallContractMissingEntrypoint,
    CallContractMessageFailed,
    CallContractTrap,
    CallContractLogicReject(i32),
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Self { Error::Parse }
}

impl From<LogError> for Error {
    fn from(_: LogError) -> Self { Error::Log }
}

impl<T> From<CallContractError<T>> for Error {
    fn from(e: CallContractError<T>) -> Self {
        match e {
            CallContractError::AmountTooLarge => Error::CallContractAmountTooLarge,
            CallContractError::MissingAccount => Error::CallContractMissingAccount,
            CallContractError::MissingContract => Error::CallContractMissingContract,
            CallContractError::MissingEntrypoint => Error::CallContractMissingEntrypoint,
            CallContractError::MessageFailed => Error::CallContractMessageFailed,
            CallContractError::LogicReject {
                reason,
                ..
            } => Error::CallContractLogicReject(reason),
            CallContractError::Trap => Error::CallContractTrap,
        }
    }
}

/// Executes a function on behalf of the signer
/// If the signature is valid, the function is executed
#[receive(
    contract = "rwa_sponsor",
    name = "permit",
    parameter = "PermitParam",
    crypto_primitives,
    mutable,
    enable_logger,
    payable,
    error = "Error"
)]
fn permit(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    amount: Amount,
    logger: &mut impl HasLogger,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> Result<(), Error> {
    let param: PermitParam = ctx.parameter_cursor().get()?;
    let message = param.message;
    ensure_ne!(message.contract_address, ctx.self_address(), Error::WrongContract);
    ensure!(message.timestamp > ctx.metadata().slot_time(), Error::Expired);

    let nonce: u64 = host.state().get_nonce(param.signer);
    ensure_eq!(message.nonce, nonce, Error::NonceMismatch);

    let signed_bytes = calculate_message_hash(&param.signer, &message, crypto_primitives)
        .map_err(|_| Error::Serialization)?;
    let valid_signature = host
        .check_account_signature(param.signer, &param.signature, &signed_bytes)
        .map_err(|_| Error::AccountMissing)?;
    ensure!(valid_signature, Error::WrongSignature);

    host.invoke_contract_read_only(
        &message.contract_address,
        &SponsoredParamsRaw {
            signer: param.signer,
            params: message.payload,
        },
        message.entry_point.as_entrypoint_name(),
        amount,
    )?;

    host.state_mut().increment_nonce(param.signer);
    logger.log(&Event::Nonce(NonceEvent {
        account: param.signer,
        nonce,
    }))?;

    Ok(())
}
