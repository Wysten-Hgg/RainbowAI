use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::db::Database;
use crate::models::{
    WalletTx, CurrencyType, Gift, GiftRecord, LuckyCard, TxType
};
use crate::services::PointsService;
use crate::middleware::auth::AuthenticatedUser;

// ==================== 请求和响应结构 ====================

#[derive(Deserialize)]
pub struct DailyCheckinRequest {
    user_id: String,
}

#[derive(Serialize)]
pub struct DailyCheckinResponse {
    success: bool,
    streak: u32,
    current_hp: u32,
}

#[derive(Deserialize)]
pub struct SendGiftRequest {
    gift_id: String,
    receiver_ai_id: String,
    message: Option<String>,
}

#[derive(Serialize)]
pub struct SendGiftResponse {
    success: bool,
}

#[derive(Deserialize)]
pub struct UseLuckyCardRequest {
    card_id: String,
}

#[derive(Serialize)]
pub struct UseLuckyCardResponse {
    success: bool,
    multiplier: Option<f32>,
}

#[derive(Deserialize)]
pub struct RechargeLCRequest {
    amount: u32,
}

#[derive(Serialize)]
pub struct RechargeLCResponse {
    success: bool,
    new_balance: u32,
}

#[derive(Serialize)]
pub struct WalletInfoResponse {
    hp: u32,
    lc_balance: u32,
}

// ==================== 路由处理函数 ====================

// 每日签到
pub async fn daily_checkin(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<DailyCheckinResponse>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.daily_checkin(&auth_user.user_id).await {
        Ok((success, streak, current_hp)) => {
            Ok(Json(DailyCheckinResponse {
                success,
                streak,
                current_hp,
            }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取钱包信息
pub async fn get_wallet_info(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<WalletInfoResponse>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.get_user_wallet(&auth_user.user_id).await {
        Ok((hp, lc_balance)) => {
            Ok(Json(WalletInfoResponse {
                hp,
                lc_balance,
            }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取钱包交易记录
pub async fn get_wallet_transactions(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<WalletTx>>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.get_user_wallet_transactions(&auth_user.user_id, None, 50).await {
        Ok(transactions) => Ok(Json(transactions)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取积分交易记录
pub async fn get_hp_transactions(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<WalletTx>>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.get_user_wallet_transactions(&auth_user.user_id, Some(CurrencyType::HP), 50).await {
        Ok(transactions) => Ok(Json(transactions)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取金币交易记录
pub async fn get_lc_transactions(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<WalletTx>>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.get_user_wallet_transactions(&auth_user.user_id, Some(CurrencyType::LC), 50).await {
        Ok(transactions) => Ok(Json(transactions)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 充值金币
pub async fn recharge_lc(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<RechargeLCRequest>,
) -> Result<Json<RechargeLCResponse>, StatusCode> {
    let points_service = PointsService::new(db.clone());
    
    // 这里应该有支付流程，暂时简化处理
    match points_service.recharge_lc(&auth_user.user_id, payload.amount).await {
        Ok(()) => {
            // 获取新的余额
            let user = db.get_user_by_id(&auth_user.user_id).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::NOT_FOUND)?;
            
            Ok(Json(RechargeLCResponse {
                success: true,
                new_balance: user.lc_balance,
            }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取可用礼物列表
pub async fn get_available_gifts(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<Gift>>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.get_available_gifts().await {
        Ok(gifts) => Ok(Json(gifts)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 赠送礼物
pub async fn send_gift(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<SendGiftRequest>,
) -> Result<Json<SendGiftResponse>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.send_gift(&auth_user.user_id, &payload.gift_id, &payload.receiver_ai_id, payload.message).await {
        Ok(success) => {
            Ok(Json(SendGiftResponse {
                success,
            }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取用户赠送的礼物记录
pub async fn get_sent_gifts(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<GiftRecord>>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.get_user_sent_gifts(&auth_user.user_id, 50).await {
        Ok(records) => Ok(Json(records)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取AI收到的礼物记录
pub async fn get_ai_received_gifts(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Path(ai_id): Path<String>,
) -> Result<Json<Vec<GiftRecord>>, StatusCode> {
    let points_service = PointsService::new(db.clone());
    
    // 验证AI是否属于当前用户
    let ais = db.get_user_ais(&auth_user.user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let ai_belongs_to_user = ais.iter().any(|ai| ai.id == ai_id);
    if !ai_belongs_to_user {
        return Err(StatusCode::FORBIDDEN);
    }
    
    match points_service.get_ai_received_gifts(&ai_id, 50).await {
        Ok(records) => Ok(Json(records)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 获取有效的幸运卡
pub async fn get_valid_lucky_cards(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<LuckyCard>>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.get_user_valid_lucky_cards(&auth_user.user_id).await {
        Ok(cards) => Ok(Json(cards)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 使用幸运卡
pub async fn use_lucky_card(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UseLuckyCardRequest>,
) -> Result<Json<UseLuckyCardResponse>, StatusCode> {
    let points_service = PointsService::new(db);
    
    match points_service.use_lucky_card(&auth_user.user_id, &payload.card_id).await {
        Ok(multiplier) => {
            Ok(Json(UseLuckyCardResponse {
                success: multiplier.is_some(),
                multiplier,
            }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ==================== 路由配置 ====================

pub fn points_routes() -> Router<Database> {
    Router::new()
        // 积分相关路由
        .route("/checkin", post(daily_checkin))
        .route("/hp/transactions", get(get_hp_transactions))
        
        // 钱包相关路由
        .route("/wallet", get(get_wallet_info))
        .route("/wallet/transactions", get(get_wallet_transactions))
        .route("/wallet/recharge", post(recharge_lc))
        .route("/wallet/lc/transactions", get(get_lc_transactions))
        
        // 礼物系统路由
        .route("/gifts", get(get_available_gifts))
        .route("/gifts/send", post(send_gift))
        .route("/gifts/sent", get(get_sent_gifts))
        .route("/gifts/received/:ai_id", get(get_ai_received_gifts))
        
        // 幸运卡系统路由
        .route("/lucky-cards", get(get_valid_lucky_cards))
        .route("/lucky-cards/use", post(use_lucky_card))
}
