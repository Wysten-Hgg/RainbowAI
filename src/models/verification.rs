use serde::{Serialize, Deserialize};
use rand;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VerificationType {
    Registration,
    PasswordReset,
    EmailChange,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailVerification {
    pub id: String,
    pub email: String,
    pub code: String,
    pub verification_type: VerificationType,
    pub expires_at: i64,
    pub used: bool,
    pub created_at: i64,
}

impl EmailVerification {
    pub fn new(email: String, verification_type: VerificationType) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email,
            // 生成6位数验证码
            code: format!("{:06}", rand::random::<u32>() % 1000000),
            verification_type,
            // 15分钟有效期
            expires_at: now + 15 * 60,
            used: false,
            created_at: now,
        }
    }

    pub fn is_valid(&self) -> bool {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        !self.used && now < self.expires_at
    }
}
