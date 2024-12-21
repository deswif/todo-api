use actix_web::Error;
use actix_web::web::Json;
use crate::app::auth::dto::{RegisterRequest, RegisterResponse, TokenDetails};
use crate::app::user::dto::UserResponse;
use crate::app_state::AppState;
use crate::helpers::respond_json;
use crate::services::models::user::{CreateUserDTO};
use crate::shared::jwt;
use crate::shared::models::api_error::ServerError;

pub async fn register_action(req: RegisterRequest, app_state: &AppState) -> Result<Json<RegisterResponse>, Error> {
    let mut conn = app_state.database.get().unwrap();

    let res = CreateUserDTO {
        email: req.email.clone(),
        password_hash: req.password.clone(),
    }.insert(&mut conn);


    if let Err(e) = res {
        return Err(Error::from(ServerError::UnknownError(e.to_string())));
    }

    let user = res.unwrap();

    let token = jwt::encode_jwt(user.id).ok();

    if let None = token {
        return Err(Error::from(ServerError::UnknownError(String::new())));
    }
    let token = token.unwrap();

    respond_json(
        RegisterResponse {
            token: TokenDetails {
                token
            },
            user: UserResponse::new(&user),
        }
    )
}