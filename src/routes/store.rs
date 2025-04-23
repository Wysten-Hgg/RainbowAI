use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::models::{ShopItem, ShopItemCategory, PurchaseRecord, MonthlyRedemptionStat};
use crate::middleware::auth::AuthenticatedUser;

// 创建商城路由
pub fn create_store_routes() -> Router<Database> {
    Router::new()
        .route("/items", get(get_store_items))
        .route("/items/:category", get(get_store_items_by_category))
        .route("/item/:id", get(get_store_item))
        .route("/redeem", post(redeem_item))
        .route("/my-history", get(get_user_redemption_history))
        .route("/my-purchases", get(get_user_purchases))
}

// 创建管理员路由
pub fn create_admin_store_routes() -> Router<Database> {
    Router::new()
        .route("/items", get(admin_get_all_items))
        .route("/create", post(admin_create_item))
        .route("/update", post(admin_update_item))
        .route("/delete/:id", post(admin_delete_item))
        .route("/redemptions", post(admin_get_redemptions))
}

// ==================== 用户接口 ====================

// 获取商城商品列表响应
#[derive(Serialize)]
pub struct StoreItemsResponse {
    pub items: Vec<ShopItem>,
}

// 获取所有可用商品
pub async fn get_store_items(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<StoreItemsResponse>, StatusCode> {
    let items = db.get_available_shop_items().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(StoreItemsResponse { items }))
}

// 按分类获取商品
pub async fn get_store_items_by_category(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Path(category): Path<String>,
) -> Result<Json<StoreItemsResponse>, StatusCode> {
    let category = match category.as_str() {
        "coupon" => ShopItemCategory::Coupon,
        "decoration" => ShopItemCategory::Decoration,
        "function" => ShopItemCategory::Function,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    let items = db.get_available_shop_items_by_category(&category).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(StoreItemsResponse { items }))
}

// 获取单个商品信息响应
#[derive(Serialize)]
pub struct StoreItemResponse {
    pub item: ShopItem,
    pub discounted_price: u32,
}

