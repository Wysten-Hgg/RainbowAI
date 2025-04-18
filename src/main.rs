use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use axum::serve;
use std::env;
use dotenv::dotenv;

mod routes;
mod models;
mod db;
mod middleware {
    pub mod auth;
}
mod utils;
mod services;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();
    
    // 初始化数据库连接
    let db = db::Database::init()
        .await
        .expect("Failed to initialize database");
    
    // 创建应用路由
    let app = routes::create_routes(db);
    
    // 从环境变量获取服务器地址和端口
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    
    // 启动服务器
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
