use std::net::SocketAddr;
use tokio::net::TcpListener;
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
    let app = routes::create_routes(db.clone());
    
    // 从环境变量获取服务器地址和端口
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    
    // 启动WebSocket服务
    let ws_host = env::var("WS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let ws_port = env::var("WS_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);
    let ws_addr_display = format!("{}:{}", ws_host, ws_port);
    
    // 在单独的任务中启动WebSocket服务
    let ws_db = db.clone();
    let ws_addr = ws_addr_display.clone();
    tokio::spawn(async move {
        services::websocket::start_server(&ws_addr, ws_db).await;
    });
    
    // 启动HTTP服务器
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
    println!("HTTP Server running on http://{}", addr);
    println!("WebSocket Server running on ws://{}", ws_addr_display);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
