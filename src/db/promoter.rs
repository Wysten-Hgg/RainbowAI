use surrealdb::engine::remote::http::Client;
use surrealdb::Surreal;
use time::OffsetDateTime;

use crate::models::{
    Promoter, PromotionRecord, CommissionLog, WithdrawalRequest, 
    VerificationStatus, CommissionStatus, CommissionType, User, PromoterType
};

use super::surreal::Database;

impl Database {
    // 创建推广者
    pub async fn create_promoter(&self, promoter: &Promoter) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<Promoter>>(("promoter", &promoter.id))
            .content(promoter)
            .await?;
        Ok(())
    }
    
    // 获取推广者信息
    pub async fn get_promoter_by_id(&self, id: &str) -> Result<Option<Promoter>, surrealdb::Error> {
        let promoter: Option<Promoter> = self.client
            .select(("promoter", id))
            .await?;
        Ok(promoter)
    }
    
    // 根据用户ID获取推广者信息
    pub async fn get_promoter_by_user_id(&self, user_id: &str) -> Result<Option<Promoter>, surrealdb::Error> {
        let sql = "SELECT * FROM promoter WHERE user_id = $user_id LIMIT 1";
        let mut response = self.client
            .query(sql)
            .bind(("user_id", user_id))
            .await?;
        let promoters: Option<Vec<Promoter>> = response.take(0)?;
        Ok(promoters.and_then(|mut p| p.pop()))
    }
    
    // 根据邀请码获取推广者信息
    pub async fn get_promoter_by_invite_code(&self, invite_code: &str) -> Result<Option<Promoter>, surrealdb::Error> {
        let sql = "SELECT * FROM promoter WHERE invite_code = $invite_code LIMIT 1";
        let mut response = self.client
            .query(sql)
            .bind(("invite_code", invite_code))
            .await?;
        let promoters: Option<Vec<Promoter>> = response.take(0)?;
        Ok(promoters.and_then(|mut p| p.pop()))
    }
    
    // 更新推广者信息
    pub async fn update_promoter(&self, promoter: &Promoter) -> Result<(), surrealdb::Error> {
        self.client
            .update::<Option<Promoter>>(("promoter", &promoter.id))
            .content(promoter)
            .await?;
        Ok(())
    }
    
    // 更新推广者验证状态
    pub async fn update_promoter_verification(&self, id: &str, status: VerificationStatus) -> Result<(), surrealdb::Error> {
        let mut promoter = self.get_promoter_by_id(id).await?
            .ok_or(surrealdb::Error::Api(surrealdb::error::Api::Query(String::from("Promoter not found"))))?;
        
        promoter.verification_status = status;
        promoter.updated_at = OffsetDateTime::now_utc().unix_timestamp();
        
        self.update_promoter(&promoter).await
    }
    
    // 更新推广者协议签署状态
    pub async fn update_promoter_agreement(&self, id: &str, signed: bool) -> Result<(), surrealdb::Error> {
        let mut promoter = self.get_promoter_by_id(id).await?
            .ok_or(surrealdb::Error::Api(surrealdb::error::Api::Query(String::from("Promoter not found"))))?;
        
        promoter.agreement_signed = signed;
        promoter.updated_at = OffsetDateTime::now_utc().unix_timestamp();
        
        self.update_promoter(&promoter).await
    }
    
    // 更新推广者佣金比例
    pub async fn update_promoter_commission_rates(&self, id: &str, commission_rate: f32, renewal_rate: f32) -> Result<(), surrealdb::Error> {
        let mut promoter = self.get_promoter_by_id(id).await?
            .ok_or(surrealdb::Error::Api(surrealdb::error::Api::Query(String::from("Promoter not found"))))?;
        
        promoter.update_commission_rates(commission_rate, renewal_rate);
        
        self.update_promoter(&promoter).await
    }
    
    // 获取所有推广者
    pub async fn get_all_promoters(&self) -> Result<Vec<Promoter>, surrealdb::Error> {
        let sql = "SELECT * FROM promoter ORDER BY created_at DESC";
        let mut response = self.client.query(sql).await?;
        let promoters: Vec<Promoter> = response.take(0)?;
        Ok(promoters)
    }
    
    // 获取待审核的推广者
    pub async fn get_pending_promoters(&self) -> Result<Vec<Promoter>, surrealdb::Error> {
        let sql = "SELECT * FROM promoter WHERE verification_status = 'Pending' ORDER BY created_at ASC";
        let mut response = self.client.query(sql).await?;
        let promoters: Vec<Promoter> = response.take(0)?;
        Ok(promoters)
    }
    
    // 创建推广记录
    pub async fn create_promotion_record(&self, record: &PromotionRecord) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<PromotionRecord>>(("promotion_record", &record.id))
            .content(record)
            .await?;
        Ok(())
    }
    
    // 获取推广者的推广记录
    pub async fn get_promotion_records_by_promoter(&self, promoter_id: &str, limit: usize) -> Result<Vec<PromotionRecord>, surrealdb::Error> {
        let sql = "SELECT * FROM promotion_record WHERE promoter_id = $promoter_id ORDER BY created_at DESC LIMIT $limit";
        let mut response = self.client
            .query(sql)
            .bind(("promoter_id", promoter_id))
            .bind(("limit", limit))
            .await?;
        let records: Vec<PromotionRecord> = response.take(0)?;
        Ok(records)
    }
    
    // 获取用户的推广记录
    pub async fn get_promotion_records_by_user(&self, user_id: &str) -> Result<Vec<PromotionRecord>, surrealdb::Error> {
        let sql = "SELECT * FROM promotion_record WHERE invited_user_id = $user_id";
        let mut response = self.client
            .query(sql)
            .bind(("user_id", user_id))
            .await?;
        let records: Vec<PromotionRecord> = response.take(0)?;
        Ok(records)
    }
    
    // 创建佣金记录
    pub async fn create_commission_log(&self, log: &CommissionLog) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<CommissionLog>>(("commission_log", &log.id))
            .content(log)
            .await?;
        Ok(())
    }
    
    // 更新佣金记录
    pub async fn update_commission_log(&self, log: &CommissionLog) -> Result<(), surrealdb::Error> {
        self.client
            .update::<Option<CommissionLog>>(("commission_log", &log.id))
            .content(log)
            .await?;
        Ok(())
    }
    
    // 获取推广者的佣金记录
    pub async fn get_commission_logs_by_promoter(&self, promoter_id: &str, limit: usize) -> Result<Vec<CommissionLog>, surrealdb::Error> {
        let sql = "SELECT * FROM commission_log WHERE promoter_id = $promoter_id ORDER BY created_at DESC LIMIT $limit";
        let mut response = self.client
            .query(sql)
            .bind(("promoter_id", promoter_id))
            .bind(("limit", limit))
            .await?;
        let logs: Vec<CommissionLog> = response.take(0)?;
        Ok(logs)
    }
    
    // 获取待结算的佣金记录
    pub async fn get_pending_commission_logs(&self) -> Result<Vec<CommissionLog>, surrealdb::Error> {
        let sql = "SELECT * FROM commission_log WHERE status = 'Pending' ORDER BY created_at ASC";
        let mut response = self.client.query(sql).await?;
        let logs: Vec<CommissionLog> = response.take(0)?;
        Ok(logs)
    }
    
    // 创建提现请求
    pub async fn create_withdrawal_request(&self, request: &WithdrawalRequest) -> Result<(), surrealdb::Error> {
        self.client
            .create::<Option<WithdrawalRequest>>(("withdrawal_request", &request.id))
            .content(request)
            .await?;
        Ok(())
    }
    
    // 更新提现请求
    pub async fn update_withdrawal_request(&self, request: &WithdrawalRequest) -> Result<(), surrealdb::Error> {
        self.client
            .update::<Option<WithdrawalRequest>>(("withdrawal_request", &request.id))
            .content(request)
            .await?;
        Ok(())
    }
    
    // 获取提现请求
    pub async fn get_withdrawal_request_by_id(&self, id: &str) -> Result<Option<WithdrawalRequest>, surrealdb::Error> {
        let request: Option<WithdrawalRequest> = self.client
            .select(("withdrawal_request", id))
            .await?;
        Ok(request)
    }
    
    // 获取推广者的提现请求
    pub async fn get_withdrawal_requests_by_promoter(&self, promoter_id: &str, limit: usize) -> Result<Vec<WithdrawalRequest>, surrealdb::Error> {
        let sql = "SELECT * FROM withdrawal_request WHERE promoter_id = $promoter_id ORDER BY created_at DESC LIMIT $limit";
        let mut response = self.client
            .query(sql)
            .bind(("promoter_id", promoter_id))
            .bind(("limit", limit))
            .await?;
        let requests: Vec<WithdrawalRequest> = response.take(0)?;
        Ok(requests)
    }
    
    // 获取待处理的提现请求
    pub async fn get_pending_withdrawal_requests(&self) -> Result<Vec<WithdrawalRequest>, surrealdb::Error> {
        let sql = "SELECT * FROM withdrawal_request WHERE status = 'Pending' ORDER BY created_at ASC";
        let mut response = self.client.query(sql).await?;
        let requests: Vec<WithdrawalRequest> = response.take(0)?;
        Ok(requests)
    }
    
    // 处理用户注册时的邀请码
    pub async fn process_invite_code_registration(&self, user_id: &str, invite_code: &str) -> Result<bool, surrealdb::Error> {
        // 检查邀请码是否存在
        let promoter = self.get_promoter_by_invite_code(invite_code).await?;
        
        if let Some(promoter) = promoter {
            // 更新用户的invited_by字段
            let sql = "UPDATE user:$user_id SET invited_by = $invite_code";
            self.client
                .query(sql)
                .bind(("user_id", user_id))
                .bind(("invite_code", invite_code))
                .await?;
            
            // 创建推广记录
            let record = PromotionRecord::new(
                promoter.id,
                user_id.to_string(),
                false,
                false,
                0.0,
                0.0
            );
            
            self.create_promotion_record(&record).await?;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    // 处理用户付费时的佣金计算
    pub async fn process_payment_commission(&self, user_id: &str, payment_amount: f32, is_renewal: bool) -> Result<Option<CommissionLog>, surrealdb::Error> {
        // 获取用户信息
        let user = self.get_user_by_id(user_id).await?;
        
        if let Some(user) = user {
            // 检查用户是否通过邀请码注册
            if let Some(invite_code) = user.invited_by {
                // 获取推广者信息
                let promoter = self.get_promoter_by_invite_code(&invite_code).await?;
                
                if let Some(promoter) = promoter {
                    // 检查推广者是否已验证
                    if !promoter.is_verified() {
                        return Ok(None);
                    }
                    
                    // 计算佣金
                    let commission_rate = if is_renewal { promoter.renewal_rate } else { promoter.commission_rate };
                    let commission_amount = payment_amount * commission_rate;
                    
                    // 创建推广记录
                    let record = PromotionRecord::new(
                        promoter.id.clone(),
                        user_id.to_string(),
                        !is_renewal,
                        is_renewal,
                        payment_amount,
                        commission_amount
                    );
                    
                    self.create_promotion_record(&record).await?;
                    
                    // 创建佣金记录
                    let commission_type = if is_renewal { CommissionType::Renewal } else { CommissionType::FirstPayment };
                    let commission_log = CommissionLog::new(
                        promoter.id.clone(),
                        commission_amount,
                        commission_type,
                        "USD".to_string()
                    );
                    
                    self.create_commission_log(&commission_log).await?;
                    
                    // 更新推广者的待结算佣金
                    let mut updated_promoter = promoter;
                    updated_promoter.add_pending_commission(commission_amount);
                    self.update_promoter(&updated_promoter).await?;
                    
                    return Ok(Some(commission_log));
                }
            }
        }
        
        Ok(None)
    }
    
    // 处理佣金结算
    pub async fn settle_commission(&self, promoter_id: &str, amount: f32) -> Result<bool, surrealdb::Error> {
        let mut promoter = self.get_promoter_by_id(promoter_id).await?
            .ok_or(surrealdb::Error::Api(surrealdb::error::Api::Query(String::from("Promoter not found"))))?;
        
        if promoter.settle_commission(amount) {
            self.update_promoter(&promoter).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
