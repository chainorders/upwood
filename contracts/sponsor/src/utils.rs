use concordium_rwa_utils::concordium_cis3::PermitMessage;
use concordium_std::*;

use super::{error::Error, state::State, types::ContractResult};

#[derive(Serialize, SchemaType)]
pub struct NonceParam {
    pub account: AccountAddress,
}

#[receive(
    contract = "rwa_sponsor",
    name = "nonce",
    parameter = "NonceParam",
    return_value = "u64",
    error = "super::error::Error"
)]
pub fn nonce(ctx: &ReceiveContext, host: &Host<State>) -> ContractResult<u64> {
    let param: NonceParam = ctx.parameter_cursor().get()?;
    Ok(host.state().get_nonce(param.account))
}

#[receive(
    contract = "rwa_sponsor",
    name = "bytesToSign",
    parameter = "PermitMessage",
    return_value = "[u8; 32]",
    error = "super::error::Error",
    crypto_primitives
)]
pub fn bytes_to_sign(
    ctx: &ReceiveContext,
    _: &Host<State>,
    crypto: &CryptoPrimitives,
) -> ContractResult<[u8; 32]> {
    let param: PermitMessage = ctx.parameter_cursor().get()?;
    let message_bytes = calculate_message_hash(&ctx.invoker(), &param, crypto)?;
    Ok(message_bytes)
}

pub fn calculate_message_hash(
    signer: &AccountAddress,
    message: &PermitMessage,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> Result<[u8; 32], Error> {
    let message_bytes = message.bytes().map_err(|_| Error::SerializationError)?;
    let signed_bytes = {
        let mut signed_bytes = [0u8; 32 + 8];
        // The message is hashed in the frontend & then the hash is signed
        // If Concordium Wallet is signing the message then it appends the
        // 1. account address (32bytes, Account Address) &
        // 2. nonce (8bytes. u64)
        // to the signing message so that a transaction cannot be signed using the same
        // process
        signed_bytes[0..32].copy_from_slice(&signer.0);
        signed_bytes[32..40].copy_from_slice(&[0u8; 8]);
        crypto_primitives.hash_sha2_256(&[&signed_bytes[0..40], &message_bytes].concat()).0
    };

    Ok(signed_bytes)
}
