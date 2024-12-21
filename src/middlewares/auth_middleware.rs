use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http::header::AUTHORIZATION, Error, HttpMessage};
use actix_web::middleware::Next;
use actix_web::web::Data;
use crate::app_state::AppState;
use crate::services::models::user::User;
use crate::shared::jwt::decode_jwt;
use crate::shared::models::api_error::ServerError;

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get(AUTHORIZATION);

    if auth.is_none() {
        return Err(Error::from(ServerError::NoBearerToken));
    }

    let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
    let decoded_token = decode_jwt(token);

    if let Err(_) = decoded_token {
        return Err(Error::from(ServerError::InvalidTokenFormat));
    }
    let claims = decoded_token.unwrap().claims;

    let app_state = req.app_data::<Data<AppState>>();
    if let None = app_state {
        return Err(Error::from(ServerError::UnknownError("No app_data".to_string())));
    }
    let app_state = app_state.unwrap();

    let user = User::by_id(
        claims.id,
        &mut app_state.database.get().unwrap(),
    );

    if let Err(_) = user {
        return Err(Error::from(ServerError::UnknownError("User not found".to_string())));
    }
    let user = user.unwrap();

    req.extensions_mut().insert(user);

    next.call(req).await
        .map_err(|err| Error::from(ServerError::UnknownError(format!("{}", err))))
}