use bson::{doc, to_bson};
use concordium_rust_sdk::types::ContractAddress;
use serde::{Deserialize, Serialize};

use crate::txn_processor::db::{
    Collection, DbAccountAddress, DbContractAddress, DbTokenAmount, DbTokenId, IDb,
};

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
            listed_amount: DbTokenAmount::zero(),
            unlisted_amount: DbTokenAmount::zero(),
        }
    }
}

pub trait IContractDb: IDb {
    fn deposited_tokens(&self, contract: &ContractAddress) -> Collection<DbDepositedToken> {
        self.database(contract).collection::<DbDepositedToken>("deposited_tokens").into()
    }
}
