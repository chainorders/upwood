use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::types::ToJSON;
use poem_openapi::OpenApi;
use shared::api::PagedResponse;
use shared::db::security_mint_fund::SecurityMintFundState;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectMedia, ForestProjectPrice, ForestProjectState,
};
use shared::db_app::forest_project_crypto::{
    ForestProjectCurrentTokenFundMarkets, ForestProjectFundInvestor, ForestProjectSupply,
    ForestProjectTokenContract, ForestProjectUserAggBalance, ForestProjectUserYieldsAggregate,
    ForestProjectUserYieldsForEachOwnedToken, SecurityTokenContractType, TokenMetadata,
};
use tracing::{debug, info};

use super::*;
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
        Path(state): Path<ForestProjectState>,
    ) -> JsonResult<PagedResponse<ForestProjectAggApiModel>> {
        let conn = &mut db_pool.get()?;
        let (projects, page_count) = ForestProject::list_by_state(conn, state, 0, i64::MAX)
            .map_err(|e| {
                error!("Failed to list projects: {}", e);
                Error::InternalServer(PlainText(format!("Failed to list projects: {}", e)))
            })?;
        let project_ids = projects.iter().map(|p| p.id).collect::<Vec<_>>();
        let project_user_balances =
            ForestProjectUserAggBalance::list_by_user_id_and_forest_project_ids(
                conn,
                &claims.sub,
                &project_ids,
            )?
            .into_iter()
            .map(|balance| (balance.forest_project_id, balance.total_balance))
            .collect::<std::collections::HashMap<_, _>>();
        let project_supplies =
            ForestProjectSupply::list_by_forest_project_ids(conn, &project_ids, 0, i64::MAX)
                .map_err(|e| {
                    error!("Failed to list project supplies: {}", e);
                    Error::InternalServer(PlainText(format!(
                        "Failed to list project supplies: {}",
                        e
                    )))
                })?
                .into_iter()
                .map(|supply| (supply.forest_project_id, supply))
                .collect::<std::collections::HashMap<_, _>>();
        let forest_project_tokens =
            ForestProjectCurrentTokenFundMarkets::list_by_forest_project_ids(
                conn,
                &project_ids,
                0,
                i64::MAX,
            )
            .map_err(|e| {
                error!("Failed to list project tokens: {}", e);
                Error::InternalServer(PlainText(format!("Failed to list project tokens: {}", e)))
            })?
            .into_iter()
            .map(|token| ((token.forest_project_id, token.token_contract_type), token))
            .collect::<std::collections::HashMap<_, _>>();

        let mut data = Vec::with_capacity(projects.len());
        for project in projects {
            let supply = project_supplies
                .get(&project.id)
                .and_then(|supply| supply.supply)
                .unwrap_or(Decimal::ZERO);
            let user_balance = project_user_balances
                .get(&project.id)
                .cloned()
                .unwrap_or(Decimal::ZERO);
            let property_market = forest_project_tokens
                .get(&(project.id, SecurityTokenContractType::Property))
                .and_then(|token| {
                    if token.market_liquidity_provider.is_some() {
                        Some(ForestProjectMarketApiModel {
                            contract_address:       token.market_contract_address.unwrap(),
                            token_id:               token.market_token_id.unwrap(),
                            token_contract_address: token.token_contract_address,
                            sell_rate_numerator:    token.market_sell_rate_numerator.unwrap(),
                            sell_rate_denominator:  token.market_sell_rate_denominator.unwrap(),
                            buy_rate_numerator:     token.market_buy_rate_numerator.unwrap(),
                            buy_rate_denominator:   token.market_buy_rate_denominator.unwrap(),
                            liquidity_provider:     token
                                .market_liquidity_provider
                                .clone()
                                .unwrap_or_default(),
                        })
                    } else {
                        None
                    }
                });
            let property_fund = forest_project_tokens
                .get(&(project.id, SecurityTokenContractType::Property))
                .and_then(|token| {
                    if token.fund_state.is_some() {
                        Some(ForestProjectFundApiModel {
                            contract_address: token.fund_contract_address.unwrap(),
                            rate_numerator: token.fund_rate_numerator.unwrap(),
                            rate_denominator: token.fund_rate_denominator.unwrap(),
                            state: token.fund_state.unwrap(),
                            investment_token_id: token.token_id.unwrap(),
                            investment_token_contract_address: token.token_contract_address,
                            token_id: token.fund_token_id.unwrap(),
                            token_contract_address: token.fund_token_contract_address.unwrap(),
                        })
                    } else {
                        None
                    }
                });
            let bond_fund = forest_project_tokens
                .get(&(project.id, SecurityTokenContractType::Bond))
                .and_then(|token| {
                    if token.fund_state.is_some() {
                        Some(ForestProjectFundApiModel {
                            contract_address: token.fund_contract_address.unwrap(),
                            rate_numerator: token.fund_rate_numerator.unwrap(),
                            rate_denominator: token.fund_rate_denominator.unwrap(),
                            state: token.fund_state.unwrap(),
                            investment_token_id: token.token_id.unwrap(),
                            investment_token_contract_address: token.token_contract_address,
                            token_id: token.fund_token_id.unwrap(),
                            token_contract_address: token.fund_token_contract_address.unwrap(),
                        })
                    } else {
                        None
                    }
                });
            data.push(ForestProjectAggApiModel {
                forest_project: project,
                supply,
                property_market,
                property_fund,
                bond_fund,
                user_balance,
            });
        }

        Ok(Json(PagedResponse {
            data,
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
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<ForestProjectAggApiModel> {
        let conn = &mut db_pool.get()?;
        let project = ForestProject::find(conn, project_id)?.ok_or(Error::NotFound(PlainText(
            format!("Forest project not found: {}", project_id),
        )))?;
        let supply = ForestProjectSupply::find_by_forest_project_id(conn, project_id)?
            .and_then(|supply| supply.supply)
            .unwrap_or(Decimal::ZERO);
        let user_balance = ForestProjectUserAggBalance::find(conn, &claims.sub, project_id)
            .map_err(|e| {
                error!("Failed to find user balance: {}", e);
                Error::InternalServer(PlainText(format!("Failed to find user balance: {}", e)))
            })?
            .map(|balance| balance.total_balance)
            .unwrap_or(Decimal::ZERO);

        let (contracts, _) = ForestProjectCurrentTokenFundMarkets::list_by_forest_project_id(
            conn,
            project_id,
            0,
            i64::MAX,
        )
        .map_err(|e| {
            error!("Failed to list project tokens: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list project tokens: {}", e)))
        })?;
        let property_market = contracts
            .iter()
            .find(|token| token.token_contract_type == SecurityTokenContractType::Property)
            .and_then(|token| {
                if token.market_liquidity_provider.is_some() {
                    Some(ForestProjectMarketApiModel {
                        contract_address:       token.market_contract_address.unwrap(),
                        token_id:               token.market_token_id.unwrap(),
                        token_contract_address: token.token_contract_address,
                        sell_rate_numerator:    token.market_sell_rate_numerator.unwrap(),
                        sell_rate_denominator:  token.market_sell_rate_denominator.unwrap(),
                        buy_rate_numerator:     token.market_buy_rate_numerator.unwrap(),
                        buy_rate_denominator:   token.market_buy_rate_denominator.unwrap(),
                        liquidity_provider:     token
                            .market_liquidity_provider
                            .clone()
                            .unwrap_or_default(),
                    })
                } else {
                    None
                }
            });
        let property_fund = contracts
            .iter()
            .find(|token| token.token_contract_type == SecurityTokenContractType::Property)
            .and_then(|token| {
                if token.fund_state.is_some() {
                    Some(ForestProjectFundApiModel {
                        contract_address: token.fund_contract_address.unwrap(),
                        rate_numerator: token.fund_rate_numerator.unwrap(),
                        rate_denominator: token.fund_rate_denominator.unwrap(),
                        state: token.fund_state.unwrap(),
                        investment_token_id: token.token_id.unwrap(),
                        investment_token_contract_address: token.token_contract_address,
                        token_id: token.fund_token_id.unwrap(),
                        token_contract_address: token.fund_token_contract_address.unwrap(),
                    })
                } else {
                    None
                }
            });
        let bond_fund = contracts
            .iter()
            .find(|token| token.token_contract_type == SecurityTokenContractType::Bond)
            .and_then(|token| {
                if token.fund_state.is_some() {
                    Some(ForestProjectFundApiModel {
                        contract_address: token.fund_contract_address.unwrap(),
                        rate_numerator: token.fund_rate_numerator.unwrap(),
                        rate_denominator: token.fund_rate_denominator.unwrap(),
                        state: token.fund_state.unwrap(),
                        investment_token_id: token.token_id.unwrap(),
                        investment_token_contract_address: token.token_contract_address,
                        token_id: token.fund_token_id.unwrap(),
                        token_contract_address: token.fund_token_contract_address.unwrap(),
                    })
                } else {
                    None
                }
            });
        Ok(Json(ForestProjectAggApiModel {
            forest_project: project,
            supply,
            property_market,
            property_fund,
            bond_fund,
            user_balance,
        }))
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
    ) -> JsonResult<PagedResponse<ForestProjectAggApiModel>> {
        let conn = &mut db_pool.get()?;
        let (user_owned_projects, _) =
            ForestProjectUserAggBalance::list_by_user_id(conn, &claims.sub, 0, i64::MAX).map_err(
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
        let projects = ForestProject::list_by_ids(conn, &project_ids)?;
        let project_user_balances = user_owned_projects
            .into_iter()
            .map(|balance| (balance.forest_project_id, balance.total_balance))
            .collect::<std::collections::HashMap<_, _>>();
        let project_supplies =
            ForestProjectSupply::list_by_forest_project_ids(conn, &project_ids, 0, i64::MAX)
                .map_err(|e| {
                    error!("Failed to list project supplies: {}", e);
                    Error::InternalServer(PlainText(format!(
                        "Failed to list project supplies: {}",
                        e
                    )))
                })?
                .into_iter()
                .map(|supply| (supply.forest_project_id, supply))
                .collect::<std::collections::HashMap<_, _>>();
        let forest_project_tokens =
            ForestProjectCurrentTokenFundMarkets::list_by_forest_project_ids(
                conn,
                &project_ids,
                0,
                i64::MAX,
            )
            .map_err(|e| {
                error!("Failed to list project tokens: {}", e);
                Error::InternalServer(PlainText(format!("Failed to list project tokens: {}", e)))
            })?
            .into_iter()
            .map(|token| ((token.forest_project_id, token.token_contract_type), token))
            .collect::<std::collections::HashMap<_, _>>();

        let mut data = Vec::with_capacity(projects.len());
        for project in projects {
            let supply = project_supplies
                .get(&project.id)
                .and_then(|supply| supply.supply)
                .unwrap_or(Decimal::ZERO);
            let user_balance = project_user_balances
                .get(&project.id)
                .cloned()
                .unwrap_or(Decimal::ZERO);
            let property_market = forest_project_tokens
                .get(&(project.id, SecurityTokenContractType::Property))
                .and_then(|token| {
                    if token.market_liquidity_provider.is_some() {
                        Some(ForestProjectMarketApiModel {
                            contract_address:       token.market_contract_address.unwrap(),
                            token_id:               token.market_token_id.unwrap(),
                            token_contract_address: token.token_contract_address,
                            sell_rate_numerator:    token.market_sell_rate_numerator.unwrap(),
                            sell_rate_denominator:  token.market_sell_rate_denominator.unwrap(),
                            buy_rate_numerator:     token.market_buy_rate_numerator.unwrap(),
                            buy_rate_denominator:   token.market_buy_rate_denominator.unwrap(),
                            liquidity_provider:     token
                                .market_liquidity_provider
                                .clone()
                                .unwrap_or_default(),
                        })
                    } else {
                        None
                    }
                });
            let property_fund = forest_project_tokens
                .get(&(project.id, SecurityTokenContractType::Property))
                .and_then(|token| {
                    if token.fund_state.is_some() {
                        Some(ForestProjectFundApiModel {
                            contract_address: token.fund_contract_address.unwrap(),
                            rate_numerator: token.fund_rate_numerator.unwrap(),
                            rate_denominator: token.fund_rate_denominator.unwrap(),
                            state: token.fund_state.unwrap(),
                            investment_token_id: token.token_id.unwrap(),
                            investment_token_contract_address: token.token_contract_address,
                            token_id: token.fund_token_id.unwrap(),
                            token_contract_address: token.fund_token_contract_address.unwrap(),
                        })
                    } else {
                        None
                    }
                });
            let bond_fund = forest_project_tokens
                .get(&(project.id, SecurityTokenContractType::Bond))
                .and_then(|token| {
                    if token.fund_state.is_some() {
                        Some(ForestProjectFundApiModel {
                            contract_address: token.fund_contract_address.unwrap(),
                            rate_numerator: token.fund_rate_numerator.unwrap(),
                            rate_denominator: token.fund_rate_denominator.unwrap(),
                            state: token.fund_state.unwrap(),
                            investment_token_id: token.token_id.unwrap(),
                            investment_token_contract_address: token.token_contract_address,
                            token_id: token.fund_token_id.unwrap(),
                            token_contract_address: token.fund_token_contract_address.unwrap(),
                        })
                    } else {
                        None
                    }
                });
            data.push(ForestProjectAggApiModel {
                forest_project: project,
                supply,
                property_market,
                property_fund,
                bond_fund,
                user_balance,
            });
        }

        Ok(Json(PagedResponse {
            data,
            page_count: 1,
            page: 0,
        }))
    }

    #[oai(
        path = "/forest_projects/:project_id/media/list/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_list_media(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectMedia>> {
        let conn = &mut db_pool.get()?;
        let (media, page_count) = ForestProjectMedia::list(conn, project_id, page, PAGE_SIZE)?;
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
    ) -> JsonResult<ForestProjectMedia> {
        let conn = &mut db_pool.get()?;
        let media = ForestProjectMedia::find(conn, media_id)?.ok_or(Error::NotFound(PlainText(
            format!(
                "Media not found for project: {} at {}",
                project_id, media_id
            ),
        )))?;
        Ok(Json(media))
    }

    #[oai(
        path = "/forest_projects/:project_id/token_contract/list",
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
        let (contracts, _) = ForestProjectTokenContract::list(conn, project_id, 0, i64::MAX)?;
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
    ) -> JsonResult<Vec<ForestProjectUserYieldsAggregate>> {
        let conn = &mut db_pool.get()?;
        let (yields, _) = ForestProjectUserYieldsAggregate::list(
            conn,
            &claims.sub,
            contracts.yielder_contract_index,
            0,
            i64::MAX,
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
    ) -> JsonResult<Vec<ForestProjectUserYieldsForEachOwnedToken>> {
        let conn = &mut db_pool.get()?;
        let (yields, _) = ForestProjectUserYieldsForEachOwnedToken::list(
            conn,
            &claims.sub,
            contracts.yielder_contract_index,
            0,
            i64::MAX,
        )
        .map_err(|e| {
            error!("Failed to list claimable yields: {}", e);
            Error::InternalServer(PlainText(format!("Failed to list claimable yields: {}", e)))
        })?;
        Ok(Json(yields))
    }
}

#[derive(Object, serde::Serialize, serde::Deserialize, Clone)]
pub struct ForestProjectMarketApiModel {
    pub contract_address:       Decimal,
    pub token_id:               Decimal,
    pub token_contract_address: Decimal,
    pub sell_rate_numerator:    Decimal,
    pub sell_rate_denominator:  Decimal,
    pub buy_rate_numerator:     Decimal,
    pub buy_rate_denominator:   Decimal,
    pub liquidity_provider:     String,
}

#[derive(Object, serde::Serialize, serde::Deserialize, Clone)]
pub struct ForestProjectFundApiModel {
    pub contract_address: Decimal,
    pub rate_numerator: Decimal,
    pub rate_denominator: Decimal,
    pub state: SecurityMintFundState,
    pub token_contract_address: Decimal,
    pub token_id: Decimal,
    pub investment_token_id: Decimal,
    pub investment_token_contract_address: Decimal,
}

#[derive(Object, serde::Serialize, serde::Deserialize, Clone)]
pub struct ForestProjectAggApiModel {
    pub forest_project:  ForestProject,
    pub supply:          Decimal,
    pub property_market: Option<ForestProjectMarketApiModel>,
    pub property_fund:   Option<ForestProjectFundApiModel>,
    pub bond_fund:       Option<ForestProjectFundApiModel>,
    pub user_balance:    Decimal,
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
    ) -> JsonResult<ForestProject> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let project = ForestProject::find(conn, project_id)?.ok_or(Error::NotFound(PlainText(
            format!("Forest project not found: {}", project_id),
        )))?;
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
    ) -> JsonResult<PagedResponse<ForestProject>> {
        ensure_is_admin(&claims)?;
        let (projects, page_count) =
            ForestProject::list_by_state_optional(&mut db_pool.get()?, state, page, PAGE_SIZE)?;
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
        if project.state != ForestProjectState::Draft {
            return Err(Error::BadRequest(PlainText(
                "Only draft projects can be created".to_string(),
            )));
        }
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
    ) -> JsonResult<ForestProjectPrice> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let price = ForestProjectPrice::latest(conn, project_id)?.ok_or(Error::NotFound(
            PlainText(format!("Price not found for project: {}", project_id)),
        ))?;
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
    ) -> JsonResult<ForestProjectPrice> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let price = ForestProjectPrice::find(conn, project_id, price_at)?.ok_or(
            Error::NotFound(PlainText(format!(
                "Price not found for project: {} at {}",
                project_id, price_at
            ))),
        )?;
        Ok(Json(price))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/price/list/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_forest_project_list_price(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectPrice>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let (prices, page_count) = ForestProjectPrice::list(conn, project_id, page, PAGE_SIZE)?;
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
        path = "/admin/forest_projects/:project_id/fund/investor/list/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_forest_project_investor_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Data(contracts): Data<&SystemContractsConfig>,
        Path(project_id): Path<uuid::Uuid>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectFundInvestor>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let (investors, page_count) = ForestProjectFundInvestor::list(
            conn,
            contracts.mint_funds_contract_index,
            project_id,
            page,
            PAGE_SIZE,
        )?;
        Ok(Json(PagedResponse {
            data: investors,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/token_contract/:contract_type",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_token_contract_find(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Path(contract_type): Path<SecurityTokenContractType>,
    ) -> JsonResult<ForestProjectTokenContract> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let contract = ForestProjectTokenContract::find(conn, project_id, contract_type)?.ok_or(
            Error::NotFound(PlainText(format!(
                "Token contract not found: {}, {}",
                project_id,
                contract_type.to_json_string()
            ))),
        )?;
        Ok(Json(contract))
    }

    #[oai(
        path = "/admin/forest_projects/token_contract",
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
        path = "/admin/forest_projects/token_contract",
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
        path = "/admin/forest_projects/:project_id/token_contract/:contract_type",
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
        path = "/admin/token_metadata",
        method = "post",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_create_token_metadata(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(metadata): Json<TokenMetadata>,
    ) -> JsonResult<TokenMetadata> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let metadata = metadata.create(conn).map_err(|e| {
            error!("Failed to create token metadata: {}", e);
            Error::InternalServer(PlainText(format!("Failed to create token metadata: {}", e)))
        })?;
        Ok(Json(metadata))
    }

    #[oai(
        path = "/admin/token_metadata/:contract_address/:token_id",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_find_token_metadata(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(contract_address): Path<Decimal>,
        Path(token_id): Path<Decimal>,
    ) -> JsonResult<TokenMetadata> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let metadata = TokenMetadata::find(conn, contract_address, token_id)?.ok_or(
            Error::NotFound(PlainText(format!(
                "Token metadata not found: {}, {}",
                contract_address, token_id
            ))),
        )?;
        Ok(Json(metadata))
    }

    #[oai(
        path = "/admin/token_metadata",
        method = "put",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_update_token_metadata(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(metadata): Json<TokenMetadata>,
    ) -> JsonResult<TokenMetadata> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let metadata = metadata.update(conn).map_err(|e| {
            error!("Failed to update token metadata: {}", e);
            Error::InternalServer(PlainText(format!("Failed to update token metadata: {}", e)))
        })?;
        Ok(Json(metadata))
    }

    #[oai(
        path = "/admin/token_metadata/list/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_list_token_metadata(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<TokenMetadata>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let (metadata, page_count) = TokenMetadata::list(conn, page, PAGE_SIZE)?;
        Ok(Json(PagedResponse {
            data: metadata,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/token_metadata/:contract_address/:token_id",
        method = "delete",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn admin_delete_token_metadata(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(contract_address): Path<Decimal>,
        Path(token_id): Path<Decimal>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        TokenMetadata::delete(conn, contract_address, token_id).map_err(|e| {
            error!("Failed to delete token metadata: {}", e);
            Error::InternalServer(PlainText(format!("Failed to delete token metadata: {}", e)))
        })?;
        Ok(())
    }
}
