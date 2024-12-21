use std::future;
use std::time::SystemTime;
use actix_web::{FromRequest, HttpMessage};
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use crate::schema::users;
use crate::schema::users::{email};
use crate::services::database::DBConnection;

#[derive(Queryable, Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
}

impl User {
    pub fn by_email(n_email: String, conn: &mut DBConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users::table()
            .filter(email.eq(&n_email))
            .first::<User>(conn)
    }

    pub fn by_id(n_id: i32, conn: &mut DBConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users::table()
            .filter(id.eq(&n_id))
            .first::<User>(conn)
    }
}

impl FromRequest for User {
    type Error = actix_web::Error;

    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> future::Ready<Result<User, actix_web::Error>> {
        match req.extensions().get::<User>() {
            Some(user) => future::ready(Ok(user.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad User")))
        }
    }
}

#[derive(Insertable, Deserialize, Serialize, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct CreateUserDTO {
    pub email: String,
    pub password_hash: String,
}

impl CreateUserDTO {
    pub fn insert(self, conn: &mut DBConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        self
            .insert_into(users)
            .get_result::<User>(conn)
    }
}
