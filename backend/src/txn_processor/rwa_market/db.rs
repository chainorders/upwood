use bson::{doc, to_bson};
use concordium_rust_sdk::types::ContractAddress;
use serde::{Deserialize, Serialize};

use crate::{
    shared::db::{Collection, DbAccountAddress, DbContractAddress, DbTokenAmount, DbTokenId},
    txn_processor::db::IDb,
};

/// Represents a deposited token in the database.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbDepositedToken {
    pub token_contract:   DbContractAddress,
    pub token_id:         DbTokenId,
    pub owner:            DbAccountAddress,
    pub deposited_amount: DbTokenAmount,
    pub listed_amount:    DbTokenAmount,
    pub unlisted_amount:  DbTokenAmount,
}

impl DbDepositedToken {
    /// Generates a key for the deposited token based on its contract address,
    /// token ID, and owner address.
    ///
    /// # Arguments
    ///
    /// * `token_contract` - The contract address of the token.
    /// * `token_id` - The ID of the token.
    /// * `owner` - The address of the token owner.
    ///
    /// # Returns
    ///
    /// A `bson::Document` representing the key for the deposited token.
    pub fn key(
        token_contract: &DbContractAddress,
        token_id: &DbTokenId,
        owner: &DbAccountAddress,
    ) -> anyhow::Result<bson::Document> {
        let filter = doc! {
            "token_contract": to_bson(token_contract)?,
            "token_id": to_bson(token_id)?,
            "owner": to_bson(owner)?,
        };
        Ok(filter)
    }

    /// Creates a new `DbDepositedToken` instance with default values for the
    /// deposited, listed, and unlisted amounts.
    ///
    /// # Arguments
    ///
    /// * `token_contract` - The contract address of the token.
    /// * `token_id` - The ID of the token.
    /// * `owner` - The address of the token owner.
    ///
    /// # Returns
    ///
    /// A new `DbDepositedToken` instance with default values for the deposited,
    /// listed, and unlisted amounts.
    pub fn default(
        token_contract: DbContractAddress,
        token_id: DbTokenId,
        owner: DbAccountAddress,
    ) -> Self {
        Self {
            token_contract,
            token_id,
            owner,
            deposited_amount: DbTokenAmount::zero(),
            listed_amount: DbTokenAmount::zero(),
            unlisted_amount: DbTokenAmount::zero(),
        }
    }
}

/// Represents the database for the RWA market.
pub trait IRwaMarketDb: IDb {
    /// Retrieves the collection of deposited tokens for a given contract
    /// address.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// A `Collection` of `DbDepositedToken` instances for the given contract
    /// address.
    fn deposited_tokens(&self, contract: &ContractAddress) -> Collection<DbDepositedToken> {
        self.database(contract).collection::<DbDepositedToken>("deposited_tokens").into()
    }
}
