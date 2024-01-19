use axum::{routing::get, Router};

mod config;
mod database;
mod route;
mod template;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = config::Config::default();
    database::create_database_if_not_exists(&config.database_url).await;
    let connection = database::connection_pool(&config.database_url).await;
    database::apply_migrations(&connection).await;

    let app = Router::new()
        .route("/", get(route::index))
        .route("/modal", get(route::modal))
        .with_state(connection);

    let listener = tokio::net::TcpListener::bind(&config.address())
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
