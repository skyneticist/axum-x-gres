pub mod handler;
pub mod helper;
pub mod model;
pub mod schema;

use crate::helper::{database_init, router_create, server_url_init};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000/api".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app_state = database_init().await;
    let app = router_create(Some(app_state)).layer(cors);
    let server_url = server_url_init(); // TODO: Get address from .env

    axum::Server::bind(&server_url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
