use serde::{Serialize, Deserialize};
use time;
use uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub msg_id: String,
    pub from_user: String,
    pub to_user: String,
    pub content: String,
    pub message_type: MessageType,
    pub is_group: bool,
    pub is_read: bool,
    pub is_last: bool,
    pub chat_identify: String,
    pub file_id: Option<String>,
    pub extends: Option<String>,
    pub at: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageType {
    Text,
    Image,
    Voice,
    Video,
    File,
    Event,
    System,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FriendStatus {
    Applying = 1,  // 申请中
    Normal = 2,    // 正常
    Blacklist = 3, // 黑名单
}

impl Message {
    pub fn new(
        from_user: String,
        to_user: String,
        content: String,
        message_type: MessageType,
        is_group: bool,
        file_id: Option<String>,
        extends: Option<String>,
        at: Option<String>,
    ) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        let msg_id = uuid::Uuid::new_v4().to_string();
        let chat_identify = if is_group {
            format!("group-{}", to_user)
        } else {
            format!("{}-{}", from_user, to_user)
        };
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            msg_id,
            from_user,
            to_user,
            content,
            message_type,
            is_group,
            is_read: false,
            is_last: true,
            chat_identify,
            file_id,
            extends,
            at,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GroupSetting {
    pub manage: i32,   // 0: 仅群主, 1: 管理员可管理
    pub invite: i32,   // 0: 仅群主, 1: 所有人可邀请
    pub nospeak: i32,  // 0: 允许发言, 1: 全员禁言
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub owner_id: String,
    pub avatar: Option<String>,
    pub notice: Option<String>,
    pub setting: GroupSetting,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Group {
    pub fn new(name: String, owner_id: String) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        let group_id = uuid::Uuid::new_v4().to_string();
        
        Self {
            id: group_id.clone(),
            group_id,
            name,
            owner_id,
            avatar: None,
            notice: None,
            setting: GroupSetting {
                manage: 0,
                invite: 1,
                nospeak: 0,
            },
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupUser {
    pub id: String,
    pub group_id: String,
    pub user_id: String,
    pub role: i32,  // 1: 群主, 2: 管理员, 3: 普通成员
    pub invite_id: Option<String>,
    pub created_at: i64,
}

impl GroupUser {
    pub fn new(group_id: String, user_id: String, role: i32, invite_id: Option<String>) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            group_id,
            user_id,
            role,
            invite_id,
            created_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupApply {
    pub id: String,
    pub group_id: String,
    pub user_id: String,
    pub reason: String,
    pub status: i32,  // 0: 待处理, 1: 已同意, 2: 已拒绝
    pub created_at: i64,
}

impl GroupApply {
    pub fn new(group_id: String, user_id: String, reason: String) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            group_id,
            user_id,
            reason,
            status: 0,  // 默认待处理
            created_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Friend {
    pub id: String,
    pub user_id: String,
    pub friend_id: String,
    pub remark: Option<String>,
    pub apply_message: String,
    pub status: i32,  // 1: 申请中, 2: 正常, 3: 黑名单
    pub is_top: bool,
    pub is_notice: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Friend {
    pub fn new(
        user_id: String, 
        friend_id: String, 
        remark: Option<String>, 
        apply_message: String,
        status: i32
    ) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            friend_id,
            remark,
            apply_message,
            status,
            is_top: false,
            is_notice: true,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatFile {
    pub id: String,
    pub user_id: String,
    pub original_name: String,
    pub save_name: String,
    pub save_path: String,
    pub file_ext: String,
    pub file_size: i64,
    pub file_type: String,
    pub created_at: i64,
}

impl ChatFile {
    pub fn new(
        user_id: String,
        original_name: String,
        save_name: String,
        save_path: String,
        file_ext: String,
        file_size: i64,
        file_type: String,
    ) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            original_name,
            save_name,
            save_path,
            file_ext,
            file_size,
            file_type,
            created_at: now,
        }
    }
}
