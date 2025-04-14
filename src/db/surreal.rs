use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use time::OffsetDateTime;

use crate::models::{AuditLog, EmailVerification, User, AI, Invite};

pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    pub async fn init() -> Result<Self, surrealdb::Error> {
        let client = Surreal::new::<Client>("127.0.0.1:8000").await?;
        
        // 使用root账户连接
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        // 选择命名空间和数据库
        client.use_ns("rainbow").use_db("rainbow").await?;
        
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
            .create(("user", &user.id))
            .content(user)
            .await?;
        Ok(())
    }

    pub async fn create_ai(&self, ai: &AI) -> Result<(), surrealdb::Error> {
        self.client
            .create(("ai", &ai.id))
            .content(ai)
            .await?;
        Ok(())
    }

    pub async fn create_invite(&self, invite: &Invite) -> Result<(), surrealdb::Error> {
        self.client
            .create(("invite", &invite.code))
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
        self.client
            .update(("invite", &invite.code))
            .content(invite)
            .await?;
        Ok(())
    }

    pub async fn get_user_ais(&self, user_id: &str) -> Result<Vec<AI>, surrealdb::Error> {
        let ais = self
            .client
            .query("SELECT * FROM ai WHERE awakened_by = $user_id")
            .bind(("user_id", user_id))
            .await?;
        Ok(ais.take(0)?)
    }

    pub async fn create_audit_log(&self, log: &AuditLog) -> Result<(), surrealdb::Error> {
        self.client
            .create(("audit_log", &log.id))
            .content(log)
            .await?;
        Ok(())
    }

    pub async fn create_verification(&self, verification: &EmailVerification) -> Result<(), surrealdb::Error> {
        self.client
            .create(("email_verification", &verification.id))
            .content(verification)
            .await?;
        Ok(())
    }

    pub async fn get_verification(&self, email: &str, code: &str) -> Result<Option<EmailVerification>, surrealdb::Error> {
        let mut results = self.client
            .query("SELECT * FROM email_verification WHERE email = $email AND code = $code AND used = false AND expires_at > time::now()")
            .bind(("email", email))
            .bind(("code", code))
            .await?;
        Ok(results.take(0)?)
    }

    pub async fn mark_verification_used(&self, id: &str) -> Result<(), surrealdb::Error> {
        self.client
            .query("UPDATE email_verification SET used = true WHERE id = $id")
            .bind(("id", id))
            .await?;
        Ok(())
    }

    pub async fn get_user_audit_logs(&self, user_id: &str) -> Result<Vec<AuditLog>, surrealdb::Error> {
        let logs = self.client
            .query("SELECT * FROM audit_log WHERE user_id = $user_id ORDER BY created_at DESC LIMIT 100")
            .bind(("user_id", user_id))
            .await?;
        Ok(logs.take(0)?)
    }
}
