use concordium_cis2::{IsTokenId, Receiver};
use concordium_std::{ensure, Address, ContractAddress, MetadataUrl, SchemaType, Serialize};

use crate::concordium_cis2_ext::IsTokenAmount;

#[derive(Serialize, SchemaType)]
pub struct AddTokenParams<T, M: Into<MetadataUrl>> {
    pub token_id:       T,
    pub token_metadata: M,
}

#[derive(Serialize, Clone, Copy, SchemaType)]
pub struct SecurityParams {
    pub identity_registry: ContractAddress,
    pub compliance:        ContractAddress,
}

#[derive(Serialize, Clone, Copy, SchemaType)]
pub struct TokenAmountSecurity<A: IsTokenAmount> {
    pub frozen:    A,
    pub un_frozen: A,
}

pub enum TokenAmountSecurityError {
    InsufficientFunds,
}

impl<A: IsTokenAmount> TokenAmountSecurity<A> {
    pub fn new_frozen(frozen: A) -> Self {
        Self {
            frozen,
            un_frozen: A::zero(),
        }
    }

    pub fn new_un_frozen(un_frozen: A) -> Self {
        Self {
            frozen: A::zero(),
            un_frozen,
        }
    }

    pub fn total(&self) -> A { self.frozen + self.un_frozen }

    pub fn gt(&self, other: &A) -> bool { self.total().gt(other) }

    pub fn sub_assign_unfrozen(
        &mut self,
        amount: A,
        forced: bool,
    ) -> Result<A, TokenAmountSecurityError> {
        match (self.un_frozen.ge(&amount), forced) {
            (true, _) => {
                self.un_frozen -= amount;
                Ok(A::zero())
            }
            (false, false) => Err(TokenAmountSecurityError::InsufficientFunds),
            (false, true) => {
                let to_un_freeze = amount - self.un_frozen;
                if self.frozen.lt(&to_un_freeze) {
                    return Err(TokenAmountSecurityError::InsufficientFunds);
                }
                self.un_frozen = A::zero();
                self.frozen -= to_un_freeze;
                Ok(to_un_freeze)
            }
        }
    }

    pub fn add_assign_unfrozen(&mut self, amount: A) { self.un_frozen += amount; }

    pub fn add_assign(&mut self, other: Self) {
        self.frozen += other.frozen;
        self.un_frozen += other.un_frozen;
    }

    pub fn freeze(&mut self, amount: A) -> Result<(), TokenAmountSecurityError> {
        ensure!(
            self.un_frozen.ge(&amount),
            TokenAmountSecurityError::InsufficientFunds
        );
        self.frozen += amount;
        self.un_frozen -= amount;

        Ok(())
    }

    pub fn un_freeze(&mut self, amount: A) -> Result<(), TokenAmountSecurityError> {
        ensure!(
            self.frozen.ge(&amount),
            TokenAmountSecurityError::InsufficientFunds
        );
        self.frozen -= amount;
        self.un_frozen += amount;

        Ok(())
    }
}

impl<A: IsTokenAmount> Default for TokenAmountSecurity<A> {
    fn default() -> Self {
        Self {
            frozen:    A::zero(),
            un_frozen: A::zero(),
        }
    }
}

#[derive(Serialize, SchemaType)]
pub struct MintParam<A: IsTokenAmount> {
    pub address: concordium_cis2::Receiver,
    pub amount:  TokenAmountSecurity<A>,
}

#[derive(Serialize, SchemaType)]
pub struct MintParams<T: IsTokenId, A: IsTokenAmount> {
    pub token_id: T,
    pub owners:   Vec<MintParam<A>>,
}

#[derive(Serialize, SchemaType)]
pub struct PauseParam<T: IsTokenId> {
    pub token_id: T,
}

#[derive(Serialize, SchemaType)]
pub struct PauseParams<T: IsTokenId> {
    pub tokens: Vec<PauseParam<T>>,
}

#[derive(Serialize, SchemaType, PartialEq, Debug)]
pub struct IsPausedResponse {
    pub tokens: Vec<bool>,
}

#[derive(Debug, Serialize, Clone, SchemaType)]
#[concordium(transparent)]
pub struct BurnParams<T: IsTokenId, A: IsTokenAmount>(
    #[concordium(size_length = 2)] pub Vec<Burn<T, A>>,
);

#[derive(Debug, Serialize, Clone, SchemaType)]
pub struct Burn<T: IsTokenId, A: IsTokenAmount> {
    pub token_id: T,
    pub amount:   A,
    pub owner:    Address,
}

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
pub struct RecoverParam {
    pub lost_account: Address,
    pub new_account:  Address,
}

#[derive(Serialize, SchemaType)]
pub struct Agent {
    pub address: Address,
}

#[derive(Serialize, SchemaType, Clone, Debug)]
pub struct AgentWithRoles<TAgentRole> {
    pub address: Address,
    pub roles:   Vec<TAgentRole>,
}

#[derive(Serialize, SchemaType, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenUId<T: IsTokenId> {
    pub contract: ContractAddress,
    pub id:       T,
}

impl<T: IsTokenId> TokenUId<T> {
    pub fn new(id: T, contract: ContractAddress) -> Self { Self { contract, id } }
}

impl<T: Clone+IsTokenId> TokenUId<T> {
    pub fn to_token_owner_uid(&self, owner: Receiver) -> TokenOwnerUId<T> {
        TokenOwnerUId::new(self.clone(), owner)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct TokenOwnerUId<T: IsTokenId> {
    pub token_id: TokenUId<T>,
    pub owner:    Receiver,
}

impl<T: IsTokenId> TokenOwnerUId<T> {
    pub fn new(token_id: TokenUId<T>, owner: Receiver) -> Self { Self { token_id, owner } }
}

impl<T: Eq+IsTokenId> TokenOwnerUId<T> {
    pub fn matches_token(&self, token_id: &TokenUId<T>) -> bool { self.token_id.eq(token_id) }
}

#[derive(Serialize, SchemaType)]
pub struct SetTokenMetadataParam<T, M: Into<MetadataUrl>> {
    pub token_id:       T,
    pub token_metadata: M,
}

#[derive(Serialize, SchemaType)]
pub struct SetTokenMetadataParams<T, M: Into<MetadataUrl>> {
    pub params: Vec<SetTokenMetadataParam<T, M>>,
}
