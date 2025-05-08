use axum::{
    Router,
    routing::{get, post, delete},
    extract::{State, Path, Query},
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::Database;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::chat::{Friend, FriendStatus};
use crate::services::FileStorage;
use std::sync::Arc;

// 请求和响应数据结构
#[derive(Deserialize)]
pub struct AddFriendRequest {
    user_id: String,
    remark: Option<String>,
    apply_message: String,
}

#[derive(Deserialize)]
pub struct VerifyFriendRequest {
    apply_id: String,
    status: i32, // 1: 同意, 2: 拒绝
    remark: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateFriendRequest {
    friend_id: String,
    remark: String,
}

#[derive(Deserialize)]
pub struct BlacklistRequest {
    friend_id: String,
    is_blacklist: bool,
}

// 创建好友路由
pub fn create_friend_routes(db: Database, file_storage: Arc<FileStorage>) -> Router<Database> {
    Router::new()
        .route("/list", get(get_friends))
        .route("/apply", post(apply_friend))
        .route("/apply/list", get(get_apply_list))
        .route("/apply/verify", post(verify_friend))
        .route("/update", post(update_friend))
        .route("/blacklist", post(set_blacklist))
        .route("/:id", delete(delete_friend))
        .with_state((db, file_storage))
}

// 好友相关处理函数
async fn get_friends(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<Friend>>, StatusCode> {
    // 获取好友列表
    match db.get_friends(&auth_user.user_id).await {
        Ok(friends) => Ok(Json(friends)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn apply_friend(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<AddFriendRequest>,
) -> Result<StatusCode, StatusCode> {
    // 检查是否已经是好友
    if let Ok(friends) = db.get_friends(&auth_user.user_id).await {
        if friends.iter().any(|f| f.friend_id == payload.user_id && f.status == FriendStatus::Normal as i32) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    
    // 检查是否已经有申请
    if let Ok(applies) = db.get_friend_applies(&auth_user.user_id).await {
        if applies.iter().any(|f| f.friend_id == payload.user_id && f.status == FriendStatus::Applying as i32) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    
    // 创建好友申请
    let friend = Friend::new(
        auth_user.user_id.clone(),
        payload.user_id,
        payload.remark,
        payload.apply_message,
        FriendStatus::Applying as i32,
    );
    
    // 保存申请
    match db.create_friend(&friend).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_apply_list(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<Friend>>, StatusCode> {
    // 获取好友申请列表
    match db.get_friend_applies(&auth_user.user_id).await {
        Ok(applies) => Ok(Json(applies)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn verify_friend(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<VerifyFriendRequest>,
) -> Result<StatusCode, StatusCode> {
    // 获取申请信息
    let apply = match db.get_friend_by_id(&payload.apply_id).await {
        Ok(Some(apply)) => apply,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 检查是否是申请的接收者
    if apply.friend_id != auth_user.user_id {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 更新申请状态
    if let Err(_) = db.update_friend_status(&payload.apply_id, payload.status).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 如果同意申请，创建双向好友关系
    if payload.status == FriendStatus::Normal as i32 {
        // 更新申请人的好友记录
        if let Err(_) = db.update_friend_remark(&payload.apply_id, payload.remark.clone()).await {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        
        // 创建接收者的好友记录
        let reverse_friend = Friend::new(
            auth_user.user_id.clone(),
            apply.user_id,
            payload.remark,
            String::new(),
            FriendStatus::Normal as i32,
        );
        
        if let Err(_) = db.create_friend(&reverse_friend).await {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    Ok(StatusCode::OK)
}

async fn update_friend(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UpdateFriendRequest>,
) -> Result<StatusCode, StatusCode> {
    // 获取好友关系
    let friend = match db.get_friend(&auth_user.user_id, &payload.friend_id).await {
        Ok(Some(friend)) => friend,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 更新备注
    match db.update_friend_remark(&friend.id, Some(payload.remark)).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn set_blacklist(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<BlacklistRequest>,
) -> Result<StatusCode, StatusCode> {
    // 获取好友关系
    let friend = match db.get_friend(&auth_user.user_id, &payload.friend_id).await {
        Ok(Some(friend)) => friend,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 更新黑名单状态
    let new_status = if payload.is_blacklist {
        FriendStatus::Blacklist as i32
    } else {
        FriendStatus::Normal as i32
    };
    
    match db.update_friend_status(&friend.id, new_status).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_friend(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(friend_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // 获取好友关系
    let friend = match db.get_friend(&auth_user.user_id, &friend_id).await {
        Ok(Some(friend)) => friend,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 删除好友关系
    if let Err(_) = db.delete_friend(&friend.id).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 删除对方的好友关系
    if let Ok(Some(reverse_friend)) = db.get_friend(&friend_id, &auth_user.user_id).await {
        let _ = db.delete_friend(&reverse_friend.id).await;
    }
    
    Ok(StatusCode::OK)
}
