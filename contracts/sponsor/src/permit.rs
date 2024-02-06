use concordium_std::*;

use concordium_rwa_utils::concordium_cis3::*;

use super::{error::Error, event::*, state::State, types::ContractResult};

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
) -> ContractResult<()> {
    let param: PermitParam = ctx.parameter_cursor().get()?;
    let message = param.message;
    ensure_eq!(message.contract_address, ctx.self_address(), Error::WrongContract);
    ensure!(message.timestamp > ctx.metadata().slot_time(), Error::Expired);

    let nonce: u64 = host.state().get_nonce(param.signer);
    ensure_eq!(message.nonce, nonce, Error::NonceMismatch);

    let message_bytes = message.bytes().map_err(|_| Error::SerializationError)?;
    let signed_bytes = {
        let message_hash: [u8; 32] = crypto_primitives.hash_sha2_256(message_bytes.as_slice()).0;
        let mut signed_bytes = [0u8; 32 + 8 + 32];
        // The message is hashed in the frontend & then the hash is signed
        // If Concordium Wallet is signing the message then it appends the
        // 1. account address (32bytes, Account Address) &
        // 2. nonce (8bytes. u64)
        // to the signing message so that a transaction cannot be signed using the same
        // process
        signed_bytes.copy_from_slice(&param.signer.0);
        signed_bytes.copy_from_slice(&[0u8; 8]);
        signed_bytes.copy_from_slice(&message_hash);
        signed_bytes
    };
    let valid_signature =
        host.check_account_signature(param.signer, &param.signature, &signed_bytes)?;
    ensure!(valid_signature, Error::WrongSignature);

    // Checks if the callee supports CIS3 by the current contract
    let supports_cis3 = supports_cis3_by(host, &message.contract_address, &ctx.self_address())?;
    ensure!(supports_cis3, Error::CIS3NotImplemented);

    host.invoke_contract_raw_read_only(
        &message.contract_address,
        Parameter::new_unchecked(message.payload.as_slice()),
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
