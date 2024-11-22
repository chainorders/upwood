use std::cmp;

use diesel::Connection;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::OpenApi;
use shared::api::PagedResponse;
use shared::db::security_mint_fund::SecurityMintFundState;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectHolderRewardTotal, ForestProjectInvestor, ForestProjectMedia,
    ForestProjectPrice, ForestProjectState, ForestProjectUser, HolderReward,
};
use tracing::{debug, info};

use super::*;
pub const MEDIA_LIMIT: i64 = 4;
pub struct ForestProjectApi;

#[OpenApi]
impl ForestProjectApi {
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
    pub async fn forest_project_list_active(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectUser>> {
        let conn = &mut db_pool.get()?;
        let res = ForestProjectUser::list(
            conn,
            &claims.sub,
            claims.account(),
            SecurityMintFundState::Open,
            page,
            i64::MAX,
        );
        let (projects, page_count) = match res {
            Ok((projects, page_count)) => (projects, page_count),
            Err(e) => {
                error!("Failed to list active projects: {}", e);
                return Err(Error::InternalServer(PlainText(format!(
                    "Failed to list active projects: {}",
                    e
                ))));
            }
        };
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
    pub async fn forest_project_list_funded(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectUser>> {
        let conn = &mut db_pool.get()?;
        let (projects, page_count) = ForestProjectUser::list(
            conn,
            &claims.sub,
            claims.account(),
            SecurityMintFundState::Success,
            page,
            i64::MAX,
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
    pub async fn forest_project_list_owned(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<PagedResponse<ForestProjectUser>> {
        let user_account = ensure_account_registered(&claims)?;
        let conn = &mut db_pool.get()?;
        let (projects, _) =
            ForestProjectUser::list_owned(conn, &claims.sub, user_account, 0, i64::MAX)?;

        Ok(Json(PagedResponse {
            data:       projects,
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
    pub async fn forest_project_find(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
    ) -> JsonResult<ForestProjectUser> {
        let project = ForestProjectUser::find(
            &mut db_pool.get()?,
            project_id,
            &claims.sub,
            claims.account(),
        )?
        .ok_or(Error::NotFound(PlainText(format!(
            "Forest project not found: {}",
            project_id
        ))))?;
        Ok(Json(project))
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
        path = "/forest_projects/rewards/total",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_rewards_total(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<Vec<ForestProjectHolderRewardTotal>> {
        let account = ensure_account_registered(&claims)?;
        let conn = &mut db_pool.get()?;
        let rewards = ForestProjectHolderRewardTotal::list(conn, &account.to_string())?;
        Ok(Json(rewards))
    }

    #[oai(
        path = "/forest_projects/rewards/claimable",
        method = "get",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn forest_project_rewards_claimable(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<Vec<HolderReward>> {
        let account = ensure_account_registered(&claims)?;
        let conn = &mut db_pool.get()?;
        let rewards = HolderReward::list(conn, &account.to_string())?;
        Ok(Json(rewards))
    }
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
            ForestProject::list(&mut db_pool.get()?, state, page, PAGE_SIZE)?;
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
        ForestProjectPrice {
            price:      project.latest_price,
            project_id: project.id,
            price_at:   project.created_at,
        }
        .insert(conn)?;
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
        Json(project): Json<ForestProject>,
    ) -> JsonResult<ForestProject> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let existing_project = ForestProject::find(conn, project.id)?.ok_or(Error::NotFound(
            PlainText(format!("Forest project not found: {}", project.id)),
        ))?;
        let project = conn.transaction(|conn| {
            if existing_project.latest_price != project.latest_price {
                let price = ForestProjectPrice {
                    price:      project.latest_price,
                    project_id: project.id,
                    price_at:   project.updated_at,
                }
                .insert(conn);
                match price {
                    Ok(price) => {
                        debug!("Inserted price: {:?}", price);
                    }
                    Err(e) => {
                        error!("Failed to insert price: {}", e);
                        return Err(Error::InternalServer(PlainText(format!(
                            "Failed to insert price: {}",
                            e
                        ))));
                    }
                }
            }
            debug!("Updating project: {:?}", project);
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

            Ok(project)
        })?;

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
        conn.transaction::<_, Error, _>(|conn| {
            let price = price.insert(conn)?;
            // Update the latest price and updated_at fields of the forest project
            let mut forest_project =
                ForestProject::find(conn, project_id)?.ok_or(Error::NotFound(PlainText(
                    format!("Forest project not found: {}", project_id),
                )))?;
            let latest_price =
                ForestProjectPrice::latest(conn, project_id)?.ok_or(Error::NotFound(PlainText(
                    format!("Latest price not found for project: {}", project_id),
                )))?;
            forest_project.latest_price = latest_price.price;
            forest_project.updated_at = cmp::max(forest_project.updated_at, price.price_at);
            forest_project.update(conn)?;
            Ok(price)
        })?;
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
        conn.transaction(|conn| {
            ForestProjectPrice::delete(conn, project_id, price_at)?;
            let latest_price =
                ForestProjectPrice::latest(conn, project_id)?.ok_or(Error::NotFound(PlainText(
                    format!("Latest price not found for project: {}", project_id),
                )))?;
            let mut forest_project =
                ForestProject::find(conn, project_id)?.ok_or(Error::NotFound(PlainText(
                    format!("Forest project not found: {}", project_id),
                )))?;
            forest_project.latest_price = latest_price.price;
            forest_project.updated_at = latest_price.price_at;
            forest_project.update(conn)?;
            NoResResult::Ok(())
        })?;
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
        Path(project_id): Path<uuid::Uuid>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectInvestor>> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let (investors, page_count) =
            ForestProjectInvestor::list(conn, project_id, page, PAGE_SIZE)?;
        Ok(Json(PagedResponse {
            data: investors,
            page_count,
            page,
        }))
    }
}
