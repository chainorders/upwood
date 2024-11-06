use std::collections::HashMap;

use concordium_rust_sdk::types::Address;
use db::forest_project::{ForestProject, ForestProjectPrice, ForestProjectState};
use db::DbResult;
use diesel::prelude::*;
use events_listener::txn_processor::cis2_security::db::TokenHolder;
use events_listener::txn_processor::security_mint_fund::db::{
    SecurityMintFundContract, SecurityMintFundState,
};
use events_listener::txn_processor::security_sft_rewards::db::{
    RewardHolder, RewardHolderTotal, SecuritySftRewardsContract,
};
use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::{Object, OpenApi};
use shared::api::PagedResponse;
use shared::db::DbConn;
use uuid::Uuid;

use super::*;
use crate::schema;
pub const MEDIA_LIMIT: i64 = 4;
pub struct Api;

#[OpenApi]
impl Api {
    /// Lists the active forest projects, paginated by the provided page number. Active projects are those in the funding state.
    ///
    /// # Arguments
    /// - `_claims`: The claims of the authenticated user.
    /// - `db_pool`: The database connection pool.
    /// - `page`: The page number to retrieve.
    ///
    /// # Returns
    /// A `PagedResponse` containing the active forest projects and the total number of pages.
    #[oai(
        path = "/forest_projects/list/active/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn list_active(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectApi>> {
        let (projects, page_count) = ForestProjectApi::list_by_status(
            &mut db_pool.get()?,
            ForestProjectState::Funding,
            page,
            PAGE_SIZE,
        )?;
        Ok(Json(PagedResponse {
            data: projects,
            page_count,
            page,
        }))
    }

    /// Lists the funded forest projects, paginated by the provided page number. Funded projects are those in the funded state.
    ///
    /// # Arguments
    /// - `claims`: The claims of the authenticated user.
    /// - `db_pool`: The database connection pool.
    /// - `page`: The page number to retrieve.
    ///
    /// # Returns
    /// A `PagedResponse` containing the funded forest projects and the total number of pages.
    #[oai(
        path = "/forest_projects/list/funded/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn list_funded(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectWithNotification>> {
        let (projects, page_count) = ForestProjectWithNotification::list_by_status(
            &mut db_pool.get()?,
            ForestProjectState::Funded,
            claims.sub.as_str(),
            page,
            PAGE_SIZE,
        )?;
        Ok(Json(PagedResponse {
            data: projects,
            page_count,
            page,
        }))
    }

    /// Lists the forest projects owned by the authenticated user, paginated by the provided page number.
    /// The projects are filtered by the authenticated user.
    ///
    /// # Arguments
    /// - `claims`: The claims of the authenticated user.
    /// - `db_pool`: The database connection pool.
    ///
    /// # Returns
    /// A `PagedResponse` containing the forest projects owned by the authenticated user and the total number of pages.
    #[oai(
        path = "/forest_projects/list/owned",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn list_owned(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<PagedResponse<ForestProjectUser>> {
        let user_account: Address = ensure_account_registered(&claims)?.into();
        let (projects, _) = ForestProject::list_by_status(
            &mut db_pool.get()?,
            ForestProjectState::Funded,
            0,
            i64::MAX,
        )?;
        let project_tokens = projects
            .iter()
            .map(|project| (project.contract_address, ForestProject::tracked_token_id()))
            .collect::<Vec<_>>();
        let mut ret: Vec<_> = vec![];
        let conn = &mut db_pool.get()?;
        let token_holders = TokenHolder::list_by_tokens(conn, project_tokens, &user_account)?;
        let mut token_holders = token_holders
            .iter()
            .map(|h| (h.cis2_address, h))
            .collect::<HashMap<_, _>>();
        for project in projects {
            let holder = token_holders.remove(&project.contract_address);
            if let Some(holder) = holder {
                ret.push(ForestProjectUser {
                    project,
                    token_holder: holder.clone(),
                });
            }
        }

        Ok(Json(PagedResponse {
            data:       ret,
            page_count: 1,
            page:       0,
        }))
    }

    /// Finds a forest project by its id.
    ///
    /// # Arguments
    /// - `claims`: The claims of the authenticated user.
    /// - `db_pool`: The database connection pool.
    /// - `project_id`: The id of the forest project to find.
    ///
    /// # Returns
    /// The forest project with the provided id.
    #[oai(
        path = "/forest_projects/:project_id",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn find(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<ForestProjectDetails> {
        let project = ForestProjectDetails::find(&mut db_pool.get()?, project_id, &claims.sub)?
            .ok_or(Error::NotFound(PlainText(format!(
                "Forest project not found: {}",
                project_id
            ))))?;
        Ok(Json(project))
    }

    /// Lists the rewards of the authenticated user for the funded forest projects, paginated by the provided page number.
    ///
    /// # Arguments
    /// - `claims`: The claims of the authenticated user.
    /// - `db_pool`: The database connection pool.
    /// - `page`: The page number to retrieve.
    ///
    /// # Returns
    /// A `PagedResponse` containing the rewards of the authenticated user for the funded forest projects and the total number of pages.
    #[oai(
        path = "/forest_projects/rewards/total",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn rewards_total(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<Vec<RewardHolderTotal>> {
        let user_account: Address = ensure_account_registered(&claims)?.into();
        let conn = &mut db_pool.get()?;
        let (projects, _) =
            ForestProject::list_by_status(conn, ForestProjectState::Funded, 0, i64::MAX)?;
        let project_contracts = projects
            .iter()
            .map(|project| (project.contract_address))
            .collect::<Vec<_>>();
        let res = RewardHolderTotal::find(conn, project_contracts, &user_account.to_string())?;
        Ok(Json(res))
    }

    /// Lists the rewards of the authenticated user for the funded forest projects, paginated by the provided page number.
    ///
    /// # Arguments
    /// - `claims`: The claims of the authenticated user.
    /// - `db_pool`: The database connection pool.
    /// - `page`: The page number to retrieve.
    ///
    /// # Returns
    /// A `PagedResponse` containing the rewards of the authenticated user for the funded forest projects and the total number of pages.
    #[oai(
        path = "/forest_projects/rewards/list/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn rewards_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<RewardHolder>> {
        let user_account: Address = ensure_account_registered(&claims)?.into();
        let conn = &mut db_pool.get()?;
        let (projects, _) =
            ForestProject::list_by_status(conn, ForestProjectState::Funded, 0, i64::MAX)?;
        let project_contracts = projects
            .iter()
            .map(|project| (project.contract_address))
            .collect::<Vec<_>>();
        let (res, page_count) = RewardHolder::list(
            conn,
            project_contracts,
            &user_account.to_string(),
            page,
            PAGE_SIZE,
        )?;

        Ok(Json(PagedResponse {
            data: res,
            page_count,
            page,
        }))
    }
}

pub struct AdminApi;

#[OpenApi]
impl AdminApi {
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
    pub async fn find(
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

    /// Lists the forest projects by status, paginated by the provided page number.
    /// Only admins can access this endpoint.
    ///
    /// # Arguments
    /// - `claims`: The claims of the authenticated user.
    /// - `db_pool`: The database connection pool.
    /// - `status`: The status of the forest projects to list.
    /// - `page`: The page number to retrieve.
    ///
    /// # Returns
    /// A `PagedResponse` containing the forest projects with the provided status and the total number of pages.
    #[oai(
        path = "/admin/forest_projects/list/:status/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn list_by_status(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(status): Path<ForestProjectState>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProject>> {
        ensure_is_admin(&claims)?;
        let (projects, page_count) =
            ForestProject::list_by_status(&mut db_pool.get()?, status, page, PAGE_SIZE)?;
        Ok(Json(PagedResponse {
            data: projects,
            page_count,
            page,
        }))
    }

    #[oai(
        path = "/admin/forest_projects/list/:page",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProject>> {
        ensure_is_admin(&claims)?;
        let (projects, page_count) = ForestProject::list(&mut db_pool.get()?, page, PAGE_SIZE)?;
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
    pub async fn create(
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
        let project = project.insert(conn)?;
        Ok(Json(project))
    }

    #[oai(
        path = "/admin/forest_projects",
        method = "put",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn update(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(project): Json<ForestProject>,
    ) -> JsonResult<ForestProject> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        match project.state {
            ForestProjectState::Funding => {
                let mint_fund_contract =
                    project.mint_fund(conn)?.ok_or(Error::BadRequest(PlainText(
                        "Mint fund contract address is required and Mint fund contract should be \
                         synced with indexer"
                            .to_string(),
                    )))?;
                if mint_fund_contract.fund_state != SecurityMintFundState::Open {
                    return Err(Error::BadRequest(PlainText(
                        "Mint fund contract must be optn".to_string(),
                    )));
                }
            }
            ForestProjectState::Funded => {
                let mint_fund_contract =
                    project.mint_fund(conn)?.ok_or(Error::BadRequest(PlainText(
                        "Mint fund contract address is required and Mint fund contract should be \
                         synced with indexer"
                            .to_string(),
                    )))?;
                if mint_fund_contract.fund_state != SecurityMintFundState::Success {
                    return Err(Error::BadRequest(PlainText(
                        "Mint fund contract must be closed".to_string(),
                    )));
                }
            }
            _ => {}
        }
        let project = project.update(conn)?;
        Ok(Json(project))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/price/latest",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn price_latest(
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
    pub async fn price_at(
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
    pub async fn price_list(
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
    pub async fn price_create(
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
    pub async fn price_delete(
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
}

#[derive(Object)]
pub struct ForestProjectApi {
    project: db::forest_project::ForestProject,
}

impl ForestProjectApi {
    pub fn list_by_status(
        conn: &mut DbConn,
        status: ForestProjectState,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let (projects, page_count) =
            db::forest_project::ForestProject::list_by_status(conn, status, page, page_size)?;
        let projects = projects
            .into_iter()
            .map(|project| Self { project })
            .collect();
        Ok((projects, page_count))
    }
}

#[derive(Object)]
pub struct ForestProjectUser {
    pub project:      ForestProject,
    pub token_holder: TokenHolder,
}

#[derive(Object)]
pub struct ForestProjectWithNotification {
    pub project:         db::forest_project::ForestProject,
    pub notification_id: Option<Uuid>,
}

impl ForestProjectWithNotification {
    pub fn list_by_status(
        conn: &mut DbConn,
        status: ForestProjectState,
        user_cognito_id: &str,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<Self>, i64)> {
        let res: Vec<(db::forest_project::ForestProject, Option<Uuid>)> =
            schema::forest_projects::table
                .left_join(schema::forest_project_notifications::table)
                .filter(schema::forest_projects::state.eq(status))
                .filter(schema::forest_project_notifications::cognito_user_id.eq(user_cognito_id))
                .order(schema::forest_projects::created_at.desc())
                .limit(page_size)
                .offset(page * page_size)
                .select((
                    db::forest_project::ForestProject::as_select(),
                    schema::forest_project_notifications::id.nullable(),
                ))
                .get_results(conn)?;
        let total_count = schema::forest_projects::table
            .filter(schema::forest_projects::state.eq(status))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        let res = res
            .into_iter()
            .map(|(project, notification_id)| ForestProjectWithNotification {
                project,
                notification_id,
            })
            .collect();

        Ok((res, page_count))
    }
}

#[derive(Object)]
pub struct ForestProjectDetails {
    pub project:             db::forest_project::ForestProject,
    pub contract:            SecuritySftRewardsContract,
    pub project_token:       events_listener::txn_processor::cis2_security::db::Token,
    pub media:               Vec<db::forest_project::PropertyMedia>,
    pub mint_fund:           Option<SecurityMintFundContract>,
    pub mint_fund_token:     Option<events_listener::txn_processor::cis2_security::db::Token>,
    pub p2p_trade:
        Option<events_listener::txn_processor::security_p2p_trading::db::P2PTradeContract>,
    pub user_notification:   Option<db::forest_project::Notification>,
    pub user_legal_contract: Option<db::forest_project::LegalContractUserSignature>,
    pub legal_contract:      Option<db::forest_project::LegalContract>,
}

impl ForestProjectDetails {
    pub fn find(
        conn: &mut DbConn,
        project_id: uuid::Uuid,
        cognito_user_id: &str,
    ) -> DbResult<Option<Self>> {
        let project = match db::forest_project::ForestProject::find(conn, project_id)? {
            Some(project) => project,
            None => return Ok(None),
        };
        let contract = match project.contract(conn)? {
            Some(contract) => contract,
            None => return Ok(None),
        };
        let project_token = match contract.token_tracked(conn)? {
            Some(token) => token,
            None => return Ok(None),
        };
        let (media, _) = project.media(conn, 0, MEDIA_LIMIT)?;
        let mint_fund = project.mint_fund(conn)?;
        let mint_fund_token = mint_fund
            .as_ref()
            .map(|f| f.token(conn))
            .transpose()?
            .flatten();
        let p2p_trade = project.p2p_trade(conn)?;
        let user_notification = project.user_notification(conn, cognito_user_id)?;
        let legal_contract = project.legal_contract(conn)?;
        let user_legal_contract = project.user_legal_contract(conn, cognito_user_id)?;

        Ok(Some(Self {
            project,
            project_token,
            media,
            mint_fund,
            mint_fund_token,
            p2p_trade,
            user_notification,
            user_legal_contract,
            legal_contract,
            contract,
        }))
    }
}
