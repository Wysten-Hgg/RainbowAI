use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuditAction {
    UserRegister,
    UserLogin,
    UserUpgrade,
    AIInitiate,
    InviteCreate,
    InviteUse,
    AdminAction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub action: AuditAction,
    pub details: String,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: i64,
}

impl AuditLog {
    pub fn new(
        user_id: String,
        action: AuditAction,
        details: String,
        ip_address: String,
        user_agent: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            action,
            details,
            ip_address,
            user_agent,
            created_at: time::OffsetDateTime::now_utc().unix_timestamp(),
        }
    }
}
