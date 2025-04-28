use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Serialize, Deserialize};
use crate::db::Database;
use crate::models::{
    Promoter, PromotionRecord, CommissionLog, WithdrawalRequest, 
    PromoterType, VerificationStatus, CommissionStatus
};
use crate::services::PromoterService;
use crate::middleware::auth::AuthenticatedUser;

// 路由配置
pub fn promoter_routes() -> Router<Database> {
    Router::new()
        // 推广者管理路由
        .route("/apply", post(apply_for_promoter))
        .route("/verify", post(upload_verification_document))
        .route("/agreement", post(sign_agreement))
        .route("/status", get(get_promoter_status))
        .route("/invite-code", get(get_invite_code))
        
        // 推广记录路由
        .route("/records", get(get_promotion_records))
        .route("/statistics", get(get_promotion_statistics))
        .route("/invited-users", get(get_invited_users))
        
        // 佣金管理路由
        .route("/commissions", get(get_commission_logs))
        .route("/withdraw", post(request_withdrawal))
        .route("/withdrawals", get(get_withdrawal_requests))
        .route("/payment-account", post(update_payment_account))
}

// 管理员路由配置
pub fn admin_promoter_routes() -> Router<Database> {
    Router::new()
        .route("/review", post(admin_review_promoter))
        .route("/list", get(admin_get_promoters))
        .route("/pending", get(admin_get_pending_promoters))
        .route("/commission-rate", post(admin_update_commission_rates))
        .route("/withdrawals", get(admin_get_withdrawal_requests))
        .route("/withdrawal/process", post(admin_process_withdrawal))
}

// ==================== 推广者管理接口 ====================

#[derive(Deserialize)]
pub struct ApplyForPromoterRequest {
    promoter_type: String,
    wallet_account: String,
}

#[derive(Serialize)]
pub struct PromoterResponse {
    promoter: Promoter,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    success: bool,
}

