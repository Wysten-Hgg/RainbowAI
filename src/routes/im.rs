use axum::{
    Router,
    routing::{get, post, delete},
    extract::{State, Path, Query, Multipart},
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::Database;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::chat::{Message, MessageType, Group, GroupUser, GroupApply, Friend, ChatFile};
use crate::services::FileStorage;
use std::sync::Arc;

// 请求和响应数据结构
#[derive(Deserialize)]
pub struct SendMessageRequest {
    to_contact_id: String,
    content: String,
    #[serde(rename = "type")]
    message_type: String,
    is_group: bool,
    file_id: Option<String>,
    extends: Option<String>,
    at: Option<String>,
}

#[derive(Deserialize)]
pub struct ForwardMessageRequest {
    msg_ids: Vec<String>,
    user_ids: Vec<String>,
    #[serde(rename = "type")]
    forward_type: String,
}

#[derive(Deserialize)]
pub struct GetMessagesRequest {
    chat_id: String,
    page: Option<u32>,
    limit: Option<u32>,
}

#[derive(Deserialize)]
pub struct SetReadRequest {
    chat_id: String,
    is_group: bool,
}

#[derive(Deserialize)]
pub struct ChatActionRequest {
    chat_id: String,
    is_group: bool,
}

// 创建IM路由
pub fn create_im_routes(db: Database, file_storage: Arc<FileStorage>) -> Router<Database> {
    Router::new()
        // 消息相关路由
        .route("/messages/send", post(send_message))
        .route("/messages/forward", post(forward_message))
        .route("/messages/list", get(get_messages))
        .route("/messages/read", post(set_messages_read))
        .route("/messages/:id/revoke", post(revoke_message))
        .route("/messages/:id", delete(delete_message))
        
        // 聊天相关路由
        .route("/chats", get(get_chats))
        .route("/chats/top", post(set_chat_top))
        .route("/chats/notice", post(set_chat_notice))
        .route("/chats/:id", delete(delete_chat))
        
        // 联系人相关路由
        .route("/contacts", get(get_contacts))
        
        // 文件上传
        .route("/files/upload", post(upload_file))
        .route("/files/:id", get(get_file))
        
        .with_state((db, file_storage))
}

// 消息相关处理函数
async fn send_message(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    // 将消息类型字符串转换为枚举
    let message_type = match payload.message_type.as_str() {
        "image" => MessageType::Image,
        "voice" => MessageType::Voice,
        "video" => MessageType::Video,
        "file" => MessageType::File,
        "event" => MessageType::Event,
        "system" => MessageType::System,
        _ => MessageType::Text,
    };
    
    // 创建消息对象
    let message = Message::new(
        auth_user.user_id.clone(),
        payload.to_contact_id.clone(),
        payload.content.clone(),
        message_type,
        payload.is_group,
        payload.file_id,
        payload.extends,
        payload.at,
    );
    
    // 保存消息到数据库
    match db.create_message(&message).await {
        Ok(_) => Ok(Json(message)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn forward_message(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ForwardMessageRequest>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let mut forwarded_messages = Vec::new();
    
    // 获取原始消息
    for msg_id in &payload.msg_ids {
        if let Ok(Some(original_message)) = db.get_message_by_id(msg_id).await {
            // 对每个目标用户转发消息
            for target_id in &payload.user_ids {
                let is_group = target_id.starts_with("group-");
                let to_id = if is_group {
                    target_id.trim_start_matches("group-").to_string()
                } else {
                    target_id.clone()
                };
                
                // 创建新的转发消息
                let forwarded = Message::new(
                    auth_user.user_id.clone(),
                    to_id,
                    original_message.content.clone(),
                    original_message.message_type.clone(),
                    is_group,
                    original_message.file_id.clone(),
                    original_message.extends.clone(),
                    None, // 转发消息不包含@
                );
                
                // 保存转发消息
                if let Ok(_) = db.create_message(&forwarded).await {
                    forwarded_messages.push(forwarded);
                }
            }
        }
    }
    
    if forwarded_messages.is_empty() {
        Err(StatusCode::BAD_REQUEST)
    } else {
        Ok(Json(forwarded_messages))
    }
}

async fn get_messages(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Query(params): Query<GetMessagesRequest>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    
    // 获取聊天记录
    match db.get_chat_messages(&params.chat_id, limit, offset).await {
        Ok(messages) => Ok(Json(messages)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn set_messages_read(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<SetReadRequest>,
) -> Result<StatusCode, StatusCode> {
    // 设置消息为已读
    match db.set_messages_read(&payload.chat_id, &auth_user.user_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn revoke_message(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(message_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // 获取消息
    if let Ok(Some(message)) = db.get_message_by_id(&message_id).await {
        // 检查是否是消息发送者
        if message.from_user != auth_user.user_id {
            return Err(StatusCode::FORBIDDEN);
        }
        
        // 删除消息
        match db.delete_message(&message_id).await {
            Ok(_) => Ok(StatusCode::OK),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_message(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(message_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // 获取消息
    if let Ok(Some(message)) = db.get_message_by_id(&message_id).await {
        // 检查是否是消息发送者或接收者
        if message.from_user != auth_user.user_id && message.to_user != auth_user.user_id {
            return Err(StatusCode::FORBIDDEN);
        }
        
        // 删除消息
        match db.delete_message(&message_id).await {
            Ok(_) => Ok(StatusCode::OK),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// 聊天相关处理函数
async fn get_chats(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // 实际实现中，这里应该获取用户的所有聊天会话
    // 简化起见，返回空数组
    Ok(Json(Vec::new()))
}

async fn set_chat_top(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ChatActionRequest>,
) -> Result<StatusCode, StatusCode> {
    // 实际实现中，这里应该设置聊天置顶
    Ok(StatusCode::OK)
}

async fn set_chat_notice(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ChatActionRequest>,
) -> Result<StatusCode, StatusCode> {
    // 实际实现中，这里应该设置聊天免打扰
    Ok(StatusCode::OK)
}

async fn delete_chat(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(chat_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // 实际实现中，这里应该删除聊天会话
    Ok(StatusCode::OK)
}

// 联系人相关处理函数
async fn get_contacts(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    // 实际实现中，这里应该获取用户的所有联系人
    // 简化起见，返回空数组
    Ok(Json(Vec::new()))
}

// 文件相关处理函数
async fn upload_file(
    State((db, file_storage)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    mut multipart: Multipart,
) -> Result<Json<ChatFile>, StatusCode> {
    use axum::extract::multipart::Field;
    use futures_util::StreamExt;
    use bytes::BytesMut;
    
    let mut file_data = BytesMut::new();
    let mut file_name = String::new();
    let mut content_type = String::new();
    
    // 处理上传的文件
    while let Some(field_result) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let mut field = field_result;
        
        if field.name().unwrap_or_default() == "file" {
            if let Some(name) = field.file_name() {
                file_name = name.to_string();
            }
            
            if let Some(content_type_value) = field.content_type() {
                content_type = content_type_value.to_string();
            }
            
            let mut field_data = BytesMut::new();
            
            while let Some(chunk) = field.chunk().await.map_err(|_| StatusCode::BAD_REQUEST)? {
                field_data.extend_from_slice(&chunk);
            }
            
            file_data = field_data;
        }
    }
    
    if file_data.is_empty() || file_name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // 保存文件
    match file_storage.save_file(&auth_user.user_id, &file_name, &content_type, &file_data).await {
        Ok(file) => {
            // 保存文件记录到数据库
            match db.create_file(&file).await {
                Ok(_) => Ok(Json(file)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_file(
    State((db, file_storage)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(file_id): Path<String>,
) -> Result<Vec<u8>, StatusCode> {
    // 获取文件记录
    if let Ok(Some(file)) = db.get_file_by_id(&file_id).await {
        // 读取文件内容
        match file_storage.get_file(&file.save_path).await {
            Ok(content) => Ok(content),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
