use crate::{
    shared::db::{Collection, DbAddress, DbContractAddress, DbTokenAmount, DbTokenId, ICollection},
    txn_processor::db::IDb,
};
use async_trait::async_trait;
use bson::{doc, to_bson, Document};
use concordium_rust_sdk::types::ContractAddress;
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

#[async_trait]
pub trait IRwaSecurityNftDb: IDb {
    /// Returns the collection of agents for a contract.
    fn agents(&self, contract: &ContractAddress) -> Collection<DbAddress> {
        self.database(contract).collection::<DbAddress>("agents").into()
    }

    /// Returns the collection of contract configurations for a contract.
    fn config(&self, contract: &ContractAddress) -> Collection<ContractConfig> {
        self.database(contract).collection::<ContractConfig>("config").into()
    }

    /// Returns the collection of tokens for a contract.
    fn tokens(&self, contract: &ContractAddress) -> Collection<DbToken> {
        self.database(contract).collection::<DbToken>("tokens").into()
    }

    /// Returns the collection of token holders for a contract.
    fn holders(&self, contract: &ContractAddress) -> Collection<TokenHolder> {
        self.database(contract).collection::<TokenHolder>("holders").into()
    }

    /// Returns the collection of token holder operators for a contract.
    fn operators(&self, contract: &ContractAddress) -> Collection<TokenHolderOperator> {
        self.database(contract).collection::<TokenHolderOperator>("operators").into()
    }

    /// Returns the collection of token holder recovery records for a contract.
    fn recovery_records(
        &self,
        contract: &ContractAddress,
    ) -> Collection<TokenHolderRecoveryRecord> {
        self.database(contract).collection::<TokenHolderRecoveryRecord>("recovery_records").into()
    }

    /// Replaces the holder of a token with a new account.
    async fn replace_holder(
        &self,
        contract: &ContractAddress,
        lost_account: DbAddress,
        new_account: DbAddress,
    ) -> anyhow::Result<()> {
        let token_holders = self.holders(contract);
        token_holders
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
