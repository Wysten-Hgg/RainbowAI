use serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CardLevel {
    A,  // 8-9.99倍
    B,  // 6-7.99倍
    C,  // 4-5.99倍
    D,  // 2-3.99倍
    E,  // 1-1.99倍
}

impl CardLevel {
    // 获取倍率范围下限
    pub fn min_multiplier(&self) -> f32 {
        match self {
            CardLevel::A => 8.0,
            CardLevel::B => 6.0,
            CardLevel::C => 4.0,
            CardLevel::D => 2.0,
            CardLevel::E => 1.0,
        }
    }

    // 获取倍率范围上限
    pub fn max_multiplier(&self) -> f32 {
        match self {
            CardLevel::A => 9.99,
            CardLevel::B => 7.99,
            CardLevel::C => 5.99,
            CardLevel::D => 3.99,
            CardLevel::E => 1.99,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LuckyCard {
    pub id: String,
    pub level: CardLevel,
    pub owner_id: String,
    pub multiplier: f32,          // 实际倍率，在范围内随机生成
    pub created_at: i64,
    pub expires_at: i64,          // 有效期2小时
    pub is_used: bool,
    pub used_at: Option<i64>,
    pub issued_by_ai_id: Option<String>,  // 由哪个AI发放
}

impl LuckyCard {
    pub fn new(level: CardLevel, owner_id: String, issued_by_ai_id: Option<String>) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let min = level.min_multiplier();
        let max = level.max_multiplier();
        
        // 在范围内生成随机倍率
        let multiplier = min + (max - min) * rand::random::<f32>();
        
        Self {
            id: Uuid::new_v4().to_string(),
            level,
            owner_id,
            multiplier,
            created_at: now,
            expires_at: now + 2 * 60 * 60,  // 2小时有效期
            is_used: false,
            used_at: None,
            issued_by_ai_id,
        }
    }
    
    // 检查卡片是否有效
    pub fn is_valid(&self) -> bool {
        !self.is_used && self.expires_at > OffsetDateTime::now_utc().unix_timestamp()
    }
    
    // 使用卡片
    pub fn use_card(&mut self) -> Result<f32, &'static str> {
        if self.is_used {
            return Err("卡片已使用");
        }
        
        if self.expires_at <= OffsetDateTime::now_utc().unix_timestamp() {
            return Err("卡片已过期");
        }
        
        self.is_used = true;
        self.used_at = Some(OffsetDateTime::now_utc().unix_timestamp());
        
        Ok(self.multiplier)
    }
}
