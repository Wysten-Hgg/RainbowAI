use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::net::{TcpListener, TcpStream};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::db::Database;
use crate::models::chat::{Message as ChatMessage, MessageType};
use crate::models::User;

type ClientId = String;
type UserId = String;
type Clients = Arc<RwLock<HashMap<ClientId, mpsc::UnboundedSender<Message>>>>;
type Users = Arc<RwLock<HashMap<UserId, Vec<ClientId>>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
    pub r#type: String,
    pub data: serde_json::Value,
}

pub struct WebSocketServer {
    clients: Clients,
    users: Users,
    db: Database,
}

impl WebSocketServer {
    pub fn new(db: Database) -> Self {
        WebSocketServer {
            clients: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(HashMap::new())),
            db,
        }
    }
    
    pub async fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).await.expect("Failed to bind");
        println!("WebSocket server running on: {}", addr);
        
        while let Ok((stream, _)) = listener.accept().await {
            let clients = self.clients.clone();
            let users = self.users.clone();
            let db = self.db.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, clients, users, db).await {
                    eprintln!("Error processing connection: {:?}", e);
                }
            });
        }
    }
    
    async fn handle_connection(
        stream: TcpStream,
        clients: Clients,
        users: Users,
        db: Database,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        // 生成客户端ID
        let client_id = Uuid::new_v4().to_string();
        
        // 存储客户端发送通道
        clients.write().await.insert(client_id.clone(), tx);
        
        // 发送初始化消息
        let init_msg = WsMessage {
            r#type: "init".to_string(),
            data: serde_json::json!({ "client_id": client_id }),
        };
        ws_sender.send(Message::Text(serde_json::to_string(&init_msg)?)).await?;
        
        // 处理从客户端接收的消息
        let clients_clone = clients.clone();
        let users_clone = users.clone();
        let client_id_clone = client_id.clone();
        let db_clone = db.clone();
        
        tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                if let Ok(msg) = msg {
                    if let Message::Text(text) = msg {
                        if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                            Self::process_message(ws_msg, &client_id_clone, clients_clone.clone(), users_clone.clone(), db_clone.clone()).await;
                        }
                    }
                } else {
                    break;
                }
            }
            
            // 客户端断开连接
            Self::client_disconnected(&client_id_clone, clients_clone, users_clone).await;
        });
        
        // 处理发送到客户端的消息
        while let Some(msg) = rx.recv().await {
            ws_sender.send(msg).await?;
        }
        
        Ok(())
    }
    
    async fn process_message(
        msg: WsMessage,
        client_id: &str,
        clients: Clients,
        users: Users,
        db: Database,
    ) {
        match msg.r#type.as_str() {
            "ping" => {
                // 处理心跳
                if let Some(sender) = clients.read().await.get(client_id) {
                    let pong_msg = WsMessage {
                        r#type: "pong".to_string(),
                        data: serde_json::json!({}),
                    };
                    let _ = sender.send(Message::Text(serde_json::to_string(&pong_msg).unwrap()));
                }
            },
            "bindUid" => {
                // 处理用户绑定
                if let Some(token) = msg.data.get("token") {
                    if let Some(token_str) = token.as_str() {
                        // 在实际应用中，这里应该验证token并获取用户ID
                        // 简化起见，我们假设token就是用户ID
                        Self::bind_user_to_client(token_str.to_string(), client_id.to_string(), users.clone()).await;
                        
                        // 发送绑定成功消息
                        if let Some(sender) = clients.read().await.get(client_id) {
                            let status_msg = WsMessage {
                                r#type: "pong".to_string(),
                                data: serde_json::json!({
                                    "multiport": false
                                }),
                            };
                            let _ = sender.send(Message::Text(serde_json::to_string(&status_msg).unwrap()));
                        }
                    }
                }
            },
            "message" => {
                // 处理消息发送
                if let Ok(data) = serde_json::from_value::<serde_json::Value>(msg.data.clone()) {
                    if let (Some(from_user), Some(to_user), Some(content), Some(is_group)) = (
                        data.get("from_user").and_then(|v| v.as_str()),
                        data.get("to_user").and_then(|v| v.as_str()),
                        data.get("content").and_then(|v| v.as_str()),
                        data.get("is_group").and_then(|v| v.as_bool()),
                    ) {
                        let message_type = match data.get("type").and_then(|v| v.as_str()) {
                            Some("image") => MessageType::Image,
                            Some("voice") => MessageType::Voice,
                            Some("video") => MessageType::Video,
                            Some("file") => MessageType::File,
                            Some("event") => MessageType::Event,
                            Some("system") => MessageType::System,
                            _ => MessageType::Text,
                        };
                        
                        let file_id = data.get("file_id").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let extends = data.get("extends").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let at = data.get("at").and_then(|v| v.as_str()).map(|s| s.to_string());
                        
                        let chat_msg = ChatMessage::new(
                            from_user.to_string(),
                            to_user.to_string(),
                            content.to_string(),
                            message_type,
                            is_group,
                            file_id,
                            extends,
                            at,
                        );
                        
                        Self::handle_chat_message(chat_msg, db.clone(), users.clone(), clients.clone()).await;
                    }
                }
            },
            _ => {}
        }
    }
    
    async fn bind_user_to_client(user_id: String, client_id: String, users: Users) {
        // 将用户ID绑定到客户端ID
        let mut users = users.write().await;
        users.entry(user_id).or_insert_with(Vec::new).push(client_id);
    }
    
    async fn handle_chat_message(
        msg: ChatMessage,
        db: Database,
        users: Users,
        clients: Clients,
    ) {
        // 保存消息到数据库
        if let Err(e) = db.create_message(&msg).await {
            eprintln!("Failed to save message: {:?}", e);
            return;
        }
        
        // 确定接收者
        let target_id = if msg.is_group {
            // 如果是群消息，获取群成员
            match db.get_group_users(&msg.to_user).await {
                Ok(group_users) => {
                    // 向所有群成员发送消息
                    for group_user in group_users {
                        Self::send_message_to_user(&group_user.user_id, &msg, &users, &clients).await;
                    }
                    return;
                },
                Err(e) => {
                    eprintln!("Failed to get group users: {:?}", e);
                    return;
                }
            }
        } else {
            // 如果是私聊消息，直接发送给接收者
            msg.to_user.clone()
        };
        
        // 发送消息给目标用户
        Self::send_message_to_user(&target_id, &msg, &users, &clients).await;
    }
    
    async fn send_message_to_user(
        user_id: &str,
        msg: &ChatMessage,
        users: &Users,
        clients: &Clients,
    ) {
        let users_read = users.read().await;
        let clients_read = clients.read().await;
        
        if let Some(client_ids) = users_read.get(user_id) {
            let ws_msg = WsMessage {
                r#type: "message".to_string(),
                data: serde_json::to_value(msg).unwrap(),
            };
            let msg_str = serde_json::to_string(&ws_msg).unwrap();
            
            for client_id in client_ids {
                if let Some(sender) = clients_read.get(client_id) {
                    let _ = sender.send(Message::Text(msg_str.clone()));
                }
            }
        }
    }
    
    async fn client_disconnected(client_id: &str, clients: Clients, users: Users) {
        // 从客户端列表中移除
        clients.write().await.remove(client_id);
        
        // 从所有用户的客户端列表中移除
        let mut users = users.write().await;
        for (_, client_ids) in users.iter_mut() {
            client_ids.retain(|id| id != client_id);
        }
        
        // 移除空用户
        users.retain(|_, client_ids| !client_ids.is_empty());
    }
}

pub async fn start_server(addr: &str, db: Database) {
    let server = WebSocketServer::new(db);
    server.run(addr).await;
}
