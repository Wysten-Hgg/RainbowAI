use crate::db::Database;
use crate::models::{
    User, Promoter, PromotionRecord, CommissionLog, WithdrawalRequest,
    VerificationStatus, CommissionStatus, CommissionType, PromoterType
};
use anyhow::{Result, anyhow};
use time::OffsetDateTime;

pub struct PromoterService {
    db: Database,
}

impl PromoterService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    // 申请成为推广者
    pub async fn apply_for_promoter(
        &self, 
        user_id: &str, 
        promoter_type: PromoterType, 
        wallet_account: String
    ) -> Result<Promoter> {
        // 检查用户是否存在
        let user = self.db.get_user_by_id(user_id).await?
            .ok_or_else(|| anyhow!("User not found"))?;
        
        // 检查用户是否有资格成为推广者
        if !user.can_apply_for_promoter(&self.db).await? {
            return Err(anyhow!("User does not meet the requirements to become a promoter"));
        }
        
        // 检查用户是否已经是推广者
        if let Some(_) = self.db.get_promoter_by_user_id(user_id).await? {
            return Err(anyhow!("User is already a promoter"));
        }
        
        // 创建推广者
        let promoter = Promoter::new(
            user_id.to_string(),
            promoter_type.clone(),
            wallet_account
        );
        
        // 保存推广者信息
        self.db.create_promoter(&promoter).await?;
        
        // 更新用户角色
        let mut updated_user = user;
        updated_user.apply_for_promoter(&self.db, promoter_type).await?;
        self.db.update_user(&updated_user).await?;
        
        Ok(promoter)
    }
    
    // 上传身份证明文档
    pub async fn upload_id_document(&self, promoter_id: &str, document_path: String) -> Result<()> {
        let mut promoter = self.db.get_promoter_by_id(promoter_id).await?
            .ok_or_else(|| anyhow!("Promoter not found"))?;
        
        promoter.id_document = Some(document_path);
        promoter.updated_at = OffsetDateTime::now_utc().unix_timestamp();
        
        self.db.update_promoter(&promoter).await?;
        
        Ok(())
    }
    
    // 签署推广协议
    pub async fn sign_agreement(&self, promoter_id: &str) -> Result<()> {
        self.db.update_promoter_agreement(promoter_id, true).await?;
        Ok(())
    }
    
    // 审核推广者申请
    pub async fn review_promoter_application(
        &self, 
        promoter_id: &str, 
        approved: bool, 
        admin_id: &str
    ) -> Result<()> {
        // 检查管理员权限
        let admin = self.db.get_user_by_id(admin_id).await?
            .ok_or_else(|| anyhow!("Admin not found"))?;
        
        if !admin.is_admin() {
            return Err(anyhow!("User does not have admin privileges"));
        }
        
        let status = if approved {
            VerificationStatus::Approved
        } else {
            VerificationStatus::Rejected
        };
        
        self.db.update_promoter_verification(promoter_id, status).await?;
        
        Ok(())
    }
    
    // 获取推广者信息
    pub async fn get_promoter(&self, promoter_id: &str) -> Result<Promoter> {
        let promoter = self.db.get_promoter_by_id(promoter_id).await?
            .ok_or_else(|| anyhow!("Promoter not found"))?;
        
        Ok(promoter)
    }
    
    // 更新推广者信息
    pub async fn update_promoter(&self, promoter: &Promoter) -> Result<()> {
        self.db.update_promoter(promoter).await?;
        Ok(())
    }
    
    // 获取用户的推广者信息
    pub async fn get_promoter_by_user(&self, user_id: &str) -> Result<Option<Promoter>> {
        let promoter = self.db.get_promoter_by_user_id(user_id).await?;
        Ok(promoter)
    }
    
    // 获取所有推广者
    pub async fn get_all_promoters(&self, admin_id: &str) -> Result<Vec<Promoter>> {
        // 检查管理员权限
        let admin = self.db.get_user_by_id(admin_id).await?
            .ok_or_else(|| anyhow!("Admin not found"))?;
        
        if !admin.is_admin() {
            return Err(anyhow!("User does not have admin privileges"));
        }
        
        let promoters = self.db.get_all_promoters().await?;
        Ok(promoters)
    }
    
    // 获取待审核的推广者
    pub async fn get_pending_promoters(&self, admin_id: &str) -> Result<Vec<Promoter>> {
        // 检查管理员权限
        let admin = self.db.get_user_by_id(admin_id).await?
            .ok_or_else(|| anyhow!("Admin not found"))?;
        
        if !admin.is_admin() {
            return Err(anyhow!("User does not have admin privileges"));
        }
        
        let promoters = self.db.get_pending_promoters().await?;
        Ok(promoters)
    }
    
    // 更新推广者佣金比例
    pub async fn update_commission_rates(
        &self, 
        promoter_id: &str, 
        commission_rate: f32, 
        renewal_rate: f32, 
        admin_id: &str
    ) -> Result<()> {
        // 检查管理员权限
        let admin = self.db.get_user_by_id(admin_id).await?
            .ok_or_else(|| anyhow!("Admin not found"))?;
        
        if !admin.is_admin() {
            return Err(anyhow!("User does not have admin privileges"));
        }
        
        self.db.update_promoter_commission_rates(promoter_id, commission_rate, renewal_rate).await?;
        
        Ok(())
    }
    
    // 获取推广记录
    pub async fn get_promotion_records(&self, promoter_id: &str, limit: usize) -> Result<Vec<PromotionRecord>> {
        let records = self.db.get_promotion_records_by_promoter(promoter_id, limit).await?;
        Ok(records)
    }
    
    // 获取推广统计
    pub async fn get_promotion_statistics(&self, promoter_id: &str) -> Result<PromotionStatistics> {
        let promoter = self.db.get_promoter_by_id(promoter_id).await?
            .ok_or_else(|| anyhow!("Promoter not found"))?;
        
        let records = self.db.get_promotion_records_by_promoter(promoter_id, 1000).await?;
        
        let total_invited = records.len();
        let first_payments = records.iter().filter(|r| r.first_payment).count();
        let renewal_payments = records.iter().filter(|r| r.renewal_payment).count();
        let total_commission = promoter.total_commission;
        let pending_commission = promoter.pending_commission;
        
        let statistics = PromotionStatistics {
            total_invited,
            first_payments,
            renewal_payments,
            total_commission,
            pending_commission,
        };
        
        Ok(statistics)
    }
    
    // 获取佣金记录
    pub async fn get_commission_logs(&self, promoter_id: &str, limit: usize) -> Result<Vec<CommissionLog>> {
        let logs = self.db.get_commission_logs_by_promoter(promoter_id, limit).await?;
        Ok(logs)
    }
    
    // 申请提现
    pub async fn request_withdrawal(
        &self, 
        promoter_id: &str, 
        amount: f32, 
        currency: String, 
        payment_method: String, 
        account_info: String
    ) -> Result<WithdrawalRequest> {
        let promoter = self.db.get_promoter_by_id(promoter_id).await?
            .ok_or_else(|| anyhow!("Promoter not found"))?;
        
        // 检查推广者是否已验证
        if !promoter.is_verified() {
            return Err(anyhow!("Promoter is not verified"));
        }
        
        // 检查推广者是否已签署协议
        if !promoter.agreement_signed {
            return Err(anyhow!("Promoter has not signed the agreement"));
        }
        
        // 检查提现金额是否超过待结算佣金
        if amount > promoter.pending_commission {
            return Err(anyhow!("Withdrawal amount exceeds pending commission"));
        }
        
        // 创建提现请求
        let request = WithdrawalRequest::new(
            promoter_id.to_string(),
            amount,
            currency,
            payment_method,
            account_info
        );
        
        // 保存提现请求
        self.db.create_withdrawal_request(&request).await?;
        
        Ok(request)
    }
    
    // 获取提现请求
    pub async fn get_withdrawal_requests(&self, promoter_id: &str, limit: usize) -> Result<Vec<WithdrawalRequest>> {
        let requests = self.db.get_withdrawal_requests_by_promoter(promoter_id, limit).await?;
        Ok(requests)
    }
    
    // 处理提现请求
    pub async fn process_withdrawal_request(
        &self, 
        request_id: &str, 
        approved: bool, 
        transaction_id: Option<String>, 
        admin_id: &str
    ) -> Result<()> {
        // 检查管理员权限
        let admin = self.db.get_user_by_id(admin_id).await?
            .ok_or_else(|| anyhow!("Admin not found"))?;
        
        if !admin.is_admin() {
            return Err(anyhow!("User does not have admin privileges"));
        }
        
        // 获取提现请求
        let mut request = self.db.get_withdrawal_request_by_id(request_id).await?
            .ok_or_else(|| anyhow!("Withdrawal request not found"))?;
        
        if approved {
            // 结算佣金
            let settled = self.db.settle_commission(&request.promoter_id, request.amount).await?;
            
            if !settled {
                return Err(anyhow!("Failed to settle commission"));
            }
            
            // 更新提现请求状态
            request.approve();
            
            // 如果提供了交易ID，则记录
            if let Some(tx_id) = transaction_id {
                // 创建佣金记录
                let mut commission_log = CommissionLog::new(
                    request.promoter_id.clone(),
                    request.amount,
                    CommissionType::FirstPayment, // 这里类型不重要，因为是提现
                    request.currency.clone()
                );
                
                commission_log.mark_as_paid(tx_id);
                
                self.db.create_commission_log(&commission_log).await?;
            }
        } else {
            // 拒绝提现请求
            request.reject();
        }
        
        // 更新提现请求
        self.db.update_withdrawal_request(&request).await?;
        
        Ok(())
    }
    
    // 处理用户注册时的邀请码
    pub async fn process_invite_code(&self, user_id: &str, invite_code: &str) -> Result<bool> {
        let result = self.db.process_invite_code_registration(user_id, invite_code).await?;
        Ok(result)
    }
    
    // 处理用户付费时的佣金计算
    pub async fn process_payment(&self, user_id: &str, payment_amount: f32, is_renewal: bool) -> Result<Option<CommissionLog>> {
        let commission_log = self.db.process_payment_commission(user_id, payment_amount, is_renewal).await?;
        Ok(commission_log)
    }
}

// 推广统计
pub struct PromotionStatistics {
    pub total_invited: usize,
    pub first_payments: usize,
    pub renewal_payments: usize,
    pub total_commission: f32,
    pub pending_commission: f32,
}
