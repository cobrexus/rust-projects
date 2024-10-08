use axum::{
    extract::RawQuery,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn index(RawQuery(url): RawQuery) -> impl IntoResponse {
    let url = url.unwrap();
    let url = url.split('=').nth(1).unwrap();
    let res = reqwest::get(url).await.unwrap().text().await.unwrap();
    Html(res)
}
