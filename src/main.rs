use axum::{routing::{ get, post }, Router, extract::{ State, Json }, http::StatusCode};
use std::sync::Arc;

use tokio::sync::Mutex;

use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

struct DBInfo {
    client: tokio_postgres::Client,
}

mod music;
use music::MusicLog;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {    
    // Connect to the database.
    let (mut client, connection) = tokio_postgres::connect("postgresql://postgres:zyk54321!!!@localhost/cloud_music_statistics", tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // music::create_table(&mut client).await?;
    // println!("add success");

    let db_info = Arc::new(Mutex::new(DBInfo {
        client,
    }));

    let app = Router::new()
        .route("/add_listen_logs", post(add_listen_logs))
        .with_state(db_info)
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                .allow_headers(Any)
                .allow_methods(Any)
                .allow_origin(Any)
            )
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4100").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn add_listen_logs(
    State(state): State<Arc<Mutex<DBInfo>>>,
    Json(payload): Json<Vec<MusicLog>>,
) -> (StatusCode, String) {
    let mut locked_state = state.lock().await;
    match music::add_logs(&mut locked_state.client, payload).await {
        Ok(_)=> (StatusCode::OK, "".to_string()),
        Err(err)=> (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}
