use actix_web::web;
use crate::app::{
    health::routes::health_routes,
    auth::routes::auth_routes,
    user::routes::user_routes,
    todo::routes::todo_routes,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .configure(health_routes)
        .service(
            web::scope("/api")
                .service(
                    web::scope("/v1")
                        .configure(auth_routes)
                        .configure(user_routes)
                        .configure(todo_routes)
                )
        );
}