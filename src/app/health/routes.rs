use actix_web::{
    web,
    web::{get, Json},
    Error,
};
use crate::{
    app::health::dto::HealthResponse,
    helpers::respond_json,
};

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", get().to(get_health))
    );
}

async fn get_health() -> Result<Json<HealthResponse>, Error> {
    respond_json(HealthResponse { status: "ok".into() })
}
