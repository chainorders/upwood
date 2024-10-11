use concordium_cis2::{IsTokenAmount, IsTokenId};
use concordium_rust_sdk::base::contracts_common::{Cursor, Deserial, Serial};
use concordium_rust_sdk::cis2;

pub fn to_cis2_token_amount<A>(amount: &A) -> cis2::TokenAmount
where A: IsTokenAmount+Serial {
    let mut bytes = vec![];
    amount
        .serial(&mut bytes)
        .expect("Failed to serialize token amount");
    let mut cursor: Cursor<_> = Cursor::new(bytes);

    cis2::TokenAmount::deserial(&mut cursor).expect("Failed to deserialize token amount")
}

pub fn to_cis2_token_id<T>(token_id: &T) -> cis2::TokenId
where T: IsTokenId+Serial {
    let mut bytes = vec![];

    token_id
        .serial(&mut bytes)
        .expect("Failed to serialize token id");
    let mut cursor: Cursor<_> = Cursor::new(bytes);

    cis2::TokenId::deserial(&mut cursor).expect("Failed to deserialize token id")
}
