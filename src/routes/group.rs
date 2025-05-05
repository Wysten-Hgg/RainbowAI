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
use crate::models::chat::{Group, GroupUser, GroupApply, GroupSetting};
use crate::services::FileStorage;
use std::sync::Arc;

// 请求和响应数据结构
#[derive(Deserialize)]
pub struct CreateGroupRequest {
    name: String,
    user_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateGroupRequest {
    name: Option<String>,
    notice: Option<String>,
}

#[derive(Deserialize)]
pub struct AddGroupUserRequest {
    group_id: String,
    user_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct RemoveGroupUserRequest {
    group_id: String,
    user_id: String,
}

#[derive(Deserialize)]
pub struct GroupSettingRequest {
    group_id: String,
    manage: Option<i32>,
    invite: Option<i32>,
    nospeak: Option<i32>,
}

#[derive(Deserialize)]
pub struct ApplyGroupRequest {
    group_id: String,
    reason: String,
}

#[derive(Deserialize)]
pub struct VerifyApplyRequest {
    apply_id: String,
    status: i32, // 1: 同意, 2: 拒绝
}

#[derive(Deserialize)]
pub struct ChangeOwnerRequest {
    group_id: String,
    new_owner_id: String,
}

// 创建群组路由
pub fn create_group_routes(db: Database, file_storage: Arc<FileStorage>) -> Router {
    Router::new()
        .route("/", post(create_group))
        .route("/:id", get(get_group_info))
        .route("/:id", put(update_group))
        .route("/:id", delete(delete_group))
        .route("/:id/users", get(get_group_users))
        .route("/users/add", post(add_group_users))
        .route("/users/remove", post(remove_group_user))
        .route("/setting", post(update_group_setting))
        .route("/apply", post(apply_group))
        .route("/apply/list", get(get_apply_list))
        .route("/apply/verify", post(verify_apply))
        .route("/owner/change", post(change_owner))
        .with_state((db, file_storage))
}

// 群组相关处理函数
async fn create_group(
    State((db, file_storage)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<CreateGroupRequest>,
) -> Result<Json<Group>, StatusCode> {
    // 创建群组
    let group = Group::new(payload.name, auth_user.user_id.clone());
    
    // 保存群组到数据库
    if let Err(_) = db.create_group(&group).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 添加群主
    let group_user = GroupUser::new(
        group.group_id.clone(),
        auth_user.user_id.clone(),
        1, // 群主
        None,
    );
    
    if let Err(_) = db.create_group_user(&group_user).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 添加其他成员
    for user_id in payload.user_ids {
        if user_id != auth_user.user_id {
            let member = GroupUser::new(
                group.group_id.clone(),
                user_id,
                3, // 普通成员
                Some(auth_user.user_id.clone()),
            );
            
            let _ = db.create_group_user(&member).await;
        }
    }
    
    Ok(Json(group))
}

async fn get_group_info(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(group_id): Path<String>,
) -> Result<Json<Group>, StatusCode> {
    // 检查用户是否是群成员
    if let Ok(group_users) = db.get_group_users(&group_id).await {
        let is_member = group_users.iter().any(|gu| gu.user_id == auth_user.user_id);
        
        if !is_member {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 获取群组信息
    match db.get_group_by_id(&group_id).await {
        Ok(Some(group)) => Ok(Json(group)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_group(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(group_id): Path<String>,
    Json(payload): Json<UpdateGroupRequest>,
) -> Result<StatusCode, StatusCode> {
    // 检查用户权限
    if let Ok(group_users) = db.get_group_users(&group_id).await {
        let user_role = group_users.iter()
            .find(|gu| gu.user_id == auth_user.user_id)
            .map(|gu| gu.role)
            .unwrap_or(0);
        
        // 只有群主和管理员可以修改群信息
        if user_role > 2 {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 获取原群组信息
    let mut group = match db.get_group_by_id(&group_id).await {
        Ok(Some(group)) => group,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 更新群组信息
    if let Some(name) = payload.name {
        group.name = name;
    }
    
    if let Some(notice) = payload.notice {
        group.notice = Some(notice);
    }
    
    group.updated_at = time::OffsetDateTime::now_utc().unix_timestamp();
    
    // 保存更新
    match db.update_group(&group).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_group(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(group_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // 检查用户是否是群主
    if let Ok(group_users) = db.get_group_users(&group_id).await {
        let is_owner = group_users.iter()
            .any(|gu| gu.user_id == auth_user.user_id && gu.role == 1);
        
        if !is_owner {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 删除群组
    match db.delete_group(&group_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_group_users(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Path(group_id): Path<String>,
) -> Result<Json<Vec<GroupUser>>, StatusCode> {
    // 检查用户是否是群成员
    if let Ok(group_users) = db.get_group_users(&group_id).await {
        let is_member = group_users.iter().any(|gu| gu.user_id == auth_user.user_id);
        
        if !is_member {
            return Err(StatusCode::FORBIDDEN);
        }
        
        Ok(Json(group_users))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn add_group_users(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<AddGroupUserRequest>,
) -> Result<StatusCode, StatusCode> {
    // 检查用户权限
    if let Ok(group_users) = db.get_group_users(&payload.group_id).await {
        let user_role = group_users.iter()
            .find(|gu| gu.user_id == auth_user.user_id)
            .map(|gu| gu.role)
            .unwrap_or(0);
        
        // 获取群组设置
        let group = match db.get_group_by_id(&payload.group_id).await {
            Ok(Some(group)) => group,
            Ok(None) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
        
        // 检查权限
        if (group.setting.invite == 0 && user_role > 1) || (group.setting.invite == 1 && user_role > 2) {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 添加成员
    for user_id in payload.user_ids {
        if user_id != auth_user.user_id {
            let member = GroupUser::new(
                payload.group_id.clone(),
                user_id,
                3, // 普通成员
                Some(auth_user.user_id.clone()),
            );
            
            let _ = db.create_group_user(&member).await;
        }
    }
    
    Ok(StatusCode::OK)
}

async fn remove_group_user(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<RemoveGroupUserRequest>,
) -> Result<StatusCode, StatusCode> {
    // 检查用户权限
    if let Ok(group_users) = db.get_group_users(&payload.group_id).await {
        let user_role = group_users.iter()
            .find(|gu| gu.user_id == auth_user.user_id)
            .map(|gu| gu.role)
            .unwrap_or(0);
        
        let target_role = group_users.iter()
            .find(|gu| gu.user_id == payload.user_id)
            .map(|gu| gu.role)
            .unwrap_or(0);
        
        // 权限检查：
        // 1. 自己退出群组
        // 2. 群主可以移除任何人
        // 3. 管理员可以移除普通成员
        if !(auth_user.user_id == payload.user_id || 
             user_role == 1 || 
             (user_role == 2 && target_role > 2)) {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 移除成员
    match db.delete_group_user(&payload.group_id, &payload.user_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_group_setting(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<GroupSettingRequest>,
) -> Result<StatusCode, StatusCode> {
    // 检查用户权限
    if let Ok(group_users) = db.get_group_users(&payload.group_id).await {
        let user_role = group_users.iter()
            .find(|gu| gu.user_id == auth_user.user_id)
            .map(|gu| gu.role)
            .unwrap_or(0);
        
        // 只有群主和管理员可以修改群设置
        if user_role > 2 {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 获取原群组信息
    let mut group = match db.get_group_by_id(&payload.group_id).await {
        Ok(Some(group)) => group,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 更新群组设置
    if let Some(manage) = payload.manage {
        group.setting.manage = manage;
    }
    
    if let Some(invite) = payload.invite {
        group.setting.invite = invite;
    }
    
    if let Some(nospeak) = payload.nospeak {
        group.setting.nospeak = nospeak;
    }
    
    group.updated_at = time::OffsetDateTime::now_utc().unix_timestamp();
    
    // 保存更新
    match db.update_group(&group).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn apply_group(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ApplyGroupRequest>,
) -> Result<StatusCode, StatusCode> {
    // 检查用户是否已经是群成员
    if let Ok(group_users) = db.get_group_users(&payload.group_id).await {
        if group_users.iter().any(|gu| gu.user_id == auth_user.user_id) {
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    
    // 创建申请
    let apply = GroupApply::new(
        payload.group_id,
        auth_user.user_id,
        payload.reason,
    );
    
    // 保存申请
    match db.create_group_apply(&apply).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_apply_list(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<Vec<GroupApply>>, StatusCode> {
    // 获取群ID
    let group_id = params.get("group_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // 检查用户权限
    if let Ok(group_users) = db.get_group_users(group_id).await {
        let user_role = group_users.iter()
            .find(|gu| gu.user_id == auth_user.user_id)
            .map(|gu| gu.role)
            .unwrap_or(0);
        
        // 只有群主和管理员可以查看申请列表
        if user_role > 2 {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 获取申请列表
    match db.get_group_applies(group_id).await {
        Ok(applies) => Ok(Json(applies)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn verify_apply(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<VerifyApplyRequest>,
) -> Result<StatusCode, StatusCode> {
    // 获取申请信息
    let apply = match db.get_group_apply_by_id(&payload.apply_id).await {
        Ok(Some(apply)) => apply,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 检查用户权限
    if let Ok(group_users) = db.get_group_users(&apply.group_id).await {
        let user_role = group_users.iter()
            .find(|gu| gu.user_id == auth_user.user_id)
            .map(|gu| gu.role)
            .unwrap_or(0);
        
        // 只有群主和管理员可以处理申请
        if user_role > 2 {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 更新申请状态
    if let Err(_) = db.update_group_apply(&payload.apply_id, payload.status).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 如果同意申请，添加用户到群组
    if payload.status == 1 {
        let group_user = GroupUser::new(
            apply.group_id,
            apply.user_id,
            3, // 普通成员
            Some(auth_user.user_id),
        );
        
        if let Err(_) = db.create_group_user(&group_user).await {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    Ok(StatusCode::OK)
}

async fn change_owner(
    State((db, _)): State<(Database, Arc<FileStorage>)>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ChangeOwnerRequest>,
) -> Result<StatusCode, StatusCode> {
    // 检查用户是否是群主
    if let Ok(group_users) = db.get_group_users(&payload.group_id).await {
        let is_owner = group_users.iter()
            .any(|gu| gu.user_id == auth_user.user_id && gu.role == 1);
        
        if !is_owner {
            return Err(StatusCode::FORBIDDEN);
        }
        
        // 检查新群主是否是群成员
        let new_owner_exists = group_users.iter()
            .any(|gu| gu.user_id == payload.new_owner_id);
        
        if !new_owner_exists {
            return Err(StatusCode::BAD_REQUEST);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 获取原群组信息
    let mut group = match db.get_group_by_id(&payload.group_id).await {
        Ok(Some(group)) => group,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 更新群主
    group.owner_id = payload.new_owner_id.clone();
    group.updated_at = time::OffsetDateTime::now_utc().unix_timestamp();
    
    // 保存更新
    if let Err(_) = db.update_group(&group).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 更新原群主角色
    if let Err(_) = db.update_group_user_role(&payload.group_id, &auth_user.user_id, 3).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // 更新新群主角色
    if let Err(_) = db.update_group_user_role(&payload.group_id, &payload.new_owner_id, 1).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(StatusCode::OK)
}
