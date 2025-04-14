mod routes;
mod models;
mod db;
mod middleware;
mod utils;

use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // 初始化数据库连接
    let db = db::Database::init()
        .await
        .expect("Failed to initialize database");

    // 创建应用路由
    let app = routes::create_router(db)
        .layer(CorsLayer::permissive()); // 允许跨域请求

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
