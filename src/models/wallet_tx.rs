use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TxType {
    Recharge,       // 充值
    GiftSend,       // 礼物赠送
    GiftReceive,    // 礼物接收
    Reward,         // 奖励
    PointsEarned,   // 积分获取
    PointsSpent,    // 积分消费
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CurrencyType {
    HP,             // 人类积分 (HumanPoints)
    LC,             // 光币 (LightCoin)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletTx {
    pub id: String,
    pub user_id: String,
    pub tx_type: TxType,
    pub amount: u32,
    pub currency: CurrencyType,
    pub timestamp: i64,
    pub related_entity_id: Option<String>,  // 相关实体ID (如礼物ID、积分商品ID等)
    pub remark: Option<String>,
}

impl WalletTx {
    pub fn new(
        user_id: String, 
        tx_type: TxType, 
        amount: u32, 
        currency: CurrencyType,
        related_entity_id: Option<String>,
        remark: Option<String>
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            tx_type,
            amount,
            currency,
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            related_entity_id,
            remark,
        }
    }
}
