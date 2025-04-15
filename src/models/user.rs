use serde::{Serialize, Deserialize};
use time;
use uuid;
use crate::db::Database;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VipLevel {
    Free,
    Pro,
    Premium,
    Ultimate,
    Team,
}

impl VipLevel {
    pub fn to_string(&self) -> &str {
        match self {
            VipLevel::Free => "Free",
            VipLevel::Pro => "Pro",
            VipLevel::Premium => "Premium",
            VipLevel::Ultimate => "Ultimate",
            VipLevel::Team => "Team",
        }
    }

    pub async fn max_ai_partners(&self, db: &Database) -> Result<u32, surrealdb::Error> {
        let config = db.get_vip_config(self).await?;
        Ok(config.max_ai_partners)
    }

    pub async fn daily_chat_limit(&self, db: &Database) -> Result<u32, surrealdb::Error> {
        let config = db.get_vip_config(self).await?;
        Ok(config.daily_chat_limit)
    }

    pub async fn daily_lio_limit(&self, db: &Database) -> Result<u32, surrealdb::Error> {
        let config = db.get_vip_config(self).await?;
        Ok(config.daily_lio_limit)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VipLevelConfig {
    pub level: VipLevel,
    pub max_ai_partners: u32,
    pub daily_chat_limit: u32,
    pub daily_lio_limit: u32,
}

impl VipLevelConfig {
    pub fn new(level: VipLevel, max_ai_partners: u32, daily_chat_limit: u32, daily_lio_limit: u32) -> Self {
        Self {
            level,
            max_ai_partners,
            daily_chat_limit,
            daily_lio_limit,
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

    pub async fn can_awaken_ai(&self, db: &Database) -> Result<bool, surrealdb::Error> {
        let max_ai_partners = self.vip_level.max_ai_partners(db).await?;
        Ok(self.ai_partner_count < max_ai_partners)
    }

    pub fn is_backend_user(&self) -> bool {
        !self.backend_roles.is_empty()
    }

    pub fn is_frontend_user(&self) -> bool {
        !self.frontend_roles.is_empty()
    }

    pub async fn can_apply_for_promoter(&self, db: &Database) -> Result<bool, surrealdb::Error> {
        let max_ai_partners = self.vip_level.max_ai_partners(db).await?;
        Ok(max_ai_partners > 1)
    }

    pub async fn apply_for_promoter(&mut self, db: &Database, promoter_type: PromoterType) -> Result<(), String> {
        if !(self.can_apply_for_promoter(db).await?)? {
            return Err("用户不是VIP会员，无法申请推广权限".to_string());
        }
        match promoter_type {
            PromoterType::Individual => self.frontend_roles.push(FrontendUserRole::Promoter),
            PromoterType::Organization => self.frontend_roles.push(FrontendUserRole::Promoter), // 可以根据需要区分
        }
        Ok(())
    }

    pub async fn revoke_promoter_if_vip_expired(&mut self, db: &Database) -> Result<(), surrealdb::Error> {
        if !(self.can_apply_for_promoter(db).await?)? {
            self.frontend_roles.retain(|role| *role != FrontendUserRole::Promoter);
        }
        Ok(())
    }

    pub fn has_role(&self, role: BackendUserRole) -> bool {
        self.backend_roles.contains(&role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(BackendUserRole::Admin) || self.has_role(BackendUserRole::SuperAdmin)
    }
}
