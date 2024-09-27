use diesel::prelude::*;

use crate::db::{DbConn, DbResult};
use crate::schema;
use crate::schema::users::dsl::*;

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id:              String,
    pub account_address: Option<String>,
}

pub fn insert(conn: &mut DbConn, user: &User) -> DbResult<String> {
    let user = diesel::insert_into(users)
        .values(user)
        .returning(schema::users::id)
        .get_result::<String>(conn)?;
    Ok(user)
}

pub fn find_by_id(conn: &mut DbConn, user_id: &str) -> DbResult<Option<User>> {
    let user = users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first(conn)
        .optional()?;
    Ok(user)
}
