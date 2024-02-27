use super::error::Error;
pub type ContractResult<R> = Result<R, Error>;
pub use concordium_rwa_utils::concordium_cis3::{PermitMessage, PermitParam};
