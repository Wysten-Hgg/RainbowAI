use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    pub code: String,
    pub used_by: Vec<String>,      // 使用者的用户ID列表
    pub creator_id: String,        // 创建者ID
    pub usage_limit: u32,         // 本周使用上限
    pub expires_at: i64,          // 过期时间戳
    pub created_at: i64,
    pub updated_at: i64,
}

impl Invite {
    pub fn new(creator_id: String, usage_limit: u32) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        // 设置7天后过期
        let expires_at = now + 7 * 24 * 60 * 60;
        
        Self {
            code: uuid::Uuid::new_v4().to_string(),
            used_by: Vec::new(),
            creator_id,
            usage_limit,
            expires_at,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_be_used(&self) -> bool {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        now < self.expires_at && self.used_by.len() < self.usage_limit as usize
    }
}
