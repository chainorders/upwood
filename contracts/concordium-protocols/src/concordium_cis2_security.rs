use concordium_cis2::{IsTokenAmount, IsTokenId};
use concordium_std::{Address, SchemaType, Serialize};

#[derive(Serialize, SchemaType)]
pub struct PauseParams<T: IsTokenId> {
    pub tokens: Vec<T>,
}

#[derive(Serialize, SchemaType, PartialEq, Debug)]
pub struct IsPausedResponse {
    pub tokens: Vec<bool>,
}

#[derive(Debug, Serialize, Clone, SchemaType)]
pub struct Burn<T: IsTokenId, A: IsTokenAmount> {
    pub token_id: T,
    pub amount:   A,
    pub owner:    Address,
}

#[derive(Debug, Serialize, Clone, SchemaType)]
#[concordium(transparent)]
pub struct BurnParams<T: IsTokenId, A: IsTokenAmount>(
    #[concordium(size_length = 2)] pub Vec<Burn<T, A>>,
);

#[derive(Serialize, SchemaType)]
pub struct FreezeParam<T: IsTokenId, A: IsTokenAmount> {
    pub token_id:     T,
    pub token_amount: A,
}

#[derive(Serialize, SchemaType)]
pub struct FreezeParams<T: IsTokenId, A: IsTokenAmount> {
    pub owner:  Address,
    pub tokens: Vec<FreezeParam<T, A>>,
}

#[derive(Serialize, SchemaType)]
pub struct FrozenParams<T: IsTokenId> {
    pub owner:  Address,
    pub tokens: Vec<T>,
}

#[derive(Serialize, SchemaType, PartialEq, Debug)]
pub struct FrozenResponse<T: IsTokenAmount> {
    pub tokens: Vec<T>,
}

#[derive(Serialize, SchemaType)]
pub struct RecoverParam {
    pub lost_account: Address,
    pub new_account:  Address,
}