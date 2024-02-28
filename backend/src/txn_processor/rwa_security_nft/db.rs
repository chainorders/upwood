use crate::{
    shared::db::{Collection, DbAddress, DbContractAddress, DbTokenAmount, DbTokenId, ICollection},
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
pub trait IContractDb: IDb {
    fn agents(&self, contract: &ContractAddress) -> Collection<DbAddress> {
        self.database(contract).collection::<DbAddress>("agents").into()
    }

    fn config(&self, contract: &ContractAddress) -> Collection<ContractConfig> {
        self.database(contract).collection::<ContractConfig>("config").into()
    }

    fn tokens(&self, contract: &ContractAddress) -> Collection<DbToken> {
        self.database(contract).collection::<DbToken>("tokens").into()
    }

    fn holders(&self, contract: &ContractAddress) -> Collection<TokenHolder> {
        self.database(contract).collection::<TokenHolder>("holders").into()
    }

    fn operators(&self, contract: &ContractAddress) -> Collection<TokenHolderOperator> {
        self.database(contract).collection::<TokenHolderOperator>("operators").into()
    }

    fn recovery_records(
        &self,
        contract: &ContractAddress,
    ) -> Collection<TokenHolderRecoveryRecord> {
        self.database(contract).collection::<TokenHolderRecoveryRecord>("recovery_records").into()
    }

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
