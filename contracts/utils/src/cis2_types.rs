use concordium_cis2::{
    TokenAmountU32 as Cis2TokenAmountU32, TokenAmountU8 as Cis2TokenAmountU8, TokenIdU32, TokenIdU8,
};
use concordium_std::{Deserial, MetadataUrl, SchemaType, Serial};

pub type TokenId = TokenIdU8;
pub type NftTokenAmount = Cis2TokenAmountU8;
pub type SftTokenId = TokenIdU32;
pub type SftTokenAmount = Cis2TokenAmountU32;
/// Represents the metadata URL and hash of a token.
#[derive(SchemaType, Serial, Clone, Deserial)]
pub struct ContractMetadataUrl {
    pub url:  String,
    pub hash: Option<String>,
}

impl From<ContractMetadataUrl> for MetadataUrl {
    fn from(val: ContractMetadataUrl) -> Self {
        MetadataUrl {
            url:  val.url,
            hash: {
                if let Some(hash) = val.hash {
                    let mut hash_bytes = [0u8; 32];
                    match hex::decode_to_slice(hash, &mut hash_bytes) {
                        Ok(_) => Some(hash_bytes),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            },
        }
    }
}
