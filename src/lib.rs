mod routes;
use std::sync::Arc;

use routes::setup_routes;

pub async fn run() {
    let service_context = Arc::new(service_context::Container{
        wow_created_by: "Judd"
    });

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(setup_routes(service_context).await.into_make_service())
        .await
        .unwrap();
}