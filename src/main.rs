use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use axum::serve;

mod routes;
mod models;
mod db;
mod middleware {
    pub mod auth;
}
mod utils;

#[tokio::main]
async fn main() {
    // 初始化数据库连接
    let db = db::Database::init()
        .await
        .expect("Failed to initialize database");
    
    // 创建应用路由
    let app = routes::create_routes(db);
    
    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app);
}
