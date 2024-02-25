use super::db::{DbToken, IContractDb, TokenHolder};
use crate::shared::{
    api::{ApiAddress, Error, PagedResponse, PAGE_SIZE},
    db::{DbAddress, DbTokenAmount, ICollection},
};
use bson::{doc, to_bson, Document};
use concordium_rust_sdk::types::ContractAddress;
use futures::TryStreamExt;
use poem_openapi::{param::Path, payload::Json, Object, OpenApi};

#[derive(Object)]
pub struct NftToken {
    pub token_id:          String,
    pub is_paused:         bool,
    pub metadata_url:      String,
    pub metadata_url_hash: String,
    pub supply:            String,
}

impl From<DbToken> for NftToken {
    fn from(db_token: DbToken) -> Self {
        Self {
            token_id:          db_token.token_id.0.into(),
            is_paused:         db_token.is_paused,
            metadata_url:      db_token.metadata_url.unwrap_or_default(),
            metadata_url_hash: db_token.metadata_url_hash.unwrap_or_default(),
            supply:            db_token.supply.0.to_string(),
        }
    }
}

#[derive(Object)]
pub struct NftHolder {
    pub token_id:       String,
    pub address:        ApiAddress,
    pub balance:        String,
    pub frozen_balance: String,
}

impl From<TokenHolder> for NftHolder {
    fn from(token_holder: TokenHolder) -> Self {
        Self {
            token_id:       token_holder.token_id.0.into(),
            address:        token_holder.address.into(),
            balance:        token_holder.balance.0.to_string(),
            frozen_balance: token_holder.frozen_balance.0.to_string(),
        }
    }
}

pub struct Api<TDb: IContractDb> {
    pub db: TDb,
}

#[OpenApi]
impl<TDb: IContractDb + Sync + Send + 'static> Api<TDb> {
    #[oai(path = "/rwa-security-nft/:index/:subindex/tokens/:page", method = "get")]
    pub async fn tokens(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<NftToken>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "supply": {
                "$ne": to_bson(&DbTokenAmount::zero())?,
            }
        };
        let res = self.to_paged_token_response(query, contract, page).await?;
        Ok(Json(res))
    }

    #[oai(path = "/rwa-security-nft/:index/:subindex/holders/:address/:page", method = "get")]
    pub async fn holders(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(address): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<NftHolder>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let address: DbAddress = DbAddress(address.parse()?);
        let query = doc! {
            "address": to_bson(&address)?,
            "balance": {
                "$ne": "0",
            }
        };
        let coll = self.db.holders(&contract);
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<TokenHolder> = cursor.try_collect().await?;
        let data: Vec<NftHolder> = data.into_iter().map(|holder| holder.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;
        let res = PagedResponse {
            page_count,
            page: 0,
            data,
        };

        Ok(Json(res))
    }

    #[oai(path = "/rwa-security-nft/:index/:subindex/holdersOf/:token_id/:page", method = "get")]
    pub async fn holders_of(
        &self,
        Path(index): Path<u64>,
        Path(subindex): Path<u64>,
        Path(token_id): Path<String>,
        Path(page): Path<u64>,
    ) -> Result<Json<PagedResponse<NftHolder>>, Error> {
        let contract = ContractAddress {
            index,
            subindex,
        };
        let query = doc! {
            "token_id": token_id,
        };
        let coll = self.db.holders(&contract);
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<TokenHolder> = cursor.try_collect().await?;
        let data: Vec<NftHolder> = data.into_iter().map(|holder| holder.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;
        let res = PagedResponse {
            page_count,
            page: 0,
            data,
        };

        Ok(Json(res))
    }

    pub async fn to_paged_token_response(
        &self,
        query: Document,
        contract: ContractAddress,
        page: u64,
    ) -> anyhow::Result<PagedResponse<NftToken>> {
        let coll = self.db.tokens(&contract);
        let cursor = coll.find(query.clone(), page * PAGE_SIZE, PAGE_SIZE as i64).await?;
        let data: Vec<DbToken> = cursor.try_collect().await?;
        let data: Vec<NftToken> = data.into_iter().map(|token| token.into()).collect();
        let total_count = coll.count(query).await?;
        let page_count = (total_count + PAGE_SIZE - 1) / PAGE_SIZE;

        Ok(PagedResponse {
            page_count,
            page,
            data,
        })
    }
}