// 获取单个商品信息
pub async fn get_store_item(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Json<StoreItemResponse>, StatusCode> {
    let item = db.get_shop_item(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // 如果商品不可见，则返回404
    if !item.visible {
        return Err(StatusCode::NOT_FOUND);
    }
    
    // 计算折扣价格
    let discounted_price = db.calculate_discounted_price(&auth_user.user_id, &id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(StoreItemResponse { 
        item, 
        discounted_price,
    }))
}

// 兑换商品请求
#[derive(Deserialize)]
pub struct RedeemItemRequest {
    pub item_id: String,
    pub remark: Option<String>,
}

// 兑换商品响应
#[derive(Serialize)]
pub struct RedeemItemResponse {
    pub success: bool,
    pub message: String,
    pub purchase_id: Option<String>,
}

// 兑换商品
pub async fn redeem_item(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(request): Json<RedeemItemRequest>,
) -> Result<Json<RedeemItemResponse>, StatusCode> {
    // 获取商品信息
    let item = db.get_shop_item(&request.item_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // 检查商品是否可用
    if !item.is_available() {
        return Ok(Json(RedeemItemResponse {
            success: false,
            message: "商品不可用或已售罄".to_string(),
            purchase_id: None,
        }));
    }
    
    // 获取用户信息
    let user = db.get_user_by_id(&auth_user.user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // 计算折扣价格
    let price_to_pay = item.get_discounted_price(&user.vip_level);
    
    // 检查用户积分是否足够
    if user.hp < price_to_pay {
        return Ok(Json(RedeemItemResponse {
            success: false,
            message: format!("积分不足，需要 {} 积分", price_to_pay),
            purchase_id: None,
        }));
    }
    
    // 检查月度兑换限制
    if let Some(monthly_limit) = item.monthly_limit {
        let stat = db.get_user_monthly_redemption_stat(&auth_user.user_id).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        if let Some(stat) = stat {
            let item_type_str = format!("{:?}", item.item_type);
            if !stat.check_monthly_limit(&item_type_str, monthly_limit) {
                return Ok(Json(RedeemItemResponse {
                    success: false,
                    message: format!("已达到本月兑换上限 ({} 次)", monthly_limit),
                    purchase_id: None,
                }));
            }
        }
    }
    
    // 执行兑换
    let result = db.redeem_shop_item(&auth_user.user_id, &request.item_id, request.remark).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result {
        // 获取最新的购买记录
        let purchases = db.get_user_purchases(&auth_user.user_id, 1).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let purchase_id = purchases.first().map(|p| p.id.clone());
        
        Ok(Json(RedeemItemResponse {
            success: true,
            message: format!("成功兑换商品: {}", item.name),
            purchase_id,
        }))
    } else {
        Ok(Json(RedeemItemResponse {
            success: false,
            message: "兑换失败，请稍后再试".to_string(),
            purchase_id: None,
        }))
    }
}

// 用户兑换历史响应
#[derive(Serialize)]
pub struct UserRedemptionHistoryResponse {
    pub monthly_stats: Vec<MonthlyRedemptionStat>,
    pub total_points_spent: u32,
}

// 获取用户兑换历史
pub async fn get_user_redemption_history(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<UserRedemptionHistoryResponse>, StatusCode> {
    let stats = db.get_user_redemption_history(&auth_user.user_id, 12).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let total_points_spent = stats.iter().map(|s| s.total_points_spent).sum();
    
    Ok(Json(UserRedemptionHistoryResponse {
        monthly_stats: stats,
        total_points_spent,
    }))
}

// 用户购买记录响应
#[derive(Serialize)]
pub struct UserPurchasesResponse {
    pub purchases: Vec<PurchaseWithItem>,
}

#[derive(Serialize)]
pub struct PurchaseWithItem {
    pub purchase: PurchaseRecord,
    pub item: Option<ShopItem>,
}

// 获取用户购买记录
pub async fn get_user_purchases(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<UserPurchasesResponse>, StatusCode> {
    let purchases = db.get_user_purchases(&auth_user.user_id, 50).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut purchases_with_items = Vec::new();
    
    for purchase in purchases {
        let item = db.get_shop_item(&purchase.item_id).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        purchases_with_items.push(PurchaseWithItem {
            purchase,
            item,
        });
    }
    
    Ok(Json(UserPurchasesResponse {
        purchases: purchases_with_items,
    }))
}

// ==================== 管理员接口 ====================

// 管理员获取所有商品
pub async fn admin_get_all_items(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
) -> Result<Json<StoreItemsResponse>, StatusCode> {
    // 检查管理员权限
    if !auth_user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let items = db.get_all_shop_items().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(StoreItemsResponse { items }))
}

// 创建商品请求
#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: String,
    pub item_type: String,
    pub category: String,
    pub price_hp: u32,
    pub image_url: Option<String>,
    pub is_limited: bool,
    pub available_until: Option<i64>,
    pub stock: Option<u32>,
    pub visible: bool,
    pub linked_coupon_id: Option<String>,
    pub monthly_limit: Option<u32>,
    pub vip_discount: Option<bool>,
}

// 创建商品响应
#[derive(Serialize)]
pub struct CreateItemResponse {
    pub success: bool,
    pub item_id: Option<String>,
    pub message: String,
}

// 管理员创建商品
pub async fn admin_create_item(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(request): Json<CreateItemRequest>,
) -> Result<Json<CreateItemResponse>, StatusCode> {
    // 检查管理员权限
    if !auth_user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 解析商品类型
    let item_type = match request.item_type.as_str() {
        "AIDecoration" => crate::models::ShopItemType::AIDecoration,
        "UserTitle" => crate::models::ShopItemType::UserTitle,
        "LIOAccessTicket" => crate::models::ShopItemType::LIOAccessTicket,
        "AISlotExpansion" => crate::models::ShopItemType::AISlotExpansion,
        "ExclusiveStory" => crate::models::ShopItemType::ExclusiveStory,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    // 解析商品分类
    let category = match request.category.as_str() {
        "Coupon" => crate::models::ShopItemCategory::Coupon,
        "Decoration" => crate::models::ShopItemCategory::Decoration,
        "Function" => crate::models::ShopItemCategory::Function,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    // 创建商品
    let item = ShopItem::new(
        request.name,
        request.description,
        item_type,
        category,
        request.price_hp,
        request.image_url,
        request.is_limited,
        request.available_until,
        request.stock,
        request.visible,
        request.linked_coupon_id,
        request.monthly_limit,
        request.vip_discount,
    );
    
    // 保存商品
    db.create_shop_item(&item).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(CreateItemResponse {
        success: true,
        item_id: Some(item.id.clone()),
        message: "商品创建成功".to_string(),
    }))
}

// 更新商品请求
#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price_hp: Option<u32>,
    pub image_url: Option<String>,
    pub is_limited: Option<bool>,
    pub available_until: Option<i64>,
    pub stock: Option<u32>,
    pub visible: Option<bool>,
    pub linked_coupon_id: Option<String>,
    pub monthly_limit: Option<u32>,
    pub vip_discount: Option<bool>,
}

// 更新商品响应
#[derive(Serialize)]
pub struct UpdateItemResponse {
    pub success: bool,
    pub message: String,
}

// 管理员更新商品
pub async fn admin_update_item(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(request): Json<UpdateItemRequest>,
) -> Result<Json<UpdateItemResponse>, StatusCode> {
    // 检查管理员权限
    if !auth_user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 获取商品
    let mut item = db.get_shop_item(&request.id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // 更新商品信息
    if let Some(name) = request.name {
        item.name = name;
    }
    
    if let Some(description) = request.description {
        item.description = description;
    }
    
    if let Some(price_hp) = request.price_hp {
        item.price_hp = price_hp;
    }
    
    if let Some(image_url) = request.image_url {
        item.image_url = Some(image_url);
    }
    
    if let Some(is_limited) = request.is_limited {
        item.is_limited = is_limited;
    }
    
    if let Some(available_until) = request.available_until {
        item.available_until = Some(available_until);
    }
    
    if let Some(stock) = request.stock {
        item.stock = Some(stock);
    }
    
    if let Some(visible) = request.visible {
        item.visible = visible;
    }
    
    if let Some(linked_coupon_id) = request.linked_coupon_id {
        item.linked_coupon_id = Some(linked_coupon_id);
    }
    
    if let Some(monthly_limit) = request.monthly_limit {
        item.monthly_limit = Some(monthly_limit);
    }
    
    if let Some(vip_discount) = request.vip_discount {
        item.vip_discount = Some(vip_discount);
    }
    
    // 保存更新
    db.update_shop_item(&item).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(UpdateItemResponse {
        success: true,
        message: "商品更新成功".to_string(),
    }))
}

// 删除商品响应
#[derive(Serialize)]
pub struct DeleteItemResponse {
    pub success: bool,
    pub message: String,
}

// 管理员删除商品
pub async fn admin_delete_item(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Json<DeleteItemResponse>, StatusCode> {
    // 检查管理员权限
    if !auth_user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // 删除商品
    db.delete_shop_item(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(DeleteItemResponse {
        success: true,
        message: "商品删除成功".to_string(),
    }))
}

// 管理员获取兑换记录请求
#[derive(Deserialize)]
pub struct AdminGetRedemptionsRequest {
    pub user_id: Option<String>,
    pub limit: Option<usize>,
}

// 管理员获取兑换记录响应
#[derive(Serialize)]
pub struct AdminGetRedemptionsResponse {
    pub purchases: Vec<PurchaseWithUserAndItem>,
}

#[derive(Serialize)]
pub struct PurchaseWithUserAndItem {
    pub purchase: PurchaseRecord,
    pub user: Option<crate::models::User>,
    pub item: Option<ShopItem>,
}

// 管理员获取兑换记录
pub async fn admin_get_redemptions(
    State(db): State<Database>,
    auth_user: AuthenticatedUser,
    Json(request): Json<AdminGetRedemptionsRequest>,
) -> Result<Json<AdminGetRedemptionsResponse>, StatusCode> {
    // 检查管理员权限
    if !auth_user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let limit = request.limit.unwrap_or(50);
    
    let purchases = if let Some(user_id) = request.user_id {
        db.get_user_purchases(&user_id, limit).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        // 获取所有用户的购买记录，这里需要实现一个新的数据库方法
        // 暂时返回空列表
        Vec::new()
    };
    
    let mut purchases_with_data = Vec::new();
    
    for purchase in purchases {
        let user = db.get_user_by_id(&purchase.user_id).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        let item = db.get_shop_item(&purchase.item_id).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        purchases_with_data.push(PurchaseWithUserAndItem {
            purchase,
            user,
            item,
        });
    }
    
    Ok(Json(AdminGetRedemptionsResponse {
        purchases: purchases_with_data,
    }))
}
