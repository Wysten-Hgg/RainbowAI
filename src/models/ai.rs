use serde::{Serialize, Deserialize};
use time;
use uuid;

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
    pub fn is_compatible_with_vip(&self, vip_level: VipLevel) -> bool {
        match self {
            AIType::Companion | AIType::Creative | AIType::Work | AIType::Service => {
                matches!(vip_level, VipLevel::Pro | VipLevel::Premium | VipLevel::Ultimate | VipLevel::Team)
            }
            AIType::Coordination | AIType::Business | AIType::Governance => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ColorSlot {
    Red,     // 伴侣型
    Orange,  // 伴侣型
    Yellow,  // 创造型
    Green,   // 创造型
    Blue,    // 工作型
    Indigo,  // 工作型
    Purple,  // 服务型
}

impl ColorSlot {
    pub fn get_ai_type(&self) -> AIType {
        match self {
            ColorSlot::Red | ColorSlot::Orange => AIType::Companion,
            ColorSlot::Yellow | ColorSlot::Green => AIType::Creative,
            ColorSlot::Blue | ColorSlot::Indigo => AIType::Work,
            ColorSlot::Purple => AIType::Service,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AIStatus {
    Active,
    Inactive,
    Evolving,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AI {
    pub id: String,
    pub name: String,
    pub ai_type: AIType,
    pub color_slot: ColorSlot,
    pub awakener_id: String,
    pub partner_id: Option<String>,
    pub status: AIStatus,
    pub evolution_level: u32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl AI {
    pub fn new(name: String, color_slot: ColorSlot, awakener_id: String) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            ai_type: color_slot.get_ai_type(),
            color_slot,
            awakener_id,
            partner_id: None,
            status: AIStatus::Active,
            evolution_level: 1,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_partner_with(&self, user: &User) -> bool {
        user.ai_partner_count < user.vip_level.max_ai_partners()
            && self.partner_id.is_none()
            && self.status == AIStatus::Active
    }
}

impl User {
    pub fn can_initiate_ai(&self, ai_type: AIType) -> bool {
        ai_type.is_compatible_with_vip(self.vip_level) && self.can_awaken_ai()
    }

    pub fn handle_vip_expiration(&mut self) {
        if self.vip_level == VipLevel::Free {
            self.ai_partner_count = 0;
        }
    }
}
