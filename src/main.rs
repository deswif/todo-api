mod env;
mod router;
mod app;
mod shared;
mod helpers;
mod middlewares;
mod app_state;
mod services;
mod schema;

use actix_web::{App, HttpServer, Error};
use crate::env::env::ENV;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{Data, JsonConfig};
use crate::app_state::AppState;
use crate::services::database;
use crate::router as r;
use crate::shared::models::api_error::ServerError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database::init();
    let app_state = Data::new(AppState { database: db });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().supports_credentials())
            .wrap(Logger::default())
            .app_data(JsonConfig::default().error_handler(|e, _| Error::from(ServerError::ParseError(e.to_string()))))
            .app_data(app_state.clone())
            .configure(r::routes)
    })
        .bind((ENV.host.as_str(), ENV.port))?
        .run()
        .await
}
