use crate::shared::db::{
    Collection, DbAccountAddress, DbAddress, DbContractAddress, DbTokenAmount, DbTokenId,
    ICollection,
};
use bson::{doc, to_bson, Document};
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

pub struct RwaSecuritySftDb {
    pub agents:           Collection<DbAddress>,
    pub config:           Collection<ContractConfig>,
    pub tokens:           Collection<DbToken>,
    pub deposited_tokens: Collection<DbDepositedToken>,
    pub holders:          Collection<TokenHolder>,
    pub operators:        Collection<TokenHolderOperator>,
    pub recovery_records: Collection<TokenHolderRecoveryRecord>,
}

impl RwaSecuritySftDb {
    pub fn init(db: mongodb::Database) -> Self {
        Self {
            agents:           db.collection::<DbAddress>("agents").into(),
            config:           db.collection::<ContractConfig>("config").into(),
            tokens:           db.collection::<DbToken>("tokens").into(),
            deposited_tokens: db.collection::<DbDepositedToken>("deposited_tokens").into(),
            holders:          db.collection::<TokenHolder>("holders").into(),
            operators:        db.collection::<TokenHolderOperator>("operators").into(),
            recovery_records: db.collection::<TokenHolderRecoveryRecord>("recovery_records").into(),
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
