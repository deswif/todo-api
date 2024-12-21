use actix_web::{web, Error};
use actix_web::middleware::from_fn;
use actix_web::web::{get, patch, post, Data, Json};
use diesel::{update, ExpressionMethods, Insertable, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use crate::app::todo::dto::{CompleteTodoRequest, CreateTodoRequest, TodoResponse};
use crate::app_state::AppState;
use crate::helpers::respond_json;
use crate::middlewares::auth_middleware::check_auth_middleware;
use crate::services::models::todo::{CreateTodoDTO, Todo};
use crate::services::models::user::User;
use crate::shared::models::api_error::ServerError;

pub fn todo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .wrap(from_fn(check_auth_middleware))
            .route("", post().to(create_todo))
            .route("", get().to(get_todos))
            .service(
                web::scope("/{todo_id}")
                    .route("/complete", patch().to(complete_todo))
                    .route("", get().to(get_todo))
            )
    );
}

async fn create_todo(create_todo_request: Json<CreateTodoRequest>, user: User, app_state: Data<AppState>) -> Result<Json<TodoResponse>, Error> {
    use crate::schema::todo::dsl::*;

    let mut conn = app_state.database.get().unwrap();

    let created_todo = CreateTodoDTO {
        user_id: user.id,
        title: create_todo_request.title.clone(),
    }
        .insert_into(todo)
        .get_result::<Todo>(&mut conn);

    if let Err(e) = created_todo {
        return Err(Error::from(ServerError::UnknownError(e.to_string())));
    }
    let created_todo = created_todo.unwrap();

    respond_json(TodoResponse::new(&created_todo))
}

async fn get_todos(user: User, app_state: Data<AppState>) -> Result<Json<Vec<TodoResponse>>, Error> {
    use crate::schema::todo::dsl::*;

    let mut conn = app_state.database.get().unwrap();

    let todos = todo::table()
        .filter(user_id.eq(user.id))
        .get_results::<Todo>(&mut conn);

    if let Err(e) = todos {
        return Err(Error::from(ServerError::UnknownError(e.to_string())));
    }
    let todos = todos.unwrap();
    let todos = todos.iter().map(|e| TodoResponse::new(e)).collect();

    respond_json(todos)
}

async fn complete_todo(complete_todo_request: Option<Json<CompleteTodoRequest>>, todo_id: web::Path<i32>, app_state: Data<AppState>) -> Result<Json<TodoResponse>, Error> {
    use crate::schema::todo::dsl::*;

    let mut conn = app_state.database.get().unwrap();

    let mut is_complete_var = true;

    if let Some(req) = complete_todo_request {
        if let Some(is_complete_req) = req.is_complete {
            is_complete_var = is_complete_req;
        }
    }

    let completed_todo = update(todo.find(todo_id.into_inner()))
        .set(is_complete.eq(is_complete_var))
        .get_result::<Todo>(&mut conn);

    if let Err(e) = completed_todo {
        return Err(Error::from(ServerError::UnknownError(e.to_string())));
    }
    let completed_todo = completed_todo.unwrap();

    respond_json(TodoResponse::new(&completed_todo))
}

async fn get_todo(user: User, todo_id: web::Path<i32>, app_state: Data<AppState>) -> Result<Json<TodoResponse>, Error> {
    use crate::schema::todo::dsl::*;

    let mut conn = app_state.database.get().unwrap();

    let todo_res = todo
        .find(todo_id.into_inner())
        .get_result::<Todo>(&mut conn);

    if let Err(e) = todo_res {
        return Err(Error::from(ServerError::UnknownError(e.to_string())));
    }
    let todo_res = todo_res.unwrap();
    if todo_res.user_id != user.id {
        return Err(Error::from(ServerError::UnknownError("No access".to_string())));
    }

    respond_json(TodoResponse::new(&todo_res))
}
