use concordium_rwa_utils::common_types;

pub type ContractResult<T> = Result<T, super::error::Error>;
pub type AttributeTag = common_types::AttributeTag;
pub type AttributeValue = common_types::AttributeValue;
pub type Identity = common_types::Identity;
pub type Issuer = common_types::Issuer;
pub use common_types::{IdentityAttribute, IdentityCredential};
pub type CredentialId = concordium_std::PublicKeyEd25519;
