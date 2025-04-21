use crate::db::Database;
use crate::models::{
    User, WalletTx, TxType, CurrencyType, Gift, GiftRecord, 
    LuckyCard, CardLevel, ShopItem, ShopItemType, PurchaseRecord,
    FrontendUserRole
};
use time::OffsetDateTime;
use rand::Rng;

pub struct PointsService {
    db: Database,
}

impl PointsService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    // ==================== 积分获取逻辑 ====================
    
    // 每日签到获取积分
    pub async fn daily_checkin(&self, user_id: &str) -> Result<(bool, u32, u32), anyhow::Error> {
        let result = self.db.user_daily_checkin(user_id).await?;
        Ok(result)
    }
    
    // 对话互动获取积分
    pub async fn reward_dialogue_points(&self, user_id: &str, message_count: u32) -> Result<u32, anyhow::Error> {
        // 基础规则：每10条消息奖励1点积分，上限为每天30点
        let base_points = message_count / 10;
        let reward_points = if base_points > 30 { 30 } else { base_points };
        
        if reward_points > 0 {
            self.db.add_user_hp(
                user_id, 
                reward_points, 
                TxType::PointsEarned, 
                None, 
                Some(format!("对话互动奖励 ({} 条消息)", message_count))
            ).await?;
        }
        
        Ok(reward_points)
    }
    
    // 邀请注册获取积分
    pub async fn reward_invitation_points(&self, inviter_id: &str, invitee_id: &str) -> Result<u32, anyhow::Error> {
        // 邀请奖励：每成功邀请一人注册，奖励50点积分
        let reward_points = 50;
        
        self.db.add_user_hp(
            inviter_id, 
            reward_points, 
            TxType::Reward, 
            Some(invitee_id.to_string()), 
            Some("邀请新用户奖励".to_string())
        ).await?;
        
        // 更新邀请总数
        let mut result = self.db
            .client
            .query("
                UPDATE user:$user_id SET 
                    total_invites = total_invites + 1,
                    updated_at = $now
            ")
            .bind(("user_id", inviter_id))
            .bind(("now", OffsetDateTime::now_utc().unix_timestamp()))
            .await?;
        
        Ok(reward_points)
    }
    
    // 首次充值奖励
    pub async fn reward_first_payment(&self, user_id: &str, amount: u32) -> Result<u32, anyhow::Error> {
        // 检查是否是首次充值
        let txs = self.db.get_user_lc_transactions(user_id, 1).await?;
        let is_first_payment = txs.iter().all(|tx| tx.tx_type != TxType::Recharge);
        
        if is_first_payment {
            // 首次充值奖励：充值金额的10%转换为积分
            let reward_points = (amount as f32 * 0.1) as u32;
            
            self.db.add_user_hp(
                user_id, 
                reward_points, 
                TxType::Reward, 
                None, 
                Some("首次充值奖励".to_string())
            ).await?;
            
            return Ok(reward_points);
        }
        
        Ok(0)
    }
    
    // ==================== 幸运卡系统 ====================
    
    // 随机发放幸运卡
    pub async fn issue_random_lucky_card(&self, user_id: &str, ai_id: Option<String>) -> Result<Option<LuckyCard>, anyhow::Error> {
        let mut rng = rand::thread_rng();
        
        // 随机决定是否发放幸运卡（20%概率）
        if rng.gen_range(0..100) < 20 {
            // 根据概率分配卡片等级
            let card_level = match rng.gen_range(0..100) {
                0..=4 => CardLevel::A,    // 5% 概率获得A卡
                5..=14 => CardLevel::B,   // 10% 概率获得B卡
                15..=34 => CardLevel::C,  // 20% 概率获得C卡
                35..=64 => CardLevel::D,  // 30% 概率获得D卡
                _ => CardLevel::E,        // 35% 概率获得E卡
            };
            
            let card = self.db.create_lucky_card(card_level, user_id, ai_id).await?;
            return Ok(Some(card));
        }
        
        Ok(None)
    }
    
    // 使用幸运卡
    pub async fn use_lucky_card(&self, user_id: &str, card_id: &str) -> Result<Option<f32>, anyhow::Error> {
        // 检查卡片是否属于该用户
        let cards = self.db.get_user_valid_lucky_cards(user_id).await?;
        let card_belongs_to_user = cards.iter().any(|card| card.id == card_id);
        
        if card_belongs_to_user {
            let multiplier = self.db.use_lucky_card(card_id).await?;
            return Ok(multiplier);
        }
        
        Ok(None)
    }
    
    // ==================== 礼物系统 ====================
    
    // 获取可用礼物列表
    pub async fn get_available_gifts(&self) -> Result<Vec<Gift>, anyhow::Error> {
        let gifts = self.db.get_available_gifts().await?;
        Ok(gifts)
    }
    
    // 赠送礼物
    pub async fn send_gift(&self, gift_id: &str, sender_id: &str, receiver_ai_id: &str, message: Option<String>) 
        -> Result<bool, anyhow::Error> {
        
        let result = self.db.send_gift(gift_id, sender_id, receiver_ai_id, message).await?;
        Ok(result)
    }
    
    // 获取用户赠送的礼物记录
    pub async fn get_user_sent_gifts(&self, user_id: &str, limit: usize) -> Result<Vec<GiftRecord>, anyhow::Error> {
        let records = self.db.get_user_sent_gifts(user_id, limit).await?;
        Ok(records)
    }
    
    // 获取AI收到的礼物记录
    pub async fn get_ai_received_gifts(&self, ai_id: &str, limit: usize) -> Result<Vec<GiftRecord>, anyhow::Error> {
        let records = self.db.get_ai_received_gifts(ai_id, limit).await?;
        Ok(records)
    }
    
    // ==================== 积分商城系统 ====================
    
    // 获取可用商品列表
    pub async fn get_available_shop_items(&self) -> Result<Vec<ShopItem>, anyhow::Error> {
        let items = self.db.get_available_shop_items().await?;
        Ok(items)
    }
    
    // 购买商品
    pub async fn purchase_shop_item(&self, user_id: &str, item_id: &str) -> Result<bool, anyhow::Error> {
        let result = self.db.purchase_shop_item(user_id, item_id).await?;
        Ok(result)
    }
    
    // 获取用户购买记录
    pub async fn get_user_purchases(&self, user_id: &str, limit: usize) -> Result<Vec<PurchaseRecord>, anyhow::Error> {
        let records = self.db.get_user_purchases(user_id, limit).await?;
        Ok(records)
    }
    
    // ==================== 推广者佣金系统 ====================
    
    // 计算推广者佣金
    pub async fn calculate_promoter_commission(&self, promoter_id: &str, amount: u32) -> Result<u32, anyhow::Error> {
        // 获取推广者信息
        let user: Option<User> = self.db.client.select(("user", promoter_id)).await?;
        
        if let Some(user) = user {
            // 检查用户是否是推广者
            let is_promoter = user.frontend_roles.contains(&FrontendUserRole::Promoter);
            
            if is_promoter {
                // 推广者佣金比例：5%
                let commission = (amount as f32 * 0.05) as u32;
                
                // 添加佣金到推广者账户
                self.db.add_user_lc(
                    promoter_id, 
                    commission, 
                    TxType::Reward, 
                    None, 
                    Some("推广者佣金".to_string())
                ).await?;
                
                return Ok(commission);
            }
        }
        
        Ok(0)
    }
    
    // ==================== 钱包操作 ====================
    
    // 充值光币
    pub async fn recharge_lc(&self, user_id: &str, amount: u32) -> Result<(), anyhow::Error> {
        self.db.add_user_lc(
            user_id, 
            amount, 
            TxType::Recharge, 
            None, 
            Some(format!("充值 {} 光币", amount))
        ).await?;
        
        // 检查是否是首次充值，如果是则奖励积分
        self.reward_first_payment(user_id, amount).await?;
        
        Ok(())
    }
    
    // 获取用户钱包信息
    pub async fn get_user_wallet(&self, user_id: &str) -> Result<(u32, u32), anyhow::Error> {
        let user: Option<User> = self.db.client.select(("user", user_id)).await?;
        
        if let Some(user) = user {
            return Ok((user.hp, user.lc_balance));
        }
        
        Err(anyhow::anyhow!("用户不存在"))
    }
    
    // 获取用户钱包交易记录
    pub async fn get_user_wallet_transactions(&self, user_id: &str, currency: Option<CurrencyType>, limit: usize) 
        -> Result<Vec<WalletTx>, anyhow::Error> {
        
        match currency {
            Some(CurrencyType::HP) => {
                let txs = self.db.get_user_hp_transactions(user_id, limit).await?;
                Ok(txs)
            },
            Some(CurrencyType::LC) => {
                let txs = self.db.get_user_lc_transactions(user_id, limit).await?;
                Ok(txs)
            },
            None => {
                // 获取所有交易记录
                let mut result = self.db
                    .client
                    .query("
                        SELECT * FROM wallet_tx 
                        WHERE user_id = $user_id
                        ORDER BY timestamp DESC
                        LIMIT $limit
                    ")
                    .bind(("user_id", user_id))
                    .bind(("limit", limit))
                    .await?;
                
                let txs: Vec<WalletTx> = result.take(0)?;
                Ok(txs)
            }
        }
    }
}
