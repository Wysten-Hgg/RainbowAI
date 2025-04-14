use serde::{Serialize, Deserialize};
use time;
use uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VipLevel {
    Free,
    Pro,
    Premium,
    Ultimate,
    Team,
}

impl VipLevel {
    pub fn max_ai_partners(&self) -> u32 {
        match self {
            VipLevel::Free => 1,
            VipLevel::Pro => 3,
            VipLevel::Premium => 5,
            VipLevel::Ultimate => 7,
            VipLevel::Team => 7,
        }
    }

    pub fn daily_chat_limit(&self) -> u32 {
        match self {
            VipLevel::Free => 10,
            VipLevel::Pro => 50,
            VipLevel::Premium => 200,
            VipLevel::Ultimate => u32::MAX,
            VipLevel::Team => u32::MAX,
        }
    }

    pub fn daily_lio_limit(&self) -> u32 {
        match self {
            VipLevel::Free => 0,
            VipLevel::Pro => 50,
            VipLevel::Premium => 100,
            VipLevel::Ultimate => u32::MAX,
            VipLevel::Team => u32::MAX,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendUserRole {
    User,
    Promoter,
    Manager,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BackendUserRole {
    Moderator,
    Admin,
    SuperAdmin,
    Editor,
    Viewer,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PromoterType {
    Individual,
    Organization,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub frontend_roles: Vec<FrontendUserRole>,
    pub backend_roles: Vec<BackendUserRole>,
    pub vip_level: VipLevel,
    pub ai_partner_count: u32,
    pub daily_chat_count: u32,
    pub daily_lio_count: u32,
    pub invite_code: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email,
            password_hash,
            frontend_roles: vec![FrontendUserRole::User],
            backend_roles: vec![],
            vip_level: VipLevel::Free,
            ai_partner_count: 0,
            daily_chat_count: 0,
            daily_lio_count: 0,
            invite_code: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_awaken_ai(&self) -> bool {
        self.ai_partner_count < self.vip_level.max_ai_partners()
    }

    pub fn is_backend_user(&self) -> bool {
        !self.backend_roles.is_empty()
    }

    pub fn is_frontend_user(&self) -> bool {
        !self.frontend_roles.is_empty()
    }

    pub fn can_apply_for_promoter(&self) -> bool {
        matches!(self.vip_level, VipLevel::Pro | VipLevel::Premium | VipLevel::Ultimate | VipLevel::Team)
    }

    pub fn apply_for_promoter(&mut self, promoter_type: PromoterType) -> Result<(), String> {
        if !self.can_apply_for_promoter() {
            return Err("用户不是VIP会员，无法申请推广权限".to_string());
        }
        match promoter_type {
            PromoterType::Individual => self.frontend_roles.push(FrontendUserRole::Promoter),
            PromoterType::Organization => self.frontend_roles.push(FrontendUserRole::Promoter), // 可以根据需要区分
        }
        Ok(())
    }

    pub fn revoke_promoter_if_vip_expired(&mut self) {
        if !self.can_apply_for_promoter() {
            self.frontend_roles.retain(|role| *role != FrontendUserRole::Promoter);
        }
    }
}
