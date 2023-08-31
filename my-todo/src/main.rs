use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

// 非同期関数であると明記
#[tokio::main]
async fn main() {
    // わざわざto_stringを使っているのは文字列データの参照が欲しいから
    // "hoge"は参照渡しを行うためto_stringで実態を渡している
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root)).route("/users", post(create_user));
    // std::convert::Fromトレイトを実装しているため、formメソッドが使える
    // Fromは引数をある方を呼び元の方に変換できる関連関数
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    // println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        // 値がなかった場合はpanicを返す
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user (
    Json(playload): Json<CreateUser>,
    // impl 継承のようなもの
) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: playload.username
    };
    // StatusCode::CREATEDは201を返す
    (StatusCode::CREATED, Json(user))
}

// 特定のトレイト（trait）の自動実装を指示するための属性
// デシリアライズ可能な型として扱われます
// 様々なフォーマットからRustの構造体に変換する過程はデシリアライズと呼ばれます。
#[derive(Deserialize)]
struct CreateUser {
    username: String, 
}

#[derive(Serialize)]
struct  User {
    id: u64,
    username: String,
}