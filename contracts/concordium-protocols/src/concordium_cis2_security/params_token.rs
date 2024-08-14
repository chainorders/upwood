use concordium_cis2::{IsTokenAmount, IsTokenId, Receiver};
use concordium_std::{Address, ContractAddress, SchemaType, Serialize};

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
pub struct FrozenResponse<A: IsTokenAmount> {
    pub tokens: Vec<A>,
}

#[derive(Serialize, SchemaType)]
pub struct RecoverParam {
    pub lost_account: Address,
    pub new_account:  Address,
}

pub type Agent = Address;

#[derive(Serialize, SchemaType, Clone)]
pub struct AgentWithRoles<TAgentRole> {
    pub address: Address,
    pub roles:   Vec<TAgentRole>,
}

#[derive(Serialize, SchemaType, Clone, Debug)]
pub struct TokenUId<T> {
    pub contract: ContractAddress,
    pub id:       T,
}

impl<T: Eq> PartialEq for TokenUId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.contract.eq(&other.contract) && self.id.eq(&other.id)
    }
}
impl<T: Eq> Eq for TokenUId<T> {}

impl<T: Clone> TokenUId<T> {
    pub fn to_token_owner_uid(&self, owner: Receiver) -> TokenOwnerUId<T> {
        TokenOwnerUId::new(self.clone(), owner)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct TokenOwnerUId<T> {
    pub token_id: TokenUId<T>,
    pub owner:    Receiver,
}

impl<T> TokenOwnerUId<T> {
    pub fn new(token_id: TokenUId<T>, owner: Receiver) -> Self {
        Self {
            token_id,
            owner,
        }
    }
}

impl<T: Eq> TokenOwnerUId<T> {
    pub fn matches_token(&self, token_id: &TokenUId<T>) -> bool { self.token_id.eq(token_id) }
}
