use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MirrorJsonDTO {
    message: String,
    first_name: String
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    port: i32,
    host: String,
    admin: String
}

pub async fn mirror_json_handler(State(state): State<Arc<service_context::Container>>, Json(payload): Json<MirrorJsonDTO>) -> Json<MirrorJsonResponse> {
    println!("Request by - {}", payload.first_name);
    Json(MirrorJsonResponse{
        host: "localhost".to_string(),
        message: payload.message,
        port: 3000,
        admin: state.wow_created_by.to_string()
    })
}