use actix_web::{web, Error};
use actix_web::web::{post, Data, Json};
use crate::app::auth::actions::login_action::login_action;
use crate::app::auth::actions::register_action::register_action;
use crate::app::auth::dto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::app_state::AppState;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", post().to(login))
            .route("/register", post().to(register))
    );
}

async fn login(req: Json<LoginRequest>, app_state: Data<AppState>) -> Result<Json<LoginResponse>, Error> {
    login_action(req.into_inner(), &app_state).await
}

async fn register(req: Json<RegisterRequest>, app_state: Data<AppState>) -> Result<Json<RegisterResponse>, Error> {
    register_action(req.into_inner(), &app_state).await
}
