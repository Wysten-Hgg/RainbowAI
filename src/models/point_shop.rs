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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ShopItemCategory {
    Coupon,             // 卡券类（体验券、折扣券、现金券）
    Decoration,         // 装饰类（AI皮肤、称号、背景）
    Function,           // 功能类（名额、故事解锁、关系改名）
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShopItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ShopItemType,
    pub category: ShopItemCategory,  // 新增：商品分类
    pub price_hp: u32,              // 人类积分价格
    pub image_url: Option<String>,
    pub is_limited: bool,           // 是否限时商品
    pub available_until: Option<i64>, // 限时商品有效期
    pub created_at: i64,
    pub stock: Option<u32>,         // 库存，None表示无限
    pub visible: bool,              // 新增：是否可见
    pub linked_coupon_id: Option<String>, // 新增：关联的卡券ID
    pub monthly_limit: Option<u32>, // 新增：月度兑换上限
    pub vip_discount: Option<bool>, // 新增：是否支持VIP折扣
}

impl ShopItem {
    pub fn new(
        name: String,
        description: String,
        item_type: ShopItemType,
        category: ShopItemCategory,
        price_hp: u32,
        image_url: Option<String>,
        is_limited: bool,
        available_until: Option<i64>,
        stock: Option<u32>,
        visible: bool,
        linked_coupon_id: Option<String>,
        monthly_limit: Option<u32>,
        vip_discount: Option<bool>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            item_type,
            category,
            price_hp,
            image_url,
            is_limited,
            available_until,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            stock,
            visible,
            linked_coupon_id,
            monthly_limit,
            vip_discount,
        }
    }
    
    // 检查商品是否可用
    pub fn is_available(&self) -> bool {
        // 检查是否可见
        if !self.visible {
            return false;
        }
        
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
    
    // 计算VIP用户的折扣价格
    pub fn get_discounted_price(&self, vip_level: &crate::models::VipLevel) -> u32 {
        if self.vip_discount.unwrap_or(false) {
            match vip_level {
                crate::models::VipLevel::Free => self.price_hp,
                crate::models::VipLevel::Pro => (self.price_hp as f32 * 0.9) as u32, // 10% 折扣
                crate::models::VipLevel::Premium => (self.price_hp as f32 * 0.85) as u32, // 15% 折扣
                crate::models::VipLevel::Ultimate => (self.price_hp as f32 * 0.8) as u32, // 20% 折扣
                crate::models::VipLevel::Team => (self.price_hp as f32 * 0.7) as u32, // 30% 折扣
            }
        } else {
            self.price_hp
        }
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
    pub remark: Option<String>,     // 新增：备注信息
}

impl PurchaseRecord {
    pub fn new(
        user_id: String,
        item_id: String,
        price_paid: u32,
        expires_at: Option<i64>,
        remark: Option<String>,
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
            remark,
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

// 新增：月度兑换统计
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonthlyRedemptionStat {
    pub id: String,
    pub user_id: String,
    pub year_month: String,  // 格式：YYYY-MM
    pub item_type_counts: std::collections::HashMap<String, u32>, // 各类型商品兑换次数
    pub total_points_spent: u32,
    pub updated_at: i64,
}

impl MonthlyRedemptionStat {
    pub fn new(user_id: String) -> Self {
        let now = OffsetDateTime::now_utc();
        let year_month = format!("{}-{:02}", now.year(), now.month() as u8);
        
        Self {
            id: format!("{}:{}", user_id, year_month),
            user_id,
            year_month,
            item_type_counts: std::collections::HashMap::new(),
            total_points_spent: 0,
            updated_at: now.unix_timestamp(),
        }
    }
    
    // 记录一次兑换
    pub fn record_redemption(&mut self, item_type: &str, points_spent: u32) {
        let count = self.item_type_counts.entry(item_type.to_string()).or_insert(0);
        *count += 1;
        self.total_points_spent += points_spent;
        self.updated_at = OffsetDateTime::now_utc().unix_timestamp();
    }
    
    // 检查是否达到月度兑换上限
    pub fn check_monthly_limit(&self, item_type: &str, limit: u32) -> bool {
        if let Some(count) = self.item_type_counts.get(item_type) {
            *count < limit
        } else {
            true
        }
    }
}
