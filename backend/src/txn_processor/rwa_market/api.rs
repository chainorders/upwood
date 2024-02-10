use crate::txn_processor::{
    api::{ApiContractAddress, Error, PagedResponse, PAGE_SIZE},
    db::{DbAccountAddress, DbTokenAmount, ICollection},
};

use bson::{doc, to_bson, Document};
use concordium_rust_sdk::types::ContractAddress;
use futures::TryStreamExt;
use poem::Result;
use poem_openapi::{param::Path, payload::Json, Object, OpenApi};

use super::db::{DbDepositedToken, IContractDb};

#[derive(Object)]
pub struct MarketToken {
    pub token_contract:   ApiContractAddress,
    pub token_id:         String,
    pub owner:            String,
    pub deposited_amount: String,
    pub listed_amount:    String,
    pub unlisted_amount:  String,
}

impl From<DbDepositedToken> for MarketToken {
    fn from(db_deposited_token: DbDepositedToken) -> Self {
        Self {
            token_contract:   db_deposited_token.token_contract.into(),
            token_id:         db_deposited_token.token_id.0.into(),
            owner:            db_deposited_token.owner.0.to_string(),
            deposited_amount: db_deposited_token.deposited_amount.0.to_string(),
            listed_amount:    db_deposited_token.listed_amount.0.to_string(),
            unlisted_amount:  db_deposited_token.unlisted_amount.0.to_string(),
        }
    }
}

pub struct Api<TDb: IContractDb> {
    pub db: TDb,
}

#[OpenApi]
impl<TDb: IContractDb + Sync + Send + 'static> Api<TDb> {
    #[oai(path = "/rwa-market/:index/:subindex/listed/:page", method = "get")]
    pub async fn listed(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<MarketToken>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "listed_amount": {
                "$ne": to_bson(&DbTokenAmount::zero())?,
            }
        };
        let res = self.to_paged_response(query, contract, page).await?;
        Ok(Json(res))
    }

    #[oai(path = "/rwa-market/:index/:subindex/unlisted/:owner/:page", method = "get")]
    pub async fn unlisted(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(owner): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<MarketToken>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "owner": to_bson(&DbAccountAddress(owner.parse()?))?,
            "unlisted_amount": {
                "$ne": to_bson(&DbTokenAmount::zero())?,
            }
        };
        let res = self.to_paged_response(query, contract, page).await?;
        Ok(Json(res))
    }

    #[oai(path = "/rwa-market/:index/:subindex/deposited/:owner/:page", method = "get")]
    pub async fn deposited(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(owner): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<MarketToken>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "owner": to_bson(&DbAccountAddress(owner.parse()?))?,
            "deposited_amount": {
                "$ne": to_bson(&DbTokenAmount::zero())?,
            }
        };
        let res = self.to_paged_response(query, contract, page).await?;
        Ok(Json(res))
    }

    pub async fn to_paged_response(
        &self,
        query: Document,
        contract: ContractAddress,
        page: u64,
    ) -> anyhow::Result<PagedResponse<MarketToken>> {
        let coll = self.db.deposited_tokens(&contract);
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<DbDepositedToken> = cursor.try_collect().await?;
        let data: Vec<MarketToken> = data.into_iter().map(|token| token.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;

        Ok(PagedResponse {
            page_count,
            page,
            data,
        })
    }
}
