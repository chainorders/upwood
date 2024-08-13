use concordium_cis2::{IsTokenAmount, IsTokenId};
use concordium_std::{Address, ContractAddress, SchemaType, Serialize};

#[derive(Serialize, SchemaType, Copy, Clone)]
pub struct Token<T: IsTokenId> {
    pub token_id: T,
    pub contract: ContractAddress,
}

impl<T: IsTokenId> Token<T> {
    pub fn new(token_id: T, contract: ContractAddress) -> Self {
        Self {
            token_id,
            contract,
        }
    }
}

/// Parameters for the `can_transfer` function.
#[derive(Serialize, SchemaType)]
pub struct CanTransferParam<T: IsTokenId, A: IsTokenAmount> {
    /// The ID of the token to transfer.
    pub token_id: Token<T>,
    /// The address to transfer to.
    pub to:       Address,
    /// The amount of tokens to transfer.
    pub amount:   A,
}

/// Parameters for the `burned` function.
#[derive(Serialize, SchemaType)]
pub struct BurnedParam<T: IsTokenId, A: IsTokenAmount> {
    /// The ID of the token that was burned.
    pub token_id: Token<T>,
    /// The address of the owner of the burned tokens.
    pub owner:    Address,
    /// The amount of tokens that were burned.
    pub amount:   A,
}

/// Parameters for the `minted` function.
#[derive(Serialize, SchemaType)]
pub struct MintedParam<T: IsTokenId, A: IsTokenAmount> {
    /// The ID of the token that was minted.
    pub token_id: Token<T>,
    /// The address of the owner of the minted tokens.
    pub owner:    Address,
    /// The amount of tokens that were minted.
    pub amount:   A,
}

/// Parameters for the `transferred` function.
#[derive(Serialize, SchemaType)]
pub struct TransferredParam<T: IsTokenId, A: IsTokenAmount> {
    /// The ID of the token that was transferred.
    pub token_id: Token<T>,
    /// The address of the sender of the transfer.
    pub from:     Address,
    /// The address of the receiver of the transfer.
    pub to:       Address,
    /// The amount of tokens that were transferred.
    pub amount:   A,
}
