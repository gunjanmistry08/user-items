use crate::{domain::user_service::UserService, state::AppState};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
}

pub async fn create_user(
    state: web::Data<AppState>,
    req: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let service = UserService::new((*state.producer).clone());

    let result = service
        .create_user(&state.kafka_topic, req.email.clone())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
