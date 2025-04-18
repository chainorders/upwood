use std::collections::HashMap;

use itertools::Itertools;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::{Attachment, AttachmentType};
use poem_openapi::OpenApi;
use shared::api::PagedResponse;
use shared::db::security_mint_fund::SecurityMintFund;
use shared::db::security_p2p_trading::Market;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectMedia, ForestProjectPrice, ForestProjectState, LegalContract,
    LegalContractUserModel, LegalContractUserSignature, Notification,
};
use shared::db_app::forest_project_crypto::prelude::*;
use shared::db_shared::DbConn;
use tracing::{debug, info};
use uuid::Uuid;

use super::*;
use crate::utils::concordium::account::{verify_account_signature, AccountSignatures};
pub const MEDIA_LIMIT: i64 = 4;
pub struct ForestProjectApi;

#[OpenApi]
impl ForestProjectApi {
    #[oai(
        path = "/forest_projects/list/:state/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_list_by_state(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        Path(state): Path<ForestProjectState>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectAggApiModel>> {
        let conn = &mut db_pool.get()?;
        let (project_ids, page_count) =
            ForestProject::list_ids(conn, Some(&[state]), page, i64::MAX).map_err(|e| {
                error!("Failed to list projects ids: {}", e);
                Error::InternalServer(PlainText(format!("Failed to list projects ids: {}", e)))
            })?;
        let projects = ForestProjectAggApiModel::list(conn, contracts, &project_ids, &claims.sub)?;
        Ok(Json(PagedResponse {
            data: projects,
            page_count,
            page: 0,
        }))
    }

    #[oai(
        path = "/forest_projects/:project_id",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_get(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<Option<ForestProjectAggApiModel>> {
        let conn = &mut db_pool.get()?;
        let project =
            ForestProjectAggApiModel::list(conn, contracts, &[project_id], &claims.sub)?.pop();
        Ok(Json(project))
    }

    #[oai(
        path = "/forest_projects/list/owned",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_list_owned(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<PagedResponse<ForestProjectAggApiModel>> {
        let conn = &mut db_pool.get()?;
        let (user_owned_projects, page_count) =
            ForestProjectUserBalanceAgg::list_by_user_id(conn, &claims.sub, 0, i64::MAX).map_err(
                |e| {
                    error!("Failed to list user owned projects: {}", e);
                    Error::InternalServer(PlainText(format!(
                        "Failed to list user owned projects: {}",
                        e
                    )))
                },
            )?;
        let project_ids = user_owned_projects
            .iter()
            .map(|p| p.forest_project_id)
            .collect::<Vec<_>>();
        let projects = ForestProjectAggApiModel::list(conn, contracts, &project_ids, &claims.sub)?;
        Ok(Json(PagedResponse {
            data: projects,
            page_count,
            page: 0,
        }))
    }

    #[oai(
        path = "/forest_projects/list/owned/download",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_list_owned_download(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> Result<Attachment<Vec<u8>>> {
        let conn = &mut db_pool.get()?;
        let (user_owned_projects, _) =
            ForestProjectUserBalanceAgg::list_by_user_id(conn, &claims.sub, 0, i64::MAX).map_err(
                |e| {
                    error!("Failed to list user owned projects: {}", e);
                    Error::InternalServer(PlainText(format!(
                        "Failed to list user owned projects: {}",
                        e
                    )))
                },
            )?;
        let project_ids = user_owned_projects
            .iter()
            .map(|p| p.forest_project_id)
            .collect::<Vec<_>>();
        let projects = ForestProjectAggApiModel::list(conn, contracts, &project_ids, &claims.sub)?;
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record([
            "Project ID",
            "Project Name",
            "Property Contract Address",
            "Property Contract Type",
            "User Balance",
        ])
        .map_err(|e| {
            error!("Failed to write csv header: {}", e);
            Error::InternalServer(PlainText(format!("Failed to write csv header: {}", e)))
        })?;
        for project in projects {
            wtr.write_record(&[
                project.forest_project.id.to_string(),
                project.forest_project.name,
                project
                    .property_contract
                    .as_ref()
                    .map(|c| c.contract_address.to_string())
                    .unwrap_or_default(),
                project
                    .property_contract
                    .as_ref()
                    .map(|c| c.contract_type.to_string())
                    .unwrap_or_default(),
                project.user_balance.to_string(),
            ])
            .map_err(|e| {
                error!("Failed to write csv record: {}", e);
                Error::InternalServer(PlainText(format!("Failed to write csv record: {}", e)))
            })?;
        }
        let data = wtr.into_inner().map_err(|e| {
            error!("Failed to write csv: {}", e);
            Error::InternalServer(PlainText(format!("Failed to write csv: {}", e)))
        })?;
        Ok(Attachment::new(data)
            .attachment_type(AttachmentType::Attachment)
            .filename(
                format!(
                    "forest_projects_owned_{}.csv",
                    chrono::Utc::now().timestamp()
                )
                .to_string(),
            ))
    }

    #[oai(
        path = "/forest_projects/contract/list/owned",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_token_contracts_list_owned(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<ForestProjectTokenContractAggApiModel>> {
        let conn = &mut db_pool.get()?;
        let (owned_contracts, page_count) =
            ForestProjectTokenContractUserBalanceAgg::list_by_user_id(
                conn,
                &claims.sub,
                page.unwrap_or(0),
                page_size.unwrap_or(PAGE_SIZE),
            )
            .map_err(|e| {
                error!("Failed to list owned token contracts: {}", e);
                Error::InternalServer(PlainText(format!(
                    "Failed to list owned token contracts: {}",
                    e
                )))
            })?;
        let forest_project_ids = owned_contracts
            .iter()
            .map(|contract| contract.forest_project_id)
            .collect::<Vec<_>>();
        let prices = ForestProjectPrice::list_by_forest_project_ids(
            conn,
            &forest_project_ids,
            contracts.euro_e_token_id,
            contracts.euro_e_contract_index,
        )
        .map_err(|e| {
            error!("Failed to list project prices: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list project prices: {}", e)))
        })?
        .into_iter()
        .map(|price| ((price.project_id), price))
        .collect::<std::collections::HashMap<_, _>>();
        let yields = ForestProjectTokenContractUserYields::list_by_forest_project_ids(
            conn,
            contracts.yielder_contract_index,
            &claims.account.ok_or(Error::BadRequest(PlainText(
                "Account not found in claims".to_string(),
            )))?,
            &forest_project_ids,
        )
        .map_err(|e| {
            error!("Failed to list user yields: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list user yields: {}", e)))
        })?;
        let mut ret = Vec::with_capacity(owned_contracts.len());
        for contract in owned_contracts {
            let price = prices
                .get(&contract.forest_project_id)
                .map(|price| price.price)
                .unwrap_or(Decimal::ZERO);
            let euro_e_yield = yields.iter().find(|yield_| {
                yield_.forest_project_id == contract.forest_project_id
                    && yield_.yield_contract_address == contracts.euro_e_contract_index
                    && yield_.yield_token_id == contracts.euro_e_token_id
            });
            let carbon_credit_yield = yields.iter().find(|yield_| {
                yield_.forest_project_id == contract.forest_project_id
                    && yield_.yield_contract_address == contracts.carbon_credit_contract_index
                    && yield_.yield_token_id == contracts.carbon_credit_token_id
            });
            ret.push(ForestProjectTokenContractAggApiModel {
                forest_project_id:           contract.forest_project_id,
                forest_project_name:         contract.forest_project_name.clone(),
                token_contract_type:         contract.contract_type,
                token_contract_address:      contract.contract_address,
                user_balance:                contract.total_balance,
                user_balance_price:          contract.total_balance * price,
                carbon_credit_yield_balance: carbon_credit_yield
                    .map(|m| m.yield_amount)
                    .unwrap_or(Decimal::ZERO),
                euro_e_yields_balance:       euro_e_yield
                    .map(|m| m.yield_amount)
                    .unwrap_or(Decimal::ZERO),
            });
        }

        Ok(Json(PagedResponse {
            data: ret,
            page_count,
            page: page.unwrap_or(0),
        }))
    }

    #[oai(
        path = "/forest_projects/:project_id/media/list",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_list_media(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<ForestProjectMedia>> {
        let conn = &mut db_pool.get()?;
        let page = page.unwrap_or(0);
        let (media, page_count) =
            ForestProjectMedia::list(conn, project_id, page, page_size.unwrap_or(PAGE_SIZE))?;
        Ok(Json(PagedResponse {
            data: media,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/forest_projects/:project_id/media/:media_id",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_find_media(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(media_id): Path<uuid::Uuid>,
    ) -> JsonResult<Option<ForestProjectMedia>> {
        let conn = &mut db_pool.get()?;
        let media = ForestProjectMedia::find(conn, media_id)?;
        if let Some(ref media) = media {
            if media.project_id != project_id {
                return Ok(Json(None));
            }
        }
        Ok(Json(media))
    }

    #[oai(
        path = "/forest_projects/:project_id/contract/list",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_list_token_contracts(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<Vec<ForestProjectTokenContract>> {
        let conn = &mut db_pool.get()?;
        let (contracts, _) =
            ForestProjectTokenContract::list(conn, Some(&[project_id]), 0, i64::MAX)?;
        Ok(Json(contracts))
    }

    #[oai(
        path = "/forest_projects/yields/total",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_yields_total(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<Vec<UserYieldsAggregate>> {
        let conn = &mut db_pool.get()?;
        let yields = UserYieldsAggregate::list(
            conn,
            contracts.yielder_contract_index,
            &claims.account.ok_or(Error::BadRequest(PlainText(
                "Account not found in claims".to_string(),
            )))?,
        )
        .map_err(|e| {
            error!("Failed to list yields: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list yields: {}", e)))
        })?;
        Ok(Json(yields))
    }

    #[oai(
        path = "/forest_projects/yields/claimable",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_yields_claimable(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<Vec<YieldClaim>> {
        let conn = &mut db_pool.get()?;
        let account = claims.account.ok_or(Error::BadRequest(PlainText(
            "Account not found in claims".to_string(),
        )))?;
        let yields =
            YieldClaim::list(conn, contracts.yielder_contract_index, &account).map_err(|e| {
                error!("Failed to list claimable yields: {}", e);
                Error::InternalServer(PlainText(format!("Failed to list claimable yields: {}", e)))
            })?;
        Ok(Json(yields))
    }

    #[oai(
        path = "/forest_projects/:project_id/legal_contract/sign",
        method = "post",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_legal_contract_sign(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(concordium_client): Data<&v2::Client>,
        Path(project_id): Path<uuid::Uuid>,
        Json(signatures): Json<AccountSignatures>,
    ) -> JsonResult<LegalContractUserSignature> {
        let now = chrono::Utc::now().naive_utc();
        let user_account = claims.account().ok_or(Error::BadRequest(PlainText(
            "Account not found in claims".to_string(),
        )))?;

        let is_verified = verify_account_signature(
            concordium_client.clone(),
            user_account,
            &signatures,
            &project_id.to_string(),
            v2::BlockIdentifier::LastFinal,
        )
        .await
        .map_err(|e| {
            error!("Failed to verify account signature: {}", e);
            Error::BadRequest(PlainText(format!(
                "Failed to verify account signature: {}",
                e
            )))
        })?;

        if !is_verified {
            return Err(Error::BadRequest(PlainText(
                "Failed to verify account signature".to_string(),
            )));
        }

        let conn = &mut db_pool.get()?;
        let signature = LegalContractUserSignature {
            project_id,
            cognito_user_id: claims.sub.clone(),
            user_account: user_account.to_string(),
            user_signature: serde_json::to_string(&signatures)
                .expect("Failed to serialize signature"),
            created_at: now,
            updated_at: now,
        }
        .upsert(conn)?;
        Ok(Json(signature))
    }

    #[oai(
        path = "/forest_projects/legal_contract/list",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_legal_contract_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<LegalContractUserModel>> {
        let conn = &mut db_pool.get()?;
        let (contracts, page_count) = LegalContractUserModel::list(
            conn,
            &claims.sub,
            page.unwrap_or(0),
            page_size.unwrap_or(PAGE_SIZE),
        )
        .map_err(|e| {
            error!("Failed to list legal contracts: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list legal contracts: {}", e)))
        })?;
        Ok(Json(PagedResponse {
            data: contracts,
            page_count,
            page: page.unwrap_or(0),
        }))
    }

    #[oai(
        path = "/forest_projects/:project_id/legal_contract",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_legal_contract(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<Option<LegalContractUserModel>> {
        let conn = &mut db_pool.get()?;
        let contract = LegalContractUserModel::find(conn, project_id, &claims.sub)?;
        Ok(Json(contract))
    }
}

#[derive(Object, serde::Serialize, serde::Deserialize)]
pub struct ForestProjectAggApiModel {
    pub forest_project:    ForestProject,
    pub supply:            Decimal,
    pub user_balance:      Decimal,
    pub property_contract: Option<ForestProjectTokenContract>,
    pub property_market:   Option<Market>,
    pub property_fund:     Option<SecurityMintFund>,
    pub bond_contract:     Option<ForestProjectTokenContract>,
    pub bond_market:       Option<Market>,
    pub bond_fund:         Option<SecurityMintFund>,
    pub contract_signed:   bool,
    pub user_notified:     bool,
}

impl ForestProjectAggApiModel {
    pub fn list(
        conn: &mut DbConn,
        contracts: &SystemContractsConfig,
        project_ids: &[Uuid],
        user_id: &str,
    ) -> Result<Vec<Self>> {
        let (projects, _) = ForestProject::list(conn, Some(project_ids), None, 0, i64::MAX)
            .map_err(|e| {
                error!("Failed to list projects: {}", e);
                Error::InternalServer(PlainText(format!("Failed to list projects: {}", e)))
            })?;
        let (user_signatures, _) = LegalContractUserSignature::list_for_user(
            conn,
            Some(project_ids),
            user_id,
            0,
            i64::MAX,
        )
        .map_err(|e| {
            error!("Failed to list user signatures: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list user signatures: {}", e)))
        })?;
        let user_signed_contracts = user_signatures
            .iter()
            .map(|signature| signature.0)
            .collect::<std::collections::HashSet<_>>();
        let (user_notifications, _) =
            Notification::list_for_user(conn, Some(project_ids), user_id, 0, i64::MAX).map_err(
                |e| {
                    error!("Failed to list user notifications: {}", e);
                    Error::InternalServer(PlainText(format!(
                        "Failed to list user notifications: {}",
                        e
                    )))
                },
            )?;
        let user_notified_projects = user_notifications
            .iter()
            .map(|notification| notification.0)
            .collect::<std::collections::HashSet<_>>();
        let project_user_balances =
            ForestProjectUserBalanceAgg::list_by_user_id_and_forest_project_ids(
                conn,
                user_id,
                project_ids,
            )?
            .into_iter()
            .map(|balance| (balance.forest_project_id, balance.total_balance))
            .collect::<std::collections::HashMap<_, _>>();
        let project_supplies = ForestProjectSupply::list_by_forest_project_ids(conn, project_ids)
            .map_err(|e| {
                error!("Failed to list project supplies: {}", e);
                Error::InternalServer(PlainText(format!("Failed to list project supplies: {}", e)))
            })?
            .into_iter()
            .chunk_by(|supply| supply.forest_project_id)
            .into_iter()
            .map(|(id, chunks)| {
                (
                    id,
                    chunks.filter_map(|supply| supply.supply).sum::<Decimal>(),
                )
            })
            .collect::<std::collections::HashMap<_, _>>();
        let (project_token_contracts, _) =
            ForestProjectTokenContract::list(conn, Some(project_ids), 0, i64::MAX).map_err(
                |e| {
                    error!("Failed to list project token contracts: {}", e);
                    Error::InternalServer(PlainText(format!(
                        "Failed to list project token contracts: {}",
                        e
                    )))
                },
            )?;
        let project_contract_addresses = project_token_contracts
            .iter()
            .map(|contract| contract.contract_address)
            .collect::<Vec<_>>();
        info!("All Contracts Addresses: {:?}", project_contract_addresses);
        let (funds, _) = SecurityMintFund::list_by_investment_contracts(
            conn,
            contracts.mint_funds_contract_index,
            Some(&project_contract_addresses),
            0,
            i64::MAX,
        )
        .map_err(|e| {
            error!("Failed to list funds: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list funds: {}", e)))
        })?;
        let funds = funds
            .into_iter()
            .map(|fund| {
                (
                    (
                        fund.investment_token_contract_address,
                        fund.investment_token_id,
                    ),
                    fund,
                )
            })
            .collect::<HashMap<_, _>>();
        let (markets, _) = Market::list(
            conn,
            contracts.trading_contract_index,
            Some(project_contract_addresses),
            None,
            0,
            i64::MAX,
        )
        .map_err(|e| {
            error!("Failed to list markets: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list markets: {}", e)))
        })?;
        let markets = markets
            .into_iter()
            .map(|market| (market.token_contract_address, market))
            .collect::<HashMap<_, _>>();
        let project_token_contracts = project_token_contracts
            .into_iter()
            .map(|contract| {
                (
                    (contract.forest_project_id, contract.contract_type),
                    contract,
                )
            })
            .collect::<std::collections::HashMap<_, _>>();

        let mut data = Vec::with_capacity(projects.len());
        for project in projects {
            let supply = project_supplies
                .get(&project.id)
                .cloned()
                .unwrap_or(Decimal::ZERO);
            let user_balance = project_user_balances
                .get(&project.id)
                .cloned()
                .unwrap_or(Decimal::ZERO);
            let property_contract =
                project_token_contracts.get(&(project.id, SecurityTokenContractType::Property));
            let property_fund = match property_contract {
                Some(property_contract) => match property_contract.fund_token_id {
                    Some(fund_token_id) => funds
                        .get(&(property_contract.contract_address, fund_token_id))
                        .cloned(),
                    None => None,
                },
                None => None,
            };
            let property_market = match property_contract {
                Some(property_contract) => {
                    markets.get(&property_contract.contract_address).cloned()
                }
                None => None,
            };
            let bond_contract =
                project_token_contracts.get(&(project.id, SecurityTokenContractType::Bond));
            let bond_fund = match bond_contract {
                Some(bond_contract) => match bond_contract.fund_token_id {
                    Some(fund_token_id) => funds
                        .get(&(bond_contract.contract_address, fund_token_id))
                        .cloned(),
                    None => None,
                },
                None => None,
            };
            let bond_market = match bond_contract {
                Some(bond_contract) => markets.get(&bond_contract.contract_address).cloned(),
                None => None,
            };
            let contract_signed = user_signed_contracts.contains(&project.id);
            let user_notified = user_notified_projects.contains(&project.id);
            data.push(ForestProjectAggApiModel {
                forest_project: project,
                supply,
                property_contract: property_contract.cloned(),
                property_market,
                property_fund,
                user_balance,
                bond_contract: bond_contract.cloned(),
                bond_fund,
                bond_market,
                contract_signed,
                user_notified,
            });
        }

        Ok(data)
    }
}

#[derive(Object, serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct ForestProjectTokenContractAggApiModel {
    pub forest_project_id:           uuid::Uuid,
    pub forest_project_name:         String,
    pub token_contract_type:         SecurityTokenContractType,
    pub token_contract_address:      Decimal,
    pub user_balance:                Decimal,
    pub user_balance_price:          Decimal,
    pub carbon_credit_yield_balance: Decimal,
    pub euro_e_yields_balance:       Decimal,
}

pub struct ForestProjectAdminApi;

#[OpenApi]
impl ForestProjectAdminApi {
    /// Finds a forest project by its ID.
    /// Only admins can access this endpoint.
    ///
    /// # Arguments
    /// - `claims`: The claims of the authenticated user.
    /// - `db_pool`: The database connection pool.
    /// - `project_id`: The ID of the forest project to find.
    ///
    /// # Returns
    /// The forest project with the given ID, or an error if the project is not found.
    #[oai(
        path = "/admin/forest_projects/:project_id",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_find_forest_project(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<Option<ForestProject>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let project = ForestProject::find(conn, project_id)?;
        Ok(Json(project))
    }

    #[oai(
        path = "/admin/forest_projects/list/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_list_forest_projects(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
        Query(state): Query<Option<ForestProjectState>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<ForestProject>> {
        ensure_is_admin(&claims)?;
        let (projects, page_count) = ForestProject::list(
            &mut db_pool.get()?,
            None,
            state.as_ref().map(std::slice::from_ref),
            page,
            page_size.unwrap_or(PAGE_SIZE),
        )?;
        Ok(Json(PagedResponse {
            data: projects,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/forest_projects",
        method = "post",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_create_forest_project(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(project): Json<ForestProject>,
    ) -> JsonResult<ForestProject> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        debug!("Creating project: {:?}", project);
        let project = project.insert(conn);
        let project = match project {
            Ok(project) => project,
            Err(e) => {
                error!("Failed to create project: {}", e);
                return Err(Error::InternalServer(PlainText(format!(
                    "Failed to create project: {}",
                    e
                ))));
            }
        };
        info!("Created project: {:?} by: {}", project.id, claims.email);
        Ok(Json(project))
    }

    #[oai(
        path = "/admin/forest_projects",
        method = "put",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_update_forest_project(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(mut project): Json<ForestProject>,
    ) -> JsonResult<ForestProject> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        project.updated_at = chrono::Utc::now().naive_utc();
        let project = project.update(conn);
        let project = match project {
            Ok(project) => project,
            Err(e) => {
                error!("Failed to update project: {}", e);
                return Err(Error::InternalServer(PlainText(format!(
                    "Failed to update project: {}",
                    e
                ))));
            }
        };

        Ok(Json(project))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/media",
        method = "post",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_create_forest_project_media(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Json(media): Json<ForestProjectMedia>,
    ) -> JsonResult<ForestProjectMedia> {
        ensure_is_admin(&claims)?;
        if project_id != media.project_id {
            return Err(Error::BadRequest(PlainText(
                "Project id in path and body must be the same".to_string(),
            )));
        }
        let conn = &mut db_pool.get()?;
        let media = media.insert(conn)?;
        Ok(Json(media))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/media/:media_id",
        method = "delete",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_delete_forest_project_media(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(media_id): Path<uuid::Uuid>,
    ) -> JsonResult<ForestProjectMedia> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let media = ForestProjectMedia::find(conn, media_id)?.ok_or(Error::NotFound(PlainText(
            format!(
                "Media not found for project: {} at {}",
                project_id, media_id
            ),
        )))?;
        if media.project_id != project_id {
            return Err(Error::BadRequest(PlainText(
                "Project id in path and media must be the same".to_string(),
            )));
        }
        let media = media.delete_self(conn)?;
        Ok(Json(media))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/price/latest",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_find_forest_project_latest_price(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<Option<ForestProjectPrice>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let price = ForestProjectPrice::latest(conn, project_id)?;
        Ok(Json(price))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/price/:price_at",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_find_forest_project_price(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(price_at): Path<chrono::NaiveDateTime>,
    ) -> JsonResult<Option<ForestProjectPrice>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let price = ForestProjectPrice::find(conn, project_id, price_at)?;
        Ok(Json(price))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/price/list",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_forest_project_list_price(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Query(page): Query<Option<i64>>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<ForestProjectPrice>> {
        ensure_is_admin(&claims)?;
        let page = page.unwrap_or_default();
        let conn = &mut db_pool.get()?;
        let (prices, page_count) =
            ForestProjectPrice::list(conn, project_id, page, page_size.unwrap_or(PAGE_SIZE))?;
        Ok(Json(PagedResponse {
            data: prices,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/price",
        method = "post",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_forest_project_create_price(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Json(price): Json<ForestProjectPrice>,
    ) -> JsonResult<ForestProjectPrice> {
        ensure_is_admin(&claims)?;
        if project_id != price.project_id {
            return Err(Error::BadRequest(PlainText(
                "Project id in path and body must be the same".to_string(),
            )));
        }

        let conn = &mut db_pool.get()?;
        let price = price.insert(conn)?;
        Ok(Json(price))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/price/:price_at",
        method = "delete",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_forest_project_delete_price(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(price_at): Path<chrono::NaiveDateTime>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        ForestProjectPrice::delete(conn, project_id, price_at)?;
        Ok(())
    }

    #[oai(
        path = "/admin/forest_projects/contract/:contract_address",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_forest_project_token_contract(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(contract_address): Path<Decimal>,
    ) -> JsonResult<Option<ForestProjectTokenContract>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let contract = ForestProjectTokenContract::find(conn, contract_address)?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/contract_by_type/:contract_type",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_forest_project_token_contract_find_by_type(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(contract_type): Path<SecurityTokenContractType>,
    ) -> JsonResult<Option<ForestProjectTokenContract>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let contract = ForestProjectTokenContract::find_by_type(conn, project_id, contract_type)?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/forest_projects/contract",
        method = "post",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_token_contract_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(mut contract): Json<ForestProjectTokenContract>,
    ) -> JsonResult<ForestProjectTokenContract> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        contract.created_at = chrono::Utc::now().naive_utc();
        contract.updated_at = contract.created_at;
        let contract = contract.insert(conn).map_err(|e| {
            error!("Failed to insert token contract: {}", e);
            Error::InternalServer(PlainText(format!("Failed to insert token contract: {}", e)))
        })?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/forest_projects/contract",
        method = "put",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_token_contract_update(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(mut contract): Json<ForestProjectTokenContract>,
    ) -> JsonResult<ForestProjectTokenContract> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        contract.updated_at = chrono::Utc::now().naive_utc();
        let contract = contract.update(conn).map_err(|e| {
            error!("Failed to update token contract: {}", e);
            Error::InternalServer(PlainText(format!("Failed to update token contract: {}", e)))
        })?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/contract/:contract_type",
        method = "delete",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_token_contract_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(contract_type): Path<SecurityTokenContractType>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        ForestProjectTokenContract::delete(conn, project_id, contract_type).map_err(|e| {
            error!("Failed to delete token contract: {}", e);
            Error::InternalServer(PlainText(format!("Failed to delete token contract: {}", e)))
        })?;
        Ok(())
    }

    #[oai(
        path = "/admin/legal_contract/list",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_list_legal_contracts(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<LegalContract>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let (contracts, page_count) =
            LegalContract::list(conn, page, page_size.unwrap_or(PAGE_SIZE))?;
        Ok(Json(PagedResponse {
            data: contracts,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/legal_contract/:project_id",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_find_legal_contract(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<Option<LegalContract>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let contract = LegalContract::find(conn, project_id)?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/legal_contract",
        method = "post",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_create_legal_contract(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(mut contract): Json<LegalContract>,
    ) -> JsonResult<LegalContract> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        contract.created_at = chrono::Utc::now().naive_utc();
        contract.updated_at = contract.created_at;
        let contract = contract.insert(conn).map_err(|e| {
            error!("Failed to insert legal contract: {}", e);
            Error::InternalServer(PlainText(format!("Failed to insert legal contract: {}", e)))
        })?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/legal_contract",
        method = "put",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_update_legal_contract(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(mut contract): Json<LegalContract>,
    ) -> JsonResult<LegalContract> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        contract.updated_at = chrono::Utc::now().naive_utc();
        let contract = contract.update(conn).map_err(|e| {
            error!("Failed to update legal contract: {}", e);
            Error::InternalServer(PlainText(format!("Failed to update legal contract: {}", e)))
        })?;
        Ok(Json(contract))
    }
}
