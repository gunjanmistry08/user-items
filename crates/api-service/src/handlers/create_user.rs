use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
}

pub async fn create_user(
    state: web::Data<AppState>,
    req: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let service = UserService::new((*state.producer).clone());
let result = service.create_user(req.email.clone()).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}