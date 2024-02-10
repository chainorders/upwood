use concordium_cis2::Receiver;
use concordium_std::{ContractAddress, SchemaType, Serialize};

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
    pub fn matches(&self, token_id: &TokenUId<T>) -> bool { self.token_id.eq(token_id) }
}
