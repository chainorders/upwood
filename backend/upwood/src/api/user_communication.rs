use poem::web::Data;
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use shared::api::PagedResponse;
use shared::db_app::user_communication::{
    Guide, MaintenanceMessage, NewsArticle, PlatformUpdate, SupportQuestion,
};
use shared::db_shared::DbPool;
use uuid::Uuid;

use super::{ensure_is_admin, ApiTags, BearerAuthorization, JsonResult, NoResResult, PAGE_SIZE};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(
        path = "/news_articles/list",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn news_articles_list(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<NewsArticle>> {
        let mut conn = db_pool.get()?;
        let (news_articles, count) = NewsArticle::list(&mut conn, page, page_size.unwrap_or(2))?;
        Ok(Json(PagedResponse {
            data: news_articles,
            page_count: count,
            page,
        }))
    }

    #[oai(
        path = "/admin/news_articles/:id",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn news_articles_get(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> JsonResult<Option<NewsArticle>> {
        let mut conn = db_pool.get()?;
        Ok(Json(NewsArticle::find(&mut conn, id)?))
    }

    #[oai(
        path = "/admin/news_articles",
        method = "post",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn news_articles_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(news_article): Json<NewsArticle>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        news_article.insert(&mut conn)?;
        Ok(())
    }

    #[oai(
        path = "/admin/news_articles/:id",
        method = "delete",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn news_articles_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        NewsArticle::delete(&mut conn, id)?;
        Ok(())
    }

    #[oai(
        path = "/platform_updates/list",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn platform_updates_list(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<PlatformUpdate>> {
        let mut conn = db_pool.get()?;
        let (platform_updates, count) =
            PlatformUpdate::list(&mut conn, page, page_size.unwrap_or(PAGE_SIZE))?;
        Ok(Json(PagedResponse {
            data: platform_updates,
            page_count: count,
            page,
        }))
    }

    #[oai(
        path = "/admin/platform_updates/:id",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn platform_updates_get(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> JsonResult<Option<PlatformUpdate>> {
        let mut conn = db_pool.get()?;
        Ok(Json(PlatformUpdate::find(&mut conn, id)?))
    }

    #[oai(
        path = "/platform_updates/latest",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn platform_updates_latest(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<Option<PlatformUpdate>> {
        let mut conn = db_pool.get()?;
        Ok(Json(PlatformUpdate::find_first(&mut conn)?))
    }

    #[oai(
        path = "/admin/platform_updates",
        method = "post",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn platform_updates_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(platform_update): Json<PlatformUpdate>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        platform_update.insert(&mut conn)?;
        Ok(())
    }

    #[oai(
        path = "/admin/platform_updates/:id",
        method = "delete",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn platform_updates_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        PlatformUpdate::delete(&mut conn, id)?;
        Ok(())
    }

    #[oai(
        path = "/maintenance_messages/list",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn maintenance_messages_list(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<MaintenanceMessage>> {
        let mut conn = db_pool.get()?;
        let (maintenance_messages, count) =
            MaintenanceMessage::list(&mut conn, page, page_size.unwrap_or(PAGE_SIZE))?;
        Ok(Json(PagedResponse {
            data: maintenance_messages,
            page_count: count,
            page,
        }))
    }

    #[oai(
        path = "/admin/maintenance_messages/:id",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn maintenance_messages_get(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> JsonResult<Option<MaintenanceMessage>> {
        let mut conn = db_pool.get()?;
        Ok(Json(MaintenanceMessage::find(&mut conn, id)?))
    }

    #[oai(
        path = "/maintenance_messages/latest",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn maintenance_messages_latest(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
    ) -> JsonResult<Option<MaintenanceMessage>> {
        let mut conn = db_pool.get()?;
        Ok(Json(MaintenanceMessage::find_first(&mut conn)?))
    }

    #[oai(
        path = "/admin/maintenance_messages",
        method = "post",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn maintenance_messages_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(maintenance_message): Json<MaintenanceMessage>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        maintenance_message.insert(&mut conn)?;
        Ok(())
    }

    #[oai(
        path = "/admin/maintenance_messages/:id",
        method = "delete",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn maintenance_messages_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        MaintenanceMessage::delete(&mut conn, id)?;
        Ok(())
    }

    #[oai(
        path = "/guides/list",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn guides_list(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<Guide>> {
        let mut conn = db_pool.get()?;
        let (guides, count) = Guide::list(&mut conn, page, page_size.unwrap_or(PAGE_SIZE))?;
        Ok(Json(PagedResponse {
            data: guides,
            page_count: count,
            page,
        }))
    }

    #[oai(
        path = "/admin/guides/:id",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn guides_get(
        &self,
        BearerAuthorization(_claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> JsonResult<Option<Guide>> {
        let mut conn = db_pool.get()?;
        Ok(Json(Guide::find(&mut conn, id)?))
    }

    #[oai(
        path = "/admin/guides",
        method = "post",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn guides_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(guide): Json<Guide>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        guide.insert(&mut conn)?;
        Ok(())
    }

    #[oai(
        path = "/admin/guides/:id",
        method = "delete",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn guides_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        Guide::delete(&mut conn, id)?;
        Ok(())
    }

    #[oai(
        path = "/admin/support_questions/list",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn support_questions_list(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Query(page): Query<i64>,
        Query(page_size): Query<Option<i64>>,
    ) -> JsonResult<PagedResponse<SupportQuestion>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        let (support_questions, count) =
            SupportQuestion::list(&mut conn, page, page_size.unwrap_or(PAGE_SIZE))?;
        Ok(Json(PagedResponse {
            data: support_questions,
            page_count: count,
            page,
        }))
    }

    #[oai(
        path = "/admin/support_questions/:id",
        method = "get",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn support_questions_get(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> JsonResult<Option<SupportQuestion>> {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        Ok(Json(SupportQuestion::find(&mut conn, id)?))
    }

    #[oai(
        path = "/support_questions",
        method = "post",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn support_questions_create(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Json(message): Json<String>,
    ) -> NoResResult {
        let mut conn = db_pool.get()?;
        SupportQuestion {
            id: Uuid::new_v4(),
            cognito_user_id: claims.sub.clone(),
            user_email: claims.email.clone(),
            message,
            created_at: chrono::Utc::now().naive_utc(),
        }
        .insert(&mut conn)?;
        Ok(())
    }

    #[oai(
        path = "/admin/support_questions/:id",
        method = "delete",
        tag = "ApiTags::UserCommunication"
    )]
    pub async fn support_questions_delete(
        &self,
        BearerAuthorization(claims): BearerAuthorization,
        Data(db_pool): Data<&DbPool>,
        Path(id): Path<Uuid>,
    ) -> NoResResult {
        ensure_is_admin(&claims)?;
        let mut conn = db_pool.get()?;
        SupportQuestion::delete(&mut conn, id)?;
        Ok(())
    }
}
