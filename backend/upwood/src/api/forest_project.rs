use diesel::Connection;
use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::OpenApi;
use shared::api::PagedResponse;
use shared::db::security_mint_fund::SecurityMintFundState;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectHolderRewardTotal, ForestProjectMedia, ForestProjectPrice,
    ForestProjectState, ForestProjectUser, HolderReward,
};
use tracing::{debug, info};

use super::*;
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
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(page): Path<i64>,
    ) -> JsonResult<PagedResponse<ForestProjectUser>> {
        let conn = &mut db_pool.get()?;
        let (projects, page_count) = ForestProjectUser::list(
            conn,
            &claims.sub,
            claims.account(),
            SecurityMintFundState::Open,
            page,
            i64::MAX,
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
    pub async fn list_owned(
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
    pub async fn find(
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
    pub async fn list_media(
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
    pub async fn find_media(
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
    pub async fn total_rewards(
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
    pub async fn claimable_rewards(
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
    pub async fn create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(mut project): Json<ForestProject>,
    ) -> JsonResult<ForestProject> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let now = chrono::Utc::now().naive_utc();
        if project.state != ForestProjectState::Draft {
            return Err(Error::BadRequest(PlainText(
                "Only draft projects can be created".to_string(),
            )));
        }
        project.created_at = now;
        project.updated_at = now;
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
        info!("Created project: {:?}", project);
        ForestProjectPrice {
            price:      project.latest_price,
            project_id: project.id,
            price_at:   chrono::Utc::now().naive_utc(),
        }
        .insert(conn)?;
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
        Json(mut project): Json<ForestProject>,
    ) -> JsonResult<ForestProject> {
        ensure_is_admin(&claims)?;
        let conn = &mut db_pool.get()?;
        let now = chrono::Utc::now().naive_utc();
        let existing_project = ForestProject::find(conn, project.id)?.ok_or(Error::NotFound(
            PlainText(format!("Forest project not found: {}", project.id)),
        ))?;
        if existing_project.latest_price != project.latest_price {
            ForestProjectPrice {
                price:      project.latest_price,
                project_id: project.id,
                price_at:   now,
            }
            .insert(conn)?;
        }
        project.updated_at = now;
        let project = project.update(conn)?;
        Ok(Json(project))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/media",
        method = "post",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn create_media(
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
    pub async fn delete_media(
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
    pub async fn find_price(
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
    pub async fn list_price(
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
    pub async fn create_price(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(project_id): Path<uuid::Uuid>,
        Json(price): Json<ForestProjectPrice>,
    ) -> JsonResult<ForestProjectPrice> {
        ensure_is_admin(&claims)?;
        let now = chrono::Utc::now().naive_utc();
        if project_id != price.project_id {
            return Err(Error::BadRequest(PlainText(
                "Project id in path and body must be the same".to_string(),
            )));
        }
        let conn = &mut db_pool.get()?;
        conn.transaction::<_, Error, _>(|conn| {
            let price = price.insert(conn)?;
            let mut forest_project =
                ForestProject::find(conn, project_id)?.ok_or(Error::NotFound(PlainText(
                    format!("Forest project not found: {}", project_id),
                )))?;
            if forest_project.latest_price.ne(&price.price) {
                forest_project.latest_price = price.price;
                forest_project.updated_at = now;
                forest_project.update(conn)?;
            }
            Ok(price)
        })?;
        Ok(Json(price))
    }

    #[oai(
        path = "/admin/forest_projects/:project_id/price/:price_at",
        method = "delete",
        tag = "ApiTags::ForestProject"
    )]
    pub async fn delete_price(
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
