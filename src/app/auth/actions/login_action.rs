use actix_web::Error;
use actix_web::web::Json;
use crate::app::auth::dto::{LoginRequest, LoginResponse, TokenDetails};
use crate::app::user::dto::UserResponse;
use crate::app_state::AppState;
use crate::helpers::respond_json;
use crate::services::models::user::User;
use crate::shared::jwt;
use crate::shared::models::api_error::ServerError;

pub async fn login_action(req: LoginRequest, app_state: &AppState) -> Result<Json<LoginResponse>, Error> {
    let res = User::by_email(req.email.clone(), &mut app_state.database.get().unwrap());

    if let Err(e) = res {
        return Err(Error::from(ServerError::UnknownError(e.to_string())));
    }
    let user = res.unwrap();

    if (user.password != req.password) {
        return Err(Error::from(ServerError::UnknownError("Password does not match".to_string())));
    }

    let token = jwt::encode_jwt(user.id).ok();

    if let None = token {
        return Err(Error::from(ServerError::UnknownError(String::new())));
    }
    let token = token.unwrap();

    respond_json(
        LoginResponse {
            token: TokenDetails {
                token
            },
            user: UserResponse::new(&user),
        }
    )
}
