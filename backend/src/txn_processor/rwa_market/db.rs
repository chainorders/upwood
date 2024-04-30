use crate::shared::db::{
    Collection, DbAccountAddress, DbContractAddress, DbTokenAmount, DbTokenId,
};
use bson::{doc, to_bson};
use serde::{Deserialize, Serialize};

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

pub struct RwaMarketDb {
    pub deposited_tokens: Collection<DbDepositedToken>,
}

impl RwaMarketDb {
    /// Initializes a new `RwaMarketDb` instance with the specified MongoDB
    /// database.
    ///
    /// # Arguments
    ///
    /// * `db` - The MongoDB database to use for the RWA market database.
    ///
    /// # Returns
    ///
    /// A new `RwaMarketDb` instance.
    pub fn init(db: mongodb::Database) -> Self {
        let deposited_tokens = db.collection("deposited_tokens").into();

        Self {
            deposited_tokens,
        }
    }
}
