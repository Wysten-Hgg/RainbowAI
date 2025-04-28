use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::models::user::PromoterType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VerificationStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommissionStatus {
    Pending,
    Paid,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommissionType {
    FirstPayment,
    Renewal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Promoter {
    pub id: String,
    pub user_id: String,
    pub promoter_type: PromoterType,
    pub invite_code: String,
    pub commission_rate: f32,        // 首次付费佣金比例
    pub renewal_rate: f32,           // 续费佣金比例
    pub total_commission: f32,       // 总佣金
    pub pending_commission: f32,     // 待结算佣金
    pub wallet_account: String,      // 收款账户
    pub verification_status: VerificationStatus,
    pub id_document: Option<String>, // 身份证明文档路径
    pub agreement_signed: bool,      // 是否已签署协议
    pub created_at: i64,
    pub updated_at: i64,
}

impl Promoter {
    pub fn new(user_id: String, promoter_type: PromoterType, wallet_account: String) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        // 根据推广者类型设置不同的佣金比例
        let (commission_rate, renewal_rate) = match promoter_type {
            PromoterType::Individual => (0.08, 0.05),  // 个人推广者：8% 首付，5% 续费
            PromoterType::Organization => (0.15, 0.10), // 机构推广者：15% 首付，10% 续费
        };
        
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            promoter_type,
            invite_code: Uuid::new_v4().to_string(),
            commission_rate,
            renewal_rate,
            total_commission: 0.0,
            pending_commission: 0.0,
            wallet_account,
            verification_status: VerificationStatus::Pending,
            id_document: None,
            agreement_signed: false,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn is_verified(&self) -> bool {
        matches!(self.verification_status, VerificationStatus::Approved)
    }
    
    pub fn can_receive_commission(&self) -> bool {
        self.is_verified() && self.agreement_signed
    }
    
    pub fn update_commission_rates(&mut self, commission_rate: f32, renewal_rate: f32) {
        self.commission_rate = commission_rate;
        self.renewal_rate = renewal_rate;
        self.updated_at = OffsetDateTime::now_utc().unix_timestamp();
    }
    
    pub fn add_pending_commission(&mut self, amount: f32) {
        self.pending_commission += amount;
        self.updated_at = OffsetDateTime::now_utc().unix_timestamp();
    }
    
    pub fn settle_commission(&mut self, amount: f32) -> bool {
        if amount <= self.pending_commission {
            self.pending_commission -= amount;
            self.total_commission += amount;
            self.updated_at = OffsetDateTime::now_utc().unix_timestamp();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromotionRecord {
    pub id: String,
    pub promoter_id: String,
    pub invited_user_id: String,
    pub first_payment: bool,         // 是否首次付费
    pub renewal_payment: bool,       // 是否续费
    pub payment_amount: f32,         // 支付金额
    pub commission_amount: f32,      // 佣金金额
    pub created_at: i64,
}

impl PromotionRecord {
    pub fn new(
        promoter_id: String, 
        invited_user_id: String, 
        first_payment: bool,
        renewal_payment: bool,
        payment_amount: f32,
        commission_amount: f32
    ) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            id: Uuid::new_v4().to_string(),
            promoter_id,
            invited_user_id,
            first_payment,
            renewal_payment,
            payment_amount,
            commission_amount,
            created_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommissionLog {
    pub id: String,
    pub promoter_id: String,
    pub amount: f32,
    pub commission_type: CommissionType,
    pub currency: String,
    pub status: CommissionStatus,
    pub transaction_id: Option<String>,  // 外部支付系统的交易ID
    pub created_at: i64,
    pub updated_at: i64,
}

impl CommissionLog {
    pub fn new(
        promoter_id: String,
        amount: f32,
        commission_type: CommissionType,
        currency: String
    ) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            id: Uuid::new_v4().to_string(),
            promoter_id,
            amount,
            commission_type,
            currency,
            status: CommissionStatus::Pending,
            transaction_id: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn mark_as_paid(&mut self, transaction_id: String) {
        self.status = CommissionStatus::Paid;
        self.transaction_id = Some(transaction_id);
        self.updated_at = OffsetDateTime::now_utc().unix_timestamp();
    }
    
    pub fn cancel(&mut self) {
        self.status = CommissionStatus::Cancelled;
        self.updated_at = OffsetDateTime::now_utc().unix_timestamp();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawalRequest {
    pub id: String,
    pub promoter_id: String,
    pub amount: f32,
    pub currency: String,
    pub payment_method: String,  // PayPal, Stripe, Bank, etc.
    pub account_info: String,    // 收款账户信息
    pub status: CommissionStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

impl WithdrawalRequest {
    pub fn new(
        promoter_id: String,
        amount: f32,
        currency: String,
        payment_method: String,
        account_info: String
    ) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            id: Uuid::new_v4().to_string(),
            promoter_id,
            amount,
            currency,
            payment_method,
            account_info,
            status: CommissionStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn approve(&mut self) {
        self.status = CommissionStatus::Paid;
        self.updated_at = OffsetDateTime::now_utc().unix_timestamp();
    }
    
    pub fn reject(&mut self) {
        self.status = CommissionStatus::Cancelled;
        self.updated_at = OffsetDateTime::now_utc().unix_timestamp();
    }
}
