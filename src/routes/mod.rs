mod hello_world;
mod mirror_body_string;
mod mirror_json;

use std::sync::Arc;

use axum::{Router, routing::{get, post}, body::Body};
use hello_world::get_handler;
use mirror_body_string::mirror_body_string_handler;

use self::mirror_json::mirror_json_handler;
pub async fn setup_routes(app_state_container: Arc<service_context::Container>) -> Router<(), Body> {
    let router = Router::new()
    .route("/", 
    get(get_handler))
    .route("/mirror_body_string", post(mirror_body_string_handler))
    .route("/mirror-json", post(mirror_json_handler))
    .with_state(app_state_container);

    return router
}