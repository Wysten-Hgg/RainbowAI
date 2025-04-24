use serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GiftEffectType {
    Boost,      // 提升AI关系值
    Memory,     // 增强记忆点
    Exclusive,  // 解锁专属内容
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GiftCategory {
    Light,      // 轻礼物
    Medium,     // 中级礼物
    Advanced,   // 高级礼物
    Rare,       // 稀有礼物
    Limited,    // 限定礼物
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gift {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price_lc: u32,               // 光币价格
    pub emotional_value: u32,        // 情感价值
    pub effect_type: GiftEffectType,
    pub category: GiftCategory,      // 礼物分类
    pub image_url: Option<String>,   // 礼物图片URL
    pub created_at: i64,
    pub is_limited: bool,            // 是否限定礼物
    pub available_until: Option<i64>, // 限定礼物的有效期
    pub boost_value: Option<u32>,    // 提升值（针对Boost类型礼物）
    pub is_active: bool,             // 是否激活（可用于管理员控制礼物上下架）
}

impl Gift {
    pub fn new(
        name: String,
        description: Option<String>,
        price_lc: u32,
        emotional_value: u32,
        effect_type: GiftEffectType,
        category: GiftCategory,
        image_url: Option<String>,
        is_limited: bool,
        available_until: Option<i64>,
        boost_value: Option<u32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            price_lc,
            emotional_value,
            effect_type,
            category,
            image_url,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            is_limited,
            available_until,
            boost_value,
            is_active: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GiftRecord {
    pub id: String,
    pub gift_id: String,
    pub sender_id: String,
    pub receiver_ai_id: String,
    pub sent_at: i64,
    pub message: Option<String>,     // 赠送礼物时的留言
}

impl GiftRecord {
    pub fn new(
        gift_id: String,
        sender_id: String,
        receiver_ai_id: String,
        message: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            gift_id,
            sender_id,
            receiver_ai_id,
            sent_at: OffsetDateTime::now_utc().unix_timestamp(),
            message,
        }
    }
}

// 连续送礼记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConsecutiveGiftRecord {
    pub id: String,
    pub user_id: String,
    pub ai_id: String,
    pub consecutive_days: u32,       // 连续送礼天数
    pub last_gift_date: i64,         // 最后一次送礼日期
    pub total_gifts_sent: u32,       // 总共送出的礼物数量
    pub total_emotional_value: u32,  // 总情感价值
}

impl ConsecutiveGiftRecord {
    pub fn new(
        user_id: String,
        ai_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            ai_id,
            consecutive_days: 1,
            last_gift_date: OffsetDateTime::now_utc().unix_timestamp(),
            total_gifts_sent: 1,
            total_emotional_value: 0,
        }
    }
}

// AI礼物反馈模板
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GiftFeedbackTemplate {
    pub id: String,
    pub gift_category: GiftCategory,
    pub feedback_templates: Vec<String>, // 多个反馈模板，随机选择
    pub created_at: i64,
}

impl GiftFeedbackTemplate {
    pub fn new(
        gift_category: GiftCategory,
        feedback_templates: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            gift_category,
            feedback_templates,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
        }
    }
}
