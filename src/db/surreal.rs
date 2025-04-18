use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use time::OffsetDateTime;
use std::env;

use crate::models::{AuditLog, EmailVerification, User, AI, Invite, VipLevelConfig, VipLevel};
use crate::models::coupon::Coupon;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
}

impl Database {
    pub async fn init() -> Result<Self, surrealdb::Error> {
        // 从环境变量读取数据库配置
        let db_host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let db_port = env::var("DB_PORT").unwrap_or_else(|_| "8000".to_string());
        let db_username = env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string());
        let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "root".to_string());
        let db_namespace = env::var("DB_NAMESPACE").unwrap_or_else(|_| "rainbow".to_string());
        let db_name = env::var("DB_NAME").unwrap_or_else(|_| "ai".to_string());
        
        // 构建数据库连接URL
        let db_url = format!("http://{}:{}", db_host, db_port);
        
        // 连接数据库
        let client = Surreal::<Client>::new::<Http>(&db_url).await?;
        
        // 使用root账户连接
        client
            .signin(Root {
                username: &db_username,
                password: &db_password,
            })
            .await?;
        
        // 使用命名空间和数据库
        client.use_ns(&db_namespace).use_db(&db_name).await?;
        
        Ok(Self { client })
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, surrealdb::Error> {
        let mut users = self
            .client
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", email))
            .await?;
        
        Ok(users.take(0)?)
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, surrealdb::Error> {
        self.client
            .select(("user", id))
            .await
    }

    pub async fn create_user(&self, user: &User) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<User>>(("user", &user.id))
            .content(user)
            .await?;
        Ok(())
    }

    pub async fn update_user(&self, user: &User) -> Result<(), surrealdb::Error> {
        let result: Result<Option<User>, surrealdb::Error> = self.client
            .update(("user", &user.id))
            .content(user)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn create_ai(&self, ai: &AI) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<AI>>(("ai", &ai.id))
            .content(ai)
            .await?;
        Ok(())
    }

    pub async fn create_invite(&self, invite: &Invite) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<Invite>>(("invite", &invite.code))
            .content(invite)
            .await?;
        Ok(())
    }

    pub async fn get_invite(&self, code: &str) -> Result<Option<Invite>, surrealdb::Error> {
        self.client
            .select(("invite", code))
            .await
    }

    pub async fn update_invite(&self, invite: &Invite) -> Result<(), surrealdb::Error> {
        let result: Result<Option<Invite>, surrealdb::Error> = self.client
            .update(("invite", &invite.code))
            .content(invite)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_user_ais(&self, user_id: &str) -> Result<Vec<AI>, surrealdb::Error> {
        let mut ais = self
            .client
            .query("SELECT * FROM ai WHERE user_id = $user_id")
            .bind(("user_id", user_id))
            .await?;
        Ok(ais.take(0)?)
    }

    pub async fn create_audit_log(&self, log: &AuditLog) -> Result<(), surrealdb::Error> {
        let result: Result<Option<AuditLog>, surrealdb::Error> = self.client
            .create(("audit_log", &log.id))
            .content(log)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn create_verification(&self, verification: &EmailVerification) -> Result<(), surrealdb::Error> {
        let result: Result<Option<EmailVerification>, surrealdb::Error> = self.client
            .create(("email_verification", &verification.id))
            .content(verification)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_verification(&self, email: &str, code: &str) -> Result<Option<EmailVerification>, surrealdb::Error> {
        let mut results = self.client
            .query("SELECT * FROM email_verification WHERE email = $email AND code = $code AND used = false AND expires_at > time::now()")
            .bind(("email", email))
            .bind(("code", code))
            .await?;
        Ok(results.take(0)?)
    }

    pub async fn get_verification_by_id(&self, id: &str) -> Result<Option<EmailVerification>, surrealdb::Error> {
        self.client
            .select(("email_verification", id))
            .await
    }

    pub async fn mark_verification_used(&self, id: &str) -> Result<(), surrealdb::Error> {
        let result = self.client
            .query("UPDATE email_verification:$id SET used = true")
            .bind(("id", id))
            .await?;
        
        Ok(())
    }

    pub async fn get_user_audit_logs(&self, user_id: &str) -> Result<Vec<AuditLog>, surrealdb::Error> {
        let mut logs = self.client
            .query("SELECT * FROM audit_log WHERE user_id = $user_id ORDER BY created_at DESC LIMIT 100")
            .bind(("user_id", user_id))
            .await?;
        Ok(logs.take(0)?)
    }

    pub async fn create_coupon(&self, coupon: &Coupon) -> Result<(), surrealdb::Error> {
        let result: Result<Option<Coupon>, surrealdb::Error> = self.client
            .create(("coupon", &coupon.id))
            .content(coupon)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_coupon(&self, id: &str) -> Result<Option<Coupon>, surrealdb::Error> {
        self.client
            .select(("coupon", id))
            .await
    }

    pub async fn update_coupon(&self, coupon: &Coupon) -> Result<(), surrealdb::Error> {
        let result: Result<Option<Coupon>, surrealdb::Error> = self.client
            .update(("coupon", &coupon.id))
            .content(coupon)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_user_coupons(&self, user_id: &str) -> Result<Vec<Coupon>, surrealdb::Error> {
        let mut coupons = self
            .client
            .query("SELECT * FROM coupon WHERE owner_id = $user_id AND status = 'active'")
            .bind(("user_id", user_id))
            .await?;
        Ok(coupons.take(0)?)
    }

    pub async fn get_vip_config(&self, vip_level: &VipLevel) -> Result<VipLevelConfig, surrealdb::Error> {
        let mut config = self.client
            .query("SELECT * FROM vip_config WHERE level = $level")
            .bind(("level", vip_level.to_string()))
            .await?;
        Ok(config.take::<Option<VipLevelConfig>>(0)?.unwrap_or_else(|| {
            // 如果没有找到配置，返回默认配置
            VipLevelConfig::new(
                vip_level.clone(),
                match vip_level {
                    VipLevel::Free => 1,
                    VipLevel::Pro => 2,
                    VipLevel::Premium => 3,
                    VipLevel::Ultimate => 5,
                    VipLevel::Team => 10,
                },
                match vip_level {
                    VipLevel::Free => 50,
                    VipLevel::Pro => 100,
                    VipLevel::Premium => 200,
                    VipLevel::Ultimate => 500,
                    VipLevel::Team => 1000,
                },
                match vip_level {
                    VipLevel::Free => 10,
                    VipLevel::Pro => 20,
                    VipLevel::Premium => 50,
                    VipLevel::Ultimate => 100,
                    VipLevel::Team => 200,
                }
            )
        }))
    }

    pub async fn set_vip_config(&self, config: &VipLevelConfig) -> Result<(), surrealdb::Error> {
        self.client
            .update::<Option<VipLevelConfig>>(("vip_config", config.level.to_string()))
            .content(config)
            .await?;
        Ok(())
    }
}
