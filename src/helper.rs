use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use std::sync::Arc;

use crate::handler::{
    create_note_handler, debug_logs_handler, health_check_handler, home_base_handler,
    note_list_handler, users_handler,
};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub fn router_create(appstate: Option<Arc<AppState>>) -> Router {
    let app_state = match appstate {
        Some(state) => state,
        None => panic!("You must pass Arc<AppState>> for now!"),
    };

    Router::new()
        .route("/", get(home_base_handler))
        .route("/api/health", get(health_check_handler))
        .route("/api/dev/debug/logs", get(debug_logs_handler))
        .route("/api/notes", post(create_note_handler))
        .route("/api/users", get(users_handler))
        .route("/api/notes", get(note_list_handler))
        .with_state(app_state)
}

pub fn server_url_init() -> String {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(err) => err.to_string(),
    };
    format!("0.0.0.0:{}", port)
}

pub async fn database_init() -> Arc<AppState> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database was successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    Arc::new(AppState { db: pool.clone() })
}
