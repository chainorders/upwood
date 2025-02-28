use diesel::prelude::*;
use poem_openapi::Object;
use serde::Serialize;
use tracing::instrument;
use uuid::Uuid;

use crate::db_shared::{DbConn, DbResult};
use crate::schema::{
    guides, maintenance_messages, news_articles, platform_updates, support_questions,
};

#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable, Serialize)]
#[diesel(table_name = news_articles)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewsArticle {
    pub id:          Uuid,
    pub title:       String,
    pub label:       String,
    pub content:     String,
    pub image_url:   String,
    pub article_url: String,
    pub created_at:  chrono::NaiveDateTime,
    pub order_index: i32,
}

impl NewsArticle {
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<NewsArticle>> {
        news_articles::table
            .filter(news_articles::id.eq(id))
            .select(NewsArticle::as_select())
            .first(conn)
            .optional()
    }

    pub fn find_first(conn: &mut DbConn) -> DbResult<Option<NewsArticle>> {
        news_articles::table
            .select(NewsArticle::as_select())
            .order_by(news_articles::order_index.asc())
            .first(conn)
            .optional()
    }

    #[instrument(skip(conn))]
    pub fn list(conn: &mut DbConn, page: i64, page_size: i64) -> DbResult<(Vec<NewsArticle>, i64)> {
        let query = news_articles::table.select(NewsArticle::as_select());
        let news_articles = query
            .limit(page_size)
            .offset(page * page_size)
            .order_by(news_articles::order_index.asc())
            .get_results(conn)?;
        let total_count: i64 = query.count().get_result(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;
        Ok((news_articles, page_count))
    }

    #[instrument(skip(conn))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(news_articles::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn delete(conn: &mut DbConn, id: Uuid) -> DbResult<()> {
        diesel::delete(news_articles::table.filter(news_articles::id.eq(id))).execute(conn)?;
        Ok(())
    }
}

#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable, Serialize)]
#[diesel(table_name = platform_updates)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PlatformUpdate {
    pub id:          Uuid,
    pub title:       String,
    pub label:       String,
    pub created_at:  chrono::NaiveDateTime,
    pub order_index: i32,
}

impl PlatformUpdate {
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<PlatformUpdate>> {
        platform_updates::table
            .filter(platform_updates::id.eq(id))
            .select(PlatformUpdate::as_select())
            .first(conn)
            .optional()
    }

    pub fn find_first(conn: &mut DbConn) -> DbResult<Option<PlatformUpdate>> {
        platform_updates::table
            .select(PlatformUpdate::as_select())
            .order_by(platform_updates::order_index.asc())
            .first(conn)
            .optional()
    }

    #[instrument(skip(conn))]
    pub fn list(
        conn: &mut DbConn,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<PlatformUpdate>, i64)> {
        let query = platform_updates::table.select(PlatformUpdate::as_select());
        let platform_updates = query
            .limit(page_size)
            .offset(page * page_size)
            .order_by(platform_updates::order_index.asc())
            .get_results(conn)?;
        let count: i64 = query.count().get_result(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((platform_updates, page_count))
    }

    #[instrument(skip(conn))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(platform_updates::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn delete(conn: &mut DbConn, id: Uuid) -> DbResult<()> {
        diesel::delete(platform_updates::table.filter(platform_updates::id.eq(id)))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable, Serialize)]
#[diesel(table_name = maintenance_messages)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MaintenanceMessage {
    pub id:          Uuid,
    pub message:     String,
    pub created_at:  chrono::NaiveDateTime,
    pub order_index: i32,
}

impl MaintenanceMessage {
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<MaintenanceMessage>> {
        maintenance_messages::table
            .filter(maintenance_messages::id.eq(id))
            .select(MaintenanceMessage::as_select())
            .first(conn)
            .optional()
    }

    pub fn find_first(conn: &mut DbConn) -> DbResult<Option<MaintenanceMessage>> {
        maintenance_messages::table
            .select(MaintenanceMessage::as_select())
            .order_by(maintenance_messages::order_index.asc())
            .first(conn)
            .optional()
    }

    #[instrument(skip(conn))]
    pub fn list(
        conn: &mut DbConn,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<MaintenanceMessage>, i64)> {
        let query = maintenance_messages::table.select(MaintenanceMessage::as_select());
        let maintenance_messages = query
            .limit(page_size)
            .offset(page * page_size)
            .order_by(maintenance_messages::order_index.asc())
            .get_results(conn)?;
        let count: i64 = query.count().get_result(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((maintenance_messages, page_count))
    }

    #[instrument(skip(conn))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(maintenance_messages::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn delete(conn: &mut DbConn, id: Uuid) -> DbResult<()> {
        diesel::delete(maintenance_messages::table.filter(maintenance_messages::id.eq(id)))
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable, Serialize)]
#[diesel(table_name = guides)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guide {
    pub id:          Uuid,
    pub title:       String,
    pub label:       String,
    pub created_at:  chrono::NaiveDateTime,
    pub order_index: i32,
}

impl Guide {
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<Guide>> {
        guides::table
            .filter(guides::id.eq(id))
            .select(Guide::as_select())
            .first(conn)
            .optional()
    }

    pub fn find_first(conn: &mut DbConn) -> DbResult<Option<Guide>> {
        guides::table
            .select(Guide::as_select())
            .order_by(guides::order_index.asc())
            .first(conn)
            .optional()
    }

    #[instrument(skip(conn))]
    pub fn list(conn: &mut DbConn, page: i64, page_size: i64) -> DbResult<(Vec<Guide>, i64)> {
        let query = guides::table.select(Guide::as_select());
        let guides = query
            .limit(page_size)
            .offset(page * page_size)
            .order_by(guides::order_index.asc())
            .get_results(conn)?;
        let count: i64 = query.count().get_result(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((guides, page_count))
    }

    #[instrument(skip(conn))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(guides::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn delete(conn: &mut DbConn, id: Uuid) -> DbResult<()> {
        diesel::delete(guides::table.filter(guides::id.eq(id))).execute(conn)?;
        Ok(())
    }
}

#[derive(Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable, Serialize)]
#[diesel(table_name = support_questions)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SupportQuestion {
    pub id:              Uuid,
    pub cognito_user_id: String,
    pub user_email:      String,
    pub message:         String,
    pub created_at:      chrono::NaiveDateTime,
}

impl SupportQuestion {
    pub fn find(conn: &mut DbConn, id: Uuid) -> DbResult<Option<SupportQuestion>> {
        support_questions::table
            .filter(support_questions::id.eq(id))
            .select(SupportQuestion::as_select())
            .first(conn)
            .optional()
    }

    #[instrument(skip(conn))]
    pub fn list(
        conn: &mut DbConn,
        page: i64,
        page_size: i64,
    ) -> DbResult<(Vec<SupportQuestion>, i64)> {
        let query = support_questions::table.select(SupportQuestion::as_select());
        let support_queries = query
            .limit(page_size)
            .offset(page * page_size)
            .get_results(conn)?;
        let count: i64 = query.count().get_result(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;
        Ok((support_queries, page_count))
    }

    #[instrument(skip(conn))]
    pub fn insert(&self, conn: &mut DbConn) -> DbResult<()> {
        diesel::insert_into(support_questions::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }

    #[instrument(skip(conn))]
    pub fn delete(conn: &mut DbConn, id: Uuid) -> DbResult<()> {
        diesel::delete(support_questions::table.filter(support_questions::id.eq(id)))
            .execute(conn)?;
        Ok(())
    }
}
