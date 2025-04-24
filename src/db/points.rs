use surrealdb::engine::remote::http::Client;
use surrealdb::Surreal;
use time::OffsetDateTime;

use crate::models::{
    User, WalletTx, TxType, CurrencyType, Gift, GiftRecord, 
    LuckyCard, CardLevel, ShopItem, PurchaseRecord, ShopItemCategory, MonthlyRedemptionStat,
    ConsecutiveGiftRecord, GiftFeedbackTemplate, GiftCategory,
};

use super::surreal::Database;

impl Database {
    // ==================== 用户积分操作 ====================
    
    // 增加用户积分
    pub async fn add_user_hp(&self, user_id: &str, amount: u32, tx_type: TxType, 
                             related_entity_id: Option<String>, remark: Option<String>) 
        -> Result<(), surrealdb::Error> {
        
        // 创建交易记录
        let tx = WalletTx::new(
            user_id.to_string(),
            tx_type,
            amount,
            CurrencyType::HP,
            related_entity_id,
            remark,
        );
        
        // 更新用户积分并添加交易记录
        let result = self
            .client
            .query("
                BEGIN TRANSACTION;
                
                LET $user = SELECT * FROM user:$user_id;
                
                UPDATE user:$user_id SET 
                    hp = $user.hp + $amount,
                    updated_at = $now;
                
                CREATE wallet_tx:$tx_id CONTENT $tx;
                
                COMMIT TRANSACTION;
            ")
            .bind(("user_id", user_id))
            .bind(("amount", amount))
            .bind(("now", OffsetDateTime::now_utc().unix_timestamp()))
            .bind(("tx_id", &tx.id))
            .bind(("tx", &tx))
            .await?;
        
        Ok(())
    }
    
    // 扣减用户积分
    pub async fn deduct_user_hp(&self, user_id: &str, amount: u32, tx_type: TxType, 
                               related_entity_id: Option<String>, remark: Option<String>) 
        -> Result<bool, surrealdb::Error> {
        
        // 创建交易记录
        let tx = WalletTx::new(
            user_id.to_string(),
            tx_type,
            amount,
            CurrencyType::HP,
            related_entity_id,
            remark,
        );
        
        // 检查用户积分是否足够，如果足够则扣减并添加交易记录
        let mut result = self
            .client
            .query("
                BEGIN TRANSACTION;
                
                LET $user = SELECT * FROM user:$user_id;
                
                IF $user.hp >= $amount THEN
                    UPDATE user:$user_id SET 
                        hp = $user.hp - $amount,
                        updated_at = $now;
                    
                    CREATE wallet_tx:$tx_id CONTENT $tx;
                    
                    RETURN true;
                ELSE
                    RETURN false;
                END;
                
                COMMIT TRANSACTION;
            ")
            .bind(("user_id", user_id))
            .bind(("amount", amount))
            .bind(("now", OffsetDateTime::now_utc().unix_timestamp()))
            .bind(("tx_id", &tx.id))
            .bind(("tx", &tx))
            .await?;
        
        // 返回是否扣减成功
        let success: Option<bool> = result.take(0)?;
        Ok(success.unwrap_or(false))
    }
    
    // 获取用户积分交易记录
    pub async fn get_user_hp_transactions(&self, user_id: &str, limit: usize) 
        -> Result<Vec<WalletTx>, surrealdb::Error> {
        
        let mut result = self
            .client
            .query("
                SELECT * FROM wallet_tx 
                WHERE user_id = $user_id AND currency = 'HP'
                ORDER BY timestamp DESC
                LIMIT $limit
            ")
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // ==================== 用户金币操作 ====================
    
    // 增加用户金币
    pub async fn add_user_lc(&self, user_id: &str, amount: u32, tx_type: TxType, 
                             related_entity_id: Option<String>, remark: Option<String>) 
        -> Result<(), surrealdb::Error> {
        
        // 创建交易记录
        let tx = WalletTx::new(
            user_id.to_string(),
            tx_type,
            amount,
            CurrencyType::LC,
            related_entity_id,
            remark,
        );
        
        // 更新用户金币并添加交易记录
        let result = self
            .client
            .query("
                BEGIN TRANSACTION;
                
                LET $user = SELECT * FROM user:$user_id;
                
                UPDATE user:$user_id SET 
                    lc_balance = $user.lc_balance + $amount,
                    updated_at = $now;
                
                CREATE wallet_tx:$tx_id CONTENT $tx;
                
                COMMIT TRANSACTION;
            ")
            .bind(("user_id", user_id))
            .bind(("amount", amount))
            .bind(("now", OffsetDateTime::now_utc().unix_timestamp()))
            .bind(("tx_id", &tx.id))
            .bind(("tx", &tx))
            .await?;
        
        Ok(())
    }
    
    // 扣减用户金币
    pub async fn deduct_user_lc(&self, user_id: &str, amount: u32, tx_type: TxType, 
                               related_entity_id: Option<String>, remark: Option<String>) 
        -> Result<bool, surrealdb::Error> {
        
        // 创建交易记录
        let tx = WalletTx::new(
            user_id.to_string(),
            tx_type,
            amount,
            CurrencyType::LC,
            related_entity_id,
            remark,
        );
        
        // 检查用户金币是否足够，如果足够则扣减并添加交易记录
        let mut result = self
            .client
            .query("
                BEGIN TRANSACTION;
                
                LET $user = SELECT * FROM user:$user_id;
                
                IF $user.lc_balance >= $amount THEN
                    UPDATE user:$user_id SET 
                        lc_balance = $user.lc_balance - $amount,
                        updated_at = $now;
                    
                    CREATE wallet_tx:$tx_id CONTENT $tx;
                    
                    RETURN true;
                ELSE
                    RETURN false;
                END;
                
                COMMIT TRANSACTION;
            ")
            .bind(("user_id", user_id))
            .bind(("amount", amount))
            .bind(("now", OffsetDateTime::now_utc().unix_timestamp()))
            .bind(("tx_id", &tx.id))
            .bind(("tx", &tx))
            .await?;
        
        // 返回是否扣减成功
        let success: Option<bool> = result.take(0)?;
        Ok(success.unwrap_or(false))
    }
    
    // 获取用户金币交易记录
    pub async fn get_user_lc_transactions(&self, user_id: &str, limit: usize) 
        -> Result<Vec<WalletTx>, surrealdb::Error> {
        
        let mut result = self
            .client
            .query("
                SELECT * FROM wallet_tx 
                WHERE user_id = $user_id AND currency = 'LC'
                ORDER BY timestamp DESC
                LIMIT $limit
            ")
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // ==================== 礼物系统操作 ====================
    
    // 创建礼物
    pub async fn create_gift(&self, gift: &Gift) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<Gift>>(("gift", &gift.id))
            .content(gift)
            .await?;
        Ok(())
    }
    
    // 更新礼物
    pub async fn update_gift(&self, gift: &Gift) -> Result<(), surrealdb::Error> {
        self.client
            .update::<Option<Gift>>(("gift", &gift.id))
            .content(gift)
            .await?;
        Ok(())
    }
    
    // 删除礼物
    pub async fn delete_gift(&self, gift_id: &str) -> Result<(), surrealdb::Error> {
        self.client
            .delete::<Option<Gift>>(("gift", gift_id))
            .await?;
        Ok(())
    }
    
    // 获取所有礼物（包括不可用的，管理员用）
    pub async fn get_all_gifts(&self) -> Result<Vec<Gift>, surrealdb::Error> {
        let mut result = self
            .client
            .query("SELECT * FROM gift ORDER BY created_at DESC")
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // 获取所有可用礼物
    pub async fn get_available_gifts(&self) -> Result<Vec<Gift>, surrealdb::Error> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        let mut result = self
            .client
            .query("
                SELECT * FROM gift 
                WHERE 
                    is_active = true AND
                    ((is_limited = false) OR 
                    (is_limited = true AND available_until > $now))
            ")
            .bind(("now", now))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // 获取礼物详情
    pub async fn get_gift_by_id(&self, gift_id: &str) -> Result<Option<Gift>, surrealdb::Error> {
        self.client.select(("gift", gift_id)).await
    }
    
    // 赠送礼物
    pub async fn send_gift(&self, gift_id: &str, sender_id: &str, receiver_ai_id: &str, message: Option<String>) 
        -> Result<bool, surrealdb::Error> {
        
        // 获取礼物信息
        let gift: Option<Gift> = self.client.select(("gift", gift_id)).await?;
        
        if let Some(gift) = gift {
            // 检查礼物是否可用
            let now = OffsetDateTime::now_utc().unix_timestamp();
            if !gift.is_active || (gift.is_limited && gift.available_until.unwrap_or(0) <= now) {
                return Ok(false);
            }
            
            // 创建礼物记录
            let gift_record = GiftRecord::new(
                gift_id.to_string(),
                sender_id.to_string(),
                receiver_ai_id.to_string(),
                message,
            );
            
            // 扣减用户金币并创建礼物记录
            let deduct_result = self.deduct_user_lc(
                sender_id, 
                gift.price_lc, 
                TxType::GiftSend, 
                Some(gift_record.id.clone()),
                Some(format!("赠送礼物: {}", gift.name)),
            ).await?;
            
            if deduct_result {
                // 创建礼物记录
                self.client
                    .create::<Option<GiftRecord>>(("gift_record", &gift_record.id))
                    .content(&gift_record)
                    .await?;
                
                // 更新连续送礼记录
                self.update_consecutive_gift_record(sender_id, receiver_ai_id, &gift).await?;
                
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    // 获取用户赠送的礼物记录
    pub async fn get_user_sent_gifts(&self, user_id: &str, limit: usize) 
        -> Result<Vec<GiftRecord>, surrealdb::Error> {
        
        let mut result = self
            .client
            .query("
                SELECT * FROM gift_record 
                WHERE sender_id = $user_id
                ORDER BY sent_at DESC
                LIMIT $limit
            ")
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // 获取AI收到的礼物记录
    pub async fn get_ai_received_gifts(&self, ai_id: &str, limit: usize) 
        -> Result<Vec<GiftRecord>, surrealdb::Error> {
        
        let mut result = self
            .client
            .query("
                SELECT * FROM gift_record 
                WHERE receiver_ai_id = $ai_id
                ORDER BY sent_at DESC
                LIMIT $limit
            ")
            .bind(("ai_id", ai_id))
            .bind(("limit", limit))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // 更新连续送礼记录
    pub async fn update_consecutive_gift_record(&self, user_id: &str, ai_id: &str, gift: &Gift) 
        -> Result<ConsecutiveGiftRecord, surrealdb::Error> {
        
        // 查询是否存在连续送礼记录
        let mut result = self
            .client
            .query("
                SELECT * FROM consecutive_gift_record 
                WHERE user_id = $user_id AND ai_id = $ai_id
                LIMIT 1
            ")
            .bind(("user_id", user_id))
            .bind(("ai_id", ai_id))
            .await?;
        
        let records: Vec<ConsecutiveGiftRecord> = result.take(0)?;
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let today_start = now - (now % 86400); // 当天0点的时间戳
        
        if let Some(mut record) = records.into_iter().next() {
            // 检查是否是连续的（同一天内多次送礼只算一次）
            let last_gift_day = record.last_gift_date - (record.last_gift_date % 86400);
            
            if last_gift_day < today_start {
                // 不是同一天
                if last_gift_day + 86400 >= today_start {
                    // 是连续的（昨天送过礼物）
                    record.consecutive_days += 1;
                } else {
                    // 不是连续的
                    record.consecutive_days = 1;
                }
                record.last_gift_date = now;
            }
            
            // 更新总计数据
            record.total_gifts_sent += 1;
            record.total_emotional_value += gift.emotional_value;
            
            // 更新记录
            self.client
                .update::<Option<ConsecutiveGiftRecord>>(("consecutive_gift_record", &record.id))
                .content(&record)
                .await?;
            
            Ok(record)
        } else {
            // 创建新记录
            let record = ConsecutiveGiftRecord::new(
                user_id.to_string(),
                ai_id.to_string(),
            );
            
            self.client
                .create::<Option<ConsecutiveGiftRecord>>(("consecutive_gift_record", &record.id))
                .content(&record)
                .await?;
            
            Ok(record)
        }
    }
    
    // 获取用户与AI的连续送礼记录
    pub async fn get_consecutive_gift_record(&self, user_id: &str, ai_id: &str) 
        -> Result<Option<ConsecutiveGiftRecord>, surrealdb::Error> {
        
        let mut result = self
            .client
            .query("
                SELECT * FROM consecutive_gift_record 
                WHERE user_id = $user_id AND ai_id = $ai_id
                LIMIT 1
            ")
            .bind(("user_id", user_id))
            .bind(("ai_id", ai_id))
            .await?;
        
        let records: Vec<ConsecutiveGiftRecord> = result.take(0)?;
        Ok(records.into_iter().next())
    }
    
    // 创建礼物反馈模板
    pub async fn create_gift_feedback_template(&self, template: &GiftFeedbackTemplate) 
        -> Result<(), surrealdb::Error> {
        
        self.client
            .create::<Option<GiftFeedbackTemplate>>(("gift_feedback_template", &template.id))
            .content(template)
            .await?;
        
        Ok(())
    }
    
    // 获取礼物反馈模板
    pub async fn get_gift_feedback_templates(&self, category: &GiftCategory) 
        -> Result<Vec<GiftFeedbackTemplate>, surrealdb::Error> {
        
        let mut result = self
            .client
            .query("
                SELECT * FROM gift_feedback_template 
                WHERE gift_category = $category
            ")
            .bind(("category", category))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // ==================== 幸运卡系统操作 ====================
    
    // 创建幸运卡
    pub async fn create_lucky_card(&self, level: CardLevel, owner_id: &str, issued_by_ai_id: Option<String>) 
        -> Result<LuckyCard, surrealdb::Error> {
        
        let lucky_card = LuckyCard::new(level, owner_id.to_string(), issued_by_ai_id);
        
        self.client
            .create::<Option<LuckyCard>>(("lucky_card", &lucky_card.id))
            .content(&lucky_card)
            .await?;
        
        Ok(lucky_card)
    }
    
    // 获取用户有效的幸运卡
    pub async fn get_user_valid_lucky_cards(&self, user_id: &str) 
        -> Result<Vec<LuckyCard>, surrealdb::Error> {
        
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        let mut result = self
            .client
            .query("
                SELECT * FROM lucky_card 
                WHERE 
                    owner_id = $user_id AND 
                    is_used = false AND 
                    expires_at > $now
                ORDER BY created_at DESC
            ")
            .bind(("user_id", user_id))
            .bind(("now", now))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // 使用幸运卡
    pub async fn use_lucky_card(&self, card_id: &str) 
        -> Result<Option<f32>, surrealdb::Error> {
        
        // 获取幸运卡信息
        let mut card: Option<LuckyCard> = self.client.select(("lucky_card", card_id)).await?;
        
        if let Some(ref mut card) = card {
            // 检查卡片是否有效
            if card.is_valid() {
                // 使用卡片
                if let Ok(multiplier) = card.use_card() {
                    // 更新卡片状态
                    self.client
                        .update::<Option<LuckyCard>>(("lucky_card", card_id))
                        .content(card)
                        .await?;
                    
                    return Ok(Some(multiplier));
                }
            }
        }
        
        Ok(None)
    }
    
    // ==================== 积分商城操作 ====================
    
    // 创建商城商品
    pub async fn create_shop_item(&self, item: &ShopItem) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<ShopItem>>(("shop_item", &item.id))
            .content(item)
            .await?;
        Ok(())
    }
    
    // 获取所有可用商品
    pub async fn get_available_shop_items(&self) -> Result<Vec<ShopItem>, surrealdb::Error> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        let mut result = self
            .client
            .query("
                SELECT * FROM shop_item 
                WHERE 
                    ((is_limited = false) OR 
                    (is_limited = true AND available_until > $now)) AND
                    (stock IS NONE OR stock > 0)
            ")
            .bind(("now", now))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // 购买商品
    pub async fn purchase_shop_item(&self, user_id: &str, item_id: &str) 
        -> Result<bool, surrealdb::Error> {
        
        // 获取商品信息
        let item: Option<ShopItem> = self.client.select(("shop_item", item_id)).await?;
        
        if let Some(item) = item {
            // 检查商品是否可用
            if !item.is_available() {
                return Ok(false);
            }
            
            // 创建购买记录
            let purchase_record = PurchaseRecord::new(
                user_id.to_string(),
                item_id.to_string(),
                item.price_hp,
                None, // 暂不设置过期时间
                None, // 无备注
            );
            
            // 扣减用户积分并创建购买记录
            let deduct_result = self.deduct_user_hp(
                user_id, 
                item.price_hp, 
                TxType::PointsSpent, 
                Some(purchase_record.id.clone()),
                Some(format!("购买商品: {}", item.name)),
            ).await?;
            
            if deduct_result {
                // 创建购买记录
                self.client
                    .create::<Option<PurchaseRecord>>(("purchase_record", &purchase_record.id))
                    .content(&purchase_record)
                    .await?;
                
                // 如果商品有库存限制，则减少库存
                if let Some(stock) = item.stock {
                    if stock > 0 {
                        self.client
                            .query("
                                UPDATE shop_item:$item_id SET 
                                    stock = $stock - 1
                            ")
                            .bind(("item_id", item_id))
                            .bind(("stock", stock))
                            .await?;
                    }
                }
                
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    // 获取用户购买记录
    pub async fn get_user_purchases(&self, user_id: &str, limit: usize) 
        -> Result<Vec<PurchaseRecord>, surrealdb::Error> {
        
        let mut result = self
            .client
            .query("
                SELECT * FROM purchase_record 
                WHERE user_id = $user_id
                ORDER BY purchased_at DESC
                LIMIT $limit
            ")
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // ==================== 积分商城操作扩展 ====================
    
    // 获取所有商品（包括不可见的，用于管理员）
    pub async fn get_all_shop_items(&self) -> Result<Vec<ShopItem>, surrealdb::Error> {
        let mut result = self
            .client
            .query("
                SELECT * FROM shop_item 
                ORDER BY created_at DESC
            ")
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // 根据分类获取可用商品
    pub async fn get_available_shop_items_by_category(&self, category: &ShopItemCategory) 
        -> Result<Vec<ShopItem>, surrealdb::Error> {
        
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let category_str = match category {
            ShopItemCategory::Coupon => "Coupon",
            ShopItemCategory::Decoration => "Decoration",
            ShopItemCategory::Function => "Function",
        };
        
        let mut result = self
            .client
            .query("
                SELECT * FROM shop_item 
                WHERE 
                    category = $category AND
                    visible = true AND
                    ((is_limited = false) OR 
                    (is_limited = true AND available_until > $now)) AND
                    (stock IS NONE OR stock > 0)
                ORDER BY price_hp ASC
            ")
            .bind(("category", category_str))
            .bind(("now", now))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // 更新商品信息
    pub async fn update_shop_item(&self, item: &ShopItem) -> Result<(), surrealdb::Error> {
        let result: Result<Option<ShopItem>, surrealdb::Error> = self.client
            .update(("shop_item", &item.id))
            .content(item)
            .await;
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    
    // 获取单个商品信息
    pub async fn get_shop_item(&self, item_id: &str) -> Result<Option<ShopItem>, surrealdb::Error> {
        self.client
            .select(("shop_item", item_id))
            .await
    }
    
    // 删除商品
    pub async fn delete_shop_item(&self, item_id: &str) -> Result<Option<ShopItem>, surrealdb::Error> {
        self.client
            .delete::<Option<ShopItem>>(("shop_item", item_id))
            .await
    }
    
    // 计算用户商品的折扣价格
    pub async fn calculate_discounted_price(&self, user_id: &str, item_id: &str) 
        -> Result<u32, surrealdb::Error> {
        
        // 获取用户信息
        let user: Option<User> = self.client.select(("user", user_id)).await?;
        
        // 获取商品信息
        let item: Option<ShopItem> = self.client.select(("shop_item", item_id)).await?;
        
        if let (Some(user), Some(item)) = (user, item) {
            // 计算折扣价格
            Ok(item.get_discounted_price(&user.vip_level))
        } else {
            Err(surrealdb::Error::Api(surrealdb::error::Api::Query(String::from("用户或商品不存在"))))
        }
    }
    
    // 兑换商品（扩展版，支持VIP折扣和月度限制）
    pub async fn redeem_shop_item(&self, user_id: &str, item_id: &str, remark: Option<String>) 
        -> Result<bool, surrealdb::Error> {
        
        // 获取商品信息
        let item: Option<ShopItem> = self.client.select(("shop_item", item_id)).await?;
        
        if let Some(item) = item {
            // 检查商品是否可用
            if !item.is_available() {
                return Ok(false);
            }
            
            // 获取用户信息
            let user: Option<User> = self.client.select(("user", user_id)).await?;
            
            if let Some(user) = user {
                // 计算折扣价格
                let price_to_pay = item.get_discounted_price(&user.vip_level);
                
                // 检查月度兑换限制
                if let Some(monthly_limit) = item.monthly_limit {
                    // 获取月度兑换统计
                    let now = OffsetDateTime::now_utc();
                    let year_month = format!("{}-{:02}", now.year(), now.month() as u8);
                    let stat_id = format!("{}:{}", user_id, year_month);
                    
                    let mut stat: Option<MonthlyRedemptionStat> = self.client
                        .select(("monthly_redemption_stat", &stat_id))
                        .await?;
                    
                    if let Some(ref stat) = stat {
                        let item_type_str = format!("{:?}", item.item_type);
                        if !stat.check_monthly_limit(&item_type_str, monthly_limit) {
                            return Ok(false); // 达到月度兑换上限
                        }
                    }
                }
                
                // 创建购买记录
                let purchase_record = PurchaseRecord::new(
                    user_id.to_string(),
                    item_id.to_string(),
                    price_to_pay,
                    None, // 暂不设置过期时间
                    remark,
                );
                
                // 扣减用户积分并创建购买记录
                let deduct_result = self.deduct_user_hp(
                    user_id, 
                    price_to_pay, 
                    TxType::PointsSpent, 
                    Some(purchase_record.id.clone()),
                    Some(format!("兑换商品: {}", item.name)),
                ).await?;
                
                if deduct_result {
                    // 创建购买记录
                    self.client
                        .create::<Option<PurchaseRecord>>(("purchase_record", &purchase_record.id))
                        .content(&purchase_record)
                        .await?;
                    
                    // 更新月度兑换统计
                    let now = OffsetDateTime::now_utc();
                    let year_month = format!("{}-{:02}", now.year(), now.month() as u8);
                    let stat_id = format!("{}:{}", user_id, year_month);
                    
                    let mut stat: Option<MonthlyRedemptionStat> = self.client
                        .select(("monthly_redemption_stat", &stat_id))
                        .await?;
                    
                    if let Some(mut stat) = stat {
                        // 更新现有统计
                        let item_type_str = format!("{:?}", item.item_type);
                        stat.record_redemption(&item_type_str, price_to_pay);
                        
                        self.client
                            .update::<Option<MonthlyRedemptionStat>>(("monthly_redemption_stat", &stat_id))
                            .content(&stat)
                            .await?;
                    } else {
                        // 创建新的统计记录
                        let mut new_stat = MonthlyRedemptionStat::new(user_id.to_string());
                        let item_type_str = format!("{:?}", item.item_type);
                        new_stat.record_redemption(&item_type_str, price_to_pay);
                        
                        self.client
                            .create::<Option<MonthlyRedemptionStat>>(("monthly_redemption_stat", &new_stat.id))
                            .content(&new_stat)
                            .await?;
                    }
                    
                    // 如果商品有库存限制，则减少库存
                    if let Some(stock) = item.stock {
                        if stock > 0 {
                            self.client
                                .query("
                                    UPDATE shop_item:$item_id SET 
                                        stock = $stock - 1
                                ")
                                .bind(("item_id", item_id))
                                .bind(("stock", stock))
                                .await?;
                        }
                    }
                    
                    // 处理卡券类商品
                    if item.category == ShopItemCategory::Coupon {
                        if let Some(coupon_id) = item.linked_coupon_id {
                            // 获取卡券模板信息
                            let coupon_template: Option<crate::models::coupon::CouponTemplate> = 
                                self.client.select(("coupon_template", &coupon_id)).await?;
                            
                            if let Some(template) = coupon_template {
                                // 创建用户卡券
                                let new_coupon = crate::models::coupon::Coupon::new_from_template(
                                    template,
                                    user_id.to_string(),
                                );
                                
                                self.create_coupon(&new_coupon).await?;
                            }
                        }
                    }
                    
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    // 获取用户月度兑换统计
    pub async fn get_user_monthly_redemption_stat(&self, user_id: &str) 
        -> Result<Option<MonthlyRedemptionStat>, surrealdb::Error> {
        
        let now = OffsetDateTime::now_utc();
        let year_month = format!("{}-{:02}", now.year(), now.month() as u8);
        let stat_id = format!("{}:{}", user_id, year_month);
        
        self.client
            .select(("monthly_redemption_stat", &stat_id))
            .await
    }
    
    // 获取用户历史兑换统计
    pub async fn get_user_redemption_history(&self, user_id: &str, limit: usize) 
        -> Result<Vec<MonthlyRedemptionStat>, surrealdb::Error> {
        
        let mut result = self
            .client
            .query("
                SELECT * FROM monthly_redemption_stat 
                WHERE user_id = $user_id
                ORDER BY year_month DESC
                LIMIT $limit
            ")
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?;
        
        Ok(result.take(0)?)
    }
    
    // ==================== 签到系统操作 ====================
    
    // 用户签到
    pub async fn user_daily_checkin(&self, user_id: &str) 
        -> Result<(bool, u32, u32), surrealdb::Error> {
        
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let today_start = now - (now % 86400);
        
        // 获取用户信息
        let user: Option<User> = self.client.select(("user", user_id)).await?;
        
        if let Some(user) = user {
            // 检查今天是否已经签到
            if let Some(last_checkin) = user.last_checkin_date {
                if last_checkin >= today_start {
                    return Ok((false, user.daily_checkin_streak, user.hp));
                }
            }
            
            // 检查是否连续签到
            let is_consecutive = if let Some(last_checkin) = user.last_checkin_date {
                // 如果上次签到是昨天，则连续签到天数+1
                last_checkin >= (today_start - 86400)
            } else {
                // 首次签到
                false
            };
            
            // 计算新的连续签到天数
            let new_streak = if is_consecutive {
                user.daily_checkin_streak + 1
            } else {
                1
            };
            
            // 计算签到奖励积分
            let base_points = 10; // 基础签到积分
            let streak_bonus = match new_streak {
                1..=6 => new_streak as u32, // 1-6天连续签到，每天额外+1
                7 => 10,                    // 7天连续签到，额外+10
                8..=13 => 8,                // 8-13天连续签到，额外+8
                14 => 15,                   // 14天连续签到，额外+15
                15..=29 => 10,              // 15-29天连续签到，额外+10
                30 => 30,                   // 30天连续签到，额外+30
                _ => 15,                    // 30天以上连续签到，额外+15
            };
            
            let total_points = base_points + streak_bonus;
            
            // 更新用户签到信息并增加积分
            let mut result = self
                .client
                .query("
                    BEGIN TRANSACTION;
                    
                    UPDATE user:$user_id SET 
                        daily_checkin_streak = $new_streak,
                        last_checkin_date = $now,
                        hp = hp + $total_points,
                        updated_at = $now;
                    
                    COMMIT TRANSACTION;
                ")
                .bind(("user_id", user_id))
                .bind(("new_streak", new_streak))
                .bind(("now", now))
                .bind(("total_points", total_points))
                .await?;
            
            // 创建积分交易记录
            let tx = WalletTx::new(
                user_id.to_string(),
                TxType::PointsEarned,
                total_points,
                CurrencyType::HP,
                None,
                Some(format!("每日签到奖励 (连续{}天)", new_streak)),
            );
            
            self.client
                .create::<Option<WalletTx>>(("wallet_tx", &tx.id))
                .content(&tx)
                .await?;
            
            return Ok((true, new_streak, user.hp + total_points));
        }
        
        Ok((false, 0, 0))
    }
}
