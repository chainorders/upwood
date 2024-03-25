use crate::{
    shared::db::{
        Collection, DbAccountAddress, DbAddress, DbContractAddress, DbTokenAmount, DbTokenId,
        ICollection,
    },
    txn_processor::db::IDb,
};
use async_trait::async_trait;
use bson::{doc, to_bson, Document};
use concordium_rust_sdk::types::ContractAddress;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ContractConfig {
    pub compliance:        Option<DbContractAddress>,
    pub identity_registry: Option<DbContractAddress>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbDepositedToken {
    pub token_contract:   DbContractAddress,
    pub token_id:         DbTokenId,
    pub owner:            DbAccountAddress,
    pub deposited_amount: DbTokenAmount,
    pub locked_amount:    DbTokenAmount,
    pub un_locked_amount: DbTokenAmount,
}

impl DbDepositedToken {
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
            locked_amount: DbTokenAmount::zero(),
            un_locked_amount: DbTokenAmount::zero(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbToken {
    pub token_id:          DbTokenId,
    pub is_paused:         bool,
    pub metadata_url:      Option<String>,
    pub metadata_url_hash: Option<String>,
    pub supply:            DbTokenAmount,
}

impl DbToken {
    pub fn key(token_id: &DbTokenId) -> Document {
        doc! {
            "token_id": to_bson(token_id).unwrap(),
        }
    }

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenHolder {
    pub token_id:       DbTokenId,
    pub address:        DbAddress,
    pub balance:        DbTokenAmount,
    pub frozen_balance: DbTokenAmount,
}

impl TokenHolder {
    pub fn key(token_id: &DbTokenId, address: &DbAddress) -> Document {
        doc! {
            "token_id": to_bson(token_id).unwrap(),
            "address": to_bson(address).unwrap(),
        }
    }

    pub fn default(token_id: DbTokenId, address: DbAddress) -> Self {
        Self {
            token_id,
            address,
            balance: DbTokenAmount::zero(),
            frozen_balance: DbTokenAmount::zero(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenHolderOperator {
    pub owner:    DbAddress,
    pub operator: DbAddress,
}

impl TokenHolderOperator {
    pub fn key(owner: &DbAddress, operator: &DbAddress) -> Document {
        doc! {
            "owner": to_bson(owner).unwrap(),
            "operator": to_bson(operator).unwrap(),
        }
    }

    pub fn default(owner: DbAddress, operator: DbAddress) -> Self {
        Self {
            owner,
            operator,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenHolderRecoveryRecord {
    pub lost_account: DbAddress,
    pub new_account:  DbAddress,
}

#[async_trait]
pub trait IRwaSecuritySftDb: IDb {
    /// Returns the collection of agents for a given contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// The collection of agents for the contract.
    fn agents(&self, contract: &ContractAddress) -> Collection<DbAddress> {
        self.database(contract).collection::<DbAddress>("agents").into()
    }

    /// Returns the collection of contract configurations for a given contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// The collection of contract configurations for the contract.
    fn config(&self, contract: &ContractAddress) -> Collection<ContractConfig> {
        self.database(contract).collection::<ContractConfig>("config").into()
    }

    /// Returns the collection of tokens for a given contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// The collection of tokens for the contract.
    fn tokens(&self, contract: &ContractAddress) -> Collection<DbToken> {
        self.database(contract).collection::<DbToken>("tokens").into()
    }

    /// Returns the collection of deposited tokens for a given contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// The collection of deposited tokens for the contract.
    fn deposited_tokens(&self, contract: &ContractAddress) -> Collection<DbDepositedToken> {
        self.database(contract).collection::<DbDepositedToken>("deposited_tokens").into()
    }

    /// Returns the collection of token holders for a given contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// The collection of token holders for the contract.
    fn holders(&self, contract: &ContractAddress) -> Collection<TokenHolder> {
        self.database(contract).collection::<TokenHolder>("holders").into()
    }

    /// Returns the collection of token holder operators for a given contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// The collection of token holder operators for the contract.
    fn operators(&self, contract: &ContractAddress) -> Collection<TokenHolderOperator> {
        self.database(contract).collection::<TokenHolderOperator>("operators").into()
    }

    /// Returns the collection of token holder recovery records for a given
    /// contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// The collection of token holder recovery records for the contract.
    fn recovery_records(
        &self,
        contract: &ContractAddress,
    ) -> Collection<TokenHolderRecoveryRecord> {
        self.database(contract).collection::<TokenHolderRecoveryRecord>("recovery_records").into()
    }

    /// Replaces a token holder's account address with a new account address.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    /// * `lost_account` - The address of the lost account.
    /// * `new_account` - The address of the new account.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation is successful, otherwise returns an
    /// `anyhow::Result` with an error.
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
