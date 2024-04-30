use crate::shared::db::{
    Collection, DbAddress, DbContractAddress, DbTokenAmount, DbTokenId, ICollection,
};
use bson::{doc, to_bson, Document};
use serde::{Deserialize, Serialize};

/// Represents the configuration of a contract.
#[derive(Serialize, Deserialize)]
pub struct ContractConfig {
    pub compliance:        Option<DbContractAddress>,
    pub identity_registry: Option<DbContractAddress>,
}

/// Represents a token in the database.
#[derive(Serialize, Deserialize, Debug)]
pub struct DbToken {
    pub token_id:          DbTokenId,
    pub is_paused:         bool,
    pub metadata_url:      Option<String>,
    pub metadata_url_hash: Option<String>,
    pub supply:            DbTokenAmount,
}

impl DbToken {
    /// Generates the key for a token based on its ID.
    pub fn key(token_id: &DbTokenId) -> Document {
        doc! {
            "token_id": to_bson(token_id).unwrap(),
        }
    }

    /// Creates a new `DbToken` with default values.
    pub fn default(token_id: DbTokenId) -> Self {
        Self {
            token_id,
            is_paused: false,
            metadata_url: None,
            metadata_url_hash: None,
            supply: DbTokenAmount::zero(),
        }
    }
}

/// Represents a token holder in the database.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenHolder {
    pub token_id:       DbTokenId,
    pub address:        DbAddress,
    pub balance:        DbTokenAmount,
    pub frozen_balance: DbTokenAmount,
}

impl TokenHolder {
    /// Generates the key for a token holder based on the token ID and address.
    pub fn key(token_id: &DbTokenId, address: &DbAddress) -> Document {
        doc! {
            "token_id": to_bson(token_id).unwrap(),
            "address": to_bson(address).unwrap(),
        }
    }

    /// Creates a new `TokenHolder` with default values.
    pub fn default(token_id: DbTokenId, address: DbAddress) -> Self {
        Self {
            token_id,
            address,
            balance: DbTokenAmount::zero(),
            frozen_balance: DbTokenAmount::zero(),
        }
    }
}

/// Represents an operator for a token holder in the database.
#[derive(Serialize, Deserialize)]
pub struct TokenHolderOperator {
    pub owner:    DbAddress,
    pub operator: DbAddress,
}

impl TokenHolderOperator {
    /// Generates the key for a token holder operator based on the owner and
    /// operator addresses.
    pub fn key(owner: &DbAddress, operator: &DbAddress) -> Document {
        doc! {
            "owner": to_bson(owner).unwrap(),
            "operator": to_bson(operator).unwrap(),
        }
    }

    /// Creates a new `TokenHolderOperator` with default values.
    pub fn default(owner: DbAddress, operator: DbAddress) -> Self {
        Self {
            owner,
            operator,
        }
    }
}

/// Represents a recovery record for a token holder in the database.
#[derive(Serialize, Deserialize)]
pub struct TokenHolderRecoveryRecord {
    pub lost_account: DbAddress,
    pub new_account:  DbAddress,
}

pub struct RwaSecurityNftDb {
    pub agents:           Collection<DbAddress>,
    pub config:           Collection<ContractConfig>,
    pub tokens:           Collection<DbToken>,
    pub holders:          Collection<TokenHolder>,
    pub operators:        Collection<TokenHolderOperator>,
    pub recovery_records: Collection<TokenHolderRecoveryRecord>,
}

impl RwaSecurityNftDb {
    pub fn init(db: mongodb::Database) -> Self {
        Self {
            agents:           db.collection("agents").into(),
            config:           db.collection("config").into(),
            tokens:           db.collection("tokens").into(),
            holders:          db.collection("holders").into(),
            operators:        db.collection("operators").into(),
            recovery_records: db.collection("recovery_records").into(),
        }
    }

    pub async fn replace_holder(
        &mut self,
        lost_account: DbAddress,
        new_account: DbAddress,
    ) -> anyhow::Result<()> {
        self.holders
            .update_many(
                doc! {
                    "address": to_bson(&lost_account)?,
                },
                doc! {
                    "$set": {
                        "address": to_bson(&new_account)?,
                    }
                },
            )
            .await?;

        Ok(())
    }
}
