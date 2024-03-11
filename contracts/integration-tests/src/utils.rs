use concordium_cis2::{IsTokenId, TokenIdVec};
use concordium_std::{to_bytes, Serial};

pub fn to_token_id_vec<T: IsTokenId + Serial>(token_id: T) -> TokenIdVec {
    TokenIdVec(to_bytes(&token_id)[1..].to_vec())
}
