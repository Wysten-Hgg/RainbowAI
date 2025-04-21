use serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ShopItemType {
    AIDecoration,       // AI装饰/皮肤
    UserTitle,          // 专属称号
    LIOAccessTicket,    // LIO访问券
    AISlotExpansion,    // AI扩展名额
    ExclusiveStory,     // 限定剧情解锁
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShopItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ShopItemType,
    pub price_hp: u32,              // 人类积分价格
    pub image_url: Option<String>,
    pub is_limited: bool,           // 是否限时商品
    pub available_until: Option<i64>, // 限时商品有效期
    pub created_at: i64,
    pub stock: Option<u32>,         // 库存，None表示无限
}

impl ShopItem {
    pub fn new(
        name: String,
        description: String,
        item_type: ShopItemType,
        price_hp: u32,
        image_url: Option<String>,
        is_limited: bool,
        available_until: Option<i64>,
        stock: Option<u32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            item_type,
            price_hp,
            image_url,
            is_limited,
            available_until,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            stock,
        }
    }
    
    // 检查商品是否可用
    pub fn is_available(&self) -> bool {
        // 检查是否过期
        if self.is_limited {
            if let Some(end_time) = self.available_until {
                if end_time <= OffsetDateTime::now_utc().unix_timestamp() {
                    return false;
                }
            }
        }
        
        // 检查库存
        if let Some(stock) = self.stock {
            if stock == 0 {
                return false;
            }
        }
        
        true
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurchaseRecord {
    pub id: String,
    pub user_id: String,
    pub item_id: String,
    pub price_paid: u32,
    pub purchased_at: i64,
    pub is_activated: bool,
    pub activated_at: Option<i64>,
    pub expires_at: Option<i64>,    // 某些物品可能有使用期限
}

impl PurchaseRecord {
    pub fn new(
        user_id: String,
        item_id: String,
        price_paid: u32,
        expires_at: Option<i64>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            item_id,
            price_paid,
            purchased_at: OffsetDateTime::now_utc().unix_timestamp(),
            is_activated: false,
            activated_at: None,
            expires_at,
        }
    }
    
    // 激活购买的物品
    pub fn activate(&mut self) -> Result<(), &'static str> {
        if self.is_activated {
            return Err("物品已激活");
        }
        
        if let Some(expire_time) = self.expires_at {
            if expire_time <= OffsetDateTime::now_utc().unix_timestamp() {
                return Err("物品已过期");
            }
        }
        
        self.is_activated = true;
        self.activated_at = Some(OffsetDateTime::now_utc().unix_timestamp());
        
        Ok(())
    }
}
