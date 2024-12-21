use actix_web::{web, Error};
use actix_web::middleware::from_fn;
use actix_web::web::{Json};
use crate::app::user::dto::UserResponse;
use crate::helpers::respond_json;
use crate::middlewares::auth_middleware::check_auth_middleware;
use crate::services::models::user::User;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(from_fn(check_auth_middleware))
            .route("/me", web::get().to(get_me))
    );
}

async fn get_me(user: User) -> Result<Json<UserResponse>, Error> {
    respond_json(UserResponse::new(&user))
}