// 申请成为推广者
pub async fn apply_for_promoter(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ApplyForPromoterRequest>,
) -> Result<Json<PromoterResponse>, StatusCode> {
    let promoter_type = match payload.promoter_type.as_str() {
        "Individual" => PromoterType::Individual,
        "Organization" => PromoterType::Organization,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    let promoter_service = PromoterService::new(db);
    
    match promoter_service.apply_for_promoter(&auth_user.user_id, promoter_type, payload.wallet_account).await {
        Ok(promoter) => Ok(Json(PromoterResponse { promoter })),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[derive(Deserialize)]
pub struct UploadDocumentRequest {
    document_path: String,
}

// 上传身份验证文档
pub async fn upload_verification_document(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UploadDocumentRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    let promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 上传文档
    match promoter_service.upload_id_document(&promoter.id, payload.document_path).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 签署推广协议
pub async fn sign_agreement(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    let promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 签署协议
    match promoter_service.sign_agreement(&promoter.id).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取推广者状态
pub async fn get_promoter_status(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<PromoterResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(promoter)) => Ok(Json(PromoterResponse { promoter })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Serialize)]
pub struct InviteCodeResponse {
    invite_code: String,
}

// 获取邀请码
pub async fn get_invite_code(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<InviteCodeResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(promoter)) => {
            // 检查推广者是否已验证
            if !promoter.is_verified() {
                return Err(StatusCode::FORBIDDEN);
            }
            
            Ok(Json(InviteCodeResponse { 
                invite_code: promoter.invite_code 
            }))
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ==================== 推广记录接口 ====================

#[derive(Serialize)]
pub struct PromotionRecordsResponse {
    records: Vec<PromotionRecord>,
}

// 获取推广记录
pub async fn get_promotion_records(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<PromotionRecordsResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    let promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 获取推广记录
    match promoter_service.get_promotion_records(&promoter.id, 100).await {
        Ok(records) => Ok(Json(PromotionRecordsResponse { records })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Serialize)]
pub struct PromotionStatisticsResponse {
    total_invited: usize,
    first_payments: usize,
    renewal_payments: usize,
    total_commission: f32,
    pending_commission: f32,
}

// 获取推广统计
pub async fn get_promotion_statistics(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<PromotionStatisticsResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    let promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 获取推广统计
    match promoter_service.get_promotion_statistics(&promoter.id).await {
        Ok(stats) => Ok(Json(PromotionStatisticsResponse {
            total_invited: stats.total_invited,
            first_payments: stats.first_payments,
            renewal_payments: stats.renewal_payments,
            total_commission: stats.total_commission,
            pending_commission: stats.pending_commission,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Serialize)]
pub struct InvitedUsersResponse {
    records: Vec<PromotionRecord>,
}

// 获取已邀请用户列表
pub async fn get_invited_users(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<InvitedUsersResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    let promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 获取推广记录
    match promoter_service.get_promotion_records(&promoter.id, 100).await {
        Ok(records) => Ok(Json(InvitedUsersResponse { records })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ==================== 佣金管理接口 ====================

#[derive(Serialize)]
pub struct CommissionLogsResponse {
    logs: Vec<CommissionLog>,
}

// 获取佣金记录
pub async fn get_commission_logs(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<CommissionLogsResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    let promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 获取佣金记录
    match promoter_service.get_commission_logs(&promoter.id, 100).await {
        Ok(logs) => Ok(Json(CommissionLogsResponse { logs })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct WithdrawalRequestPayload {
    amount: f32,
    currency: String,
    payment_method: String,
    account_info: String,
}

#[derive(Serialize)]
pub struct WithdrawalResponse {
    request: WithdrawalRequest,
}

// 申请提现
pub async fn request_withdrawal(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<WithdrawalRequestPayload>,
) -> Result<Json<WithdrawalResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    let promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 申请提现
    match promoter_service.request_withdrawal(
        &promoter.id,
        payload.amount,
        payload.currency,
        payload.payment_method,
        payload.account_info
    ).await {
        Ok(request) => Ok(Json(WithdrawalResponse { request })),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[derive(Serialize)]
pub struct WithdrawalRequestsResponse {
    requests: Vec<WithdrawalRequest>,
}

// 获取提现请求
pub async fn get_withdrawal_requests(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<WithdrawalRequestsResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取用户的推广者信息
    let promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 获取提现请求
    match promoter_service.get_withdrawal_requests(&promoter.id, 100).await {
        Ok(requests) => Ok(Json(WithdrawalRequestsResponse { requests })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct UpdatePaymentAccountRequest {
    wallet_account: String,
}

// 更新收款账户
pub async fn update_payment_account(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UpdatePaymentAccountRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db.clone());
    
    // 获取用户的推广者信息
    let mut promoter = match promoter_service.get_promoter_by_user(&auth_user.user_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // 更新收款账户
    promoter.wallet_account = payload.wallet_account;
    promoter.updated_at = time::OffsetDateTime::now_utc().unix_timestamp();
    
    // 保存更新
    match promoter_service.get_promoter(&promoter.id).await {
        Ok(_) => {
            if let Err(_) = promoter_service.update_promoter(&promoter).await {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            Ok(Json(SuccessResponse { success: true }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ==================== 管理员接口 ====================

#[derive(Deserialize)]
pub struct ReviewPromoterRequest {
    promoter_id: String,
    approved: bool,
}

// 审核推广者申请
pub async fn admin_review_promoter(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ReviewPromoterRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 审核推广者申请
    match promoter_service.review_promoter_application(
        &payload.promoter_id,
        payload.approved,
        &auth_user.user_id
    ).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}

#[derive(Serialize)]
pub struct PromotersResponse {
    promoters: Vec<Promoter>,
}

// 获取所有推广者
pub async fn admin_get_promoters(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<PromotersResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取所有推广者
    match promoter_service.get_all_promoters(&auth_user.user_id).await {
        Ok(promoters) => Ok(Json(PromotersResponse { promoters })),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}

// 获取待审核的推广者
pub async fn admin_get_pending_promoters(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<PromotersResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 获取待审核的推广者
    match promoter_service.get_pending_promoters(&auth_user.user_id).await {
        Ok(promoters) => Ok(Json(PromotersResponse { promoters })),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}

#[derive(Deserialize)]
pub struct UpdateCommissionRatesRequest {
    promoter_id: String,
    commission_rate: f32,
    renewal_rate: f32,
}

// 更新佣金比例
pub async fn admin_update_commission_rates(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UpdateCommissionRatesRequest>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 更新佣金比例
    match promoter_service.update_commission_rates(
        &payload.promoter_id,
        payload.commission_rate,
        payload.renewal_rate,
        &auth_user.user_id
    ).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}

// 获取所有待处理的提现请求
pub async fn admin_get_withdrawal_requests(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<WithdrawalRequestsResponse>, StatusCode> {
    // 检查管理员权限
    let user = match db.get_user_by_id(&auth_user.user_id).await {
        Ok(Some(u)) => u,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };
    
    if !user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 获取所有待处理的提现请求
    match db.get_pending_withdrawal_requests().await {
        Ok(requests) => Ok(Json(WithdrawalRequestsResponse { requests })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct ProcessWithdrawalRequestPayload {
    request_id: String,
    approved: bool,
    transaction_id: Option<String>,
}

// 处理提现请求
pub async fn admin_process_withdrawal(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ProcessWithdrawalRequestPayload>,
) -> Result<Json<SuccessResponse>, StatusCode> {
    let promoter_service = PromoterService::new(db);
    
    // 处理提现请求
    match promoter_service.process_withdrawal_request(
        &payload.request_id,
        payload.approved,
        payload.transaction_id,
        &auth_user.user_id
    ).await {
        Ok(_) => Ok(Json(SuccessResponse { success: true })),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}
