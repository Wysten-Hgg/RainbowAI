use serde::{Serialize, Deserialize};
use time;
use uuid;
use crate::models::{User, VipLevel};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AIType {
    Companion,    // 伴侣型
    Creative,     // 创造型
    Work,         // 工作型
    Service,      // 服务型
    Coordination, // 协调型
    Business,     // 业务型
    Governance,   // 治理型
}

impl AIType {
    pub fn is_compatible_with_vip(&self, vip_level: &VipLevel) -> bool {
        match self {
            AIType::Companion => true, // 所有等级都可以使用伴侣型AI
            AIType::Creative => matches!(vip_level, VipLevel::Pro | VipLevel::Premium | VipLevel::Ultimate | VipLevel::Team),
            AIType::Work => matches!(vip_level, VipLevel::Premium | VipLevel::Ultimate | VipLevel::Team),
            AIType::Service => matches!(vip_level, VipLevel::Ultimate | VipLevel::Team),
            AIType::Coordination | AIType::Business | AIType::Governance => matches!(vip_level, VipLevel::Team),
        }
    }
    
    pub fn to_string(&self) -> &str {
        match self {
            AIType::Companion => "伴侣型",
            AIType::Creative => "创造型",
            AIType::Work => "工作型",
            AIType::Service => "服务型",
            AIType::Coordination => "协调型",
            AIType::Business => "业务型",
            AIType::Governance => "治理型",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AIStatus {
    Active,
    Inactive,
    Suspended,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AI {
    pub id: String,
    pub name: String,
    pub ai_type: AIType,
    pub user_id: String,
    pub status: AIStatus,
    pub awakened: bool,
    pub awakened_by: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl AI {
    pub fn new(name: String, ai_type: AIType, user_id: String) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            ai_type,
            user_id,
            status: AIStatus::Active,
            awakened: false,
            awakened_by: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn awaken(&mut self, user_id: String) {
        self.awakened = true;
        self.awakened_by = Some(user_id);
    }
}

impl User {
    pub fn can_initiate_ai(&self, ai_type: AIType) -> bool {
        ai_type.is_compatible_with_vip(&self.vip_level) && self.ai_partner_count < self.ai_slots
    }

    pub fn handle_vip_expiration(&mut self) {
        if self.vip_level == VipLevel::Free {
            self.ai_partner_count = 0;
        }
    }
}
