use actix_web::{HttpResponse, web::{Json}, Error};
use serde::Serialize;

pub fn respond_json<T>(data: T) -> Result<Json<T>, Error>
where
    T: Serialize,
{
    Ok(Json(data))
}

pub fn respond_ok() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
