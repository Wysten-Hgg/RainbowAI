-- Create User table
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD id ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON user TYPE string ASSERT is::email($value);
DEFINE FIELD password_hash ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD frontend_roles ON user TYPE array;
DEFINE FIELD backend_roles ON user TYPE array;
DEFINE FIELD vip_level ON user TYPE string;
DEFINE FIELD ai_partner_count ON user TYPE int;
DEFINE FIELD companion_ai_count ON user TYPE int DEFAULT 0;
DEFINE FIELD creative_ai_count ON user TYPE int DEFAULT 0;
DEFINE FIELD work_ai_count ON user TYPE int DEFAULT 0;
DEFINE FIELD service_ai_count ON user TYPE int DEFAULT 0;
DEFINE FIELD free_mapping_used ON user TYPE int DEFAULT 0;
DEFINE FIELD daily_chat_count ON user TYPE int;
DEFINE FIELD daily_lio_count ON user TYPE int;
DEFINE FIELD invite_code ON user TYPE option<string>;
DEFINE FIELD created_at ON user TYPE int;
DEFINE FIELD updated_at ON user TYPE int;
ALTER TABLE user ADD FIELD hp TYPE int DEFAULT 0;
ALTER TABLE user ADD FIELD lc_balance TYPE int DEFAULT 0;
ALTER TABLE user ADD FIELD daily_checkin_streak TYPE int DEFAULT 0;
ALTER TABLE user ADD FIELD last_checkin_date TYPE option<int>;
ALTER TABLE user ADD FIELD total_invites TYPE int DEFAULT 0;
ALTER TABLE user ADD FIELD invited_by TYPE option<string>;
ALTER TABLE user ADD FIELD is_email_verified TYPE bool DEFAULT false;
ALTER TABLE user ADD FIELD ai_slots TYPE int DEFAULT 1;

-- Create Invite table
DEFINE TABLE invite SCHEMAFULL;
DEFINE FIELD code ON invite TYPE string ASSERT $value != NONE;
DEFINE FIELD used_by ON invite TYPE array;
DEFINE FIELD creator_id ON invite TYPE string ASSERT $value != NONE;
DEFINE FIELD usage_limit ON invite TYPE int;
DEFINE FIELD expires_at ON invite TYPE int;
DEFINE FIELD created_at ON invite TYPE int;
DEFINE FIELD updated_at ON invite TYPE int;

-- Create AI table
DEFINE TABLE ai SCHEMAFULL;
DEFINE FIELD id ON ai TYPE string ASSERT $value != NONE;
DEFINE FIELD name ON ai TYPE string;
DEFINE FIELD ai_type ON ai TYPE string;
DEFINE FIELD status ON ai TYPE string;
DEFINE FIELD user_id ON ai TYPE string ASSERT $value != NONE;
DEFINE FIELD awakened ON ai TYPE bool DEFAULT false;
DEFINE FIELD awakened_by ON ai TYPE option<string>;
DEFINE FIELD created_at ON ai TYPE int;
DEFINE FIELD updated_at ON ai TYPE int;

-- Create AuditLog table
DEFINE TABLE audit_log SCHEMAFULL;
DEFINE FIELD id ON audit_log TYPE string ASSERT $value != NONE;
DEFINE FIELD action ON audit_log TYPE string;
DEFINE FIELD user_id ON audit_log TYPE string ASSERT $value != NONE;
DEFINE FIELD created_at ON audit_log TYPE int;

-- Create EmailVerification table
DEFINE TABLE email_verification SCHEMAFULL;
DEFINE FIELD id ON email_verification TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON email_verification TYPE string ASSERT is::email($value);
DEFINE FIELD code ON email_verification TYPE string;
DEFINE FIELD verification_type ON email_verification TYPE string;
DEFINE FIELD expires_at ON email_verification TYPE int;
DEFINE FIELD used ON email_verification TYPE bool;
DEFINE FIELD created_at ON email_verification TYPE int;

-- Create Coupon table
DEFINE TABLE coupon SCHEMAFULL;
DEFINE FIELD id ON coupon TYPE string ASSERT $value != NONE;
DEFINE FIELD coupon_type ON coupon TYPE string;
DEFINE FIELD sub_type ON coupon TYPE string;
DEFINE FIELD value ON coupon TYPE float;
DEFINE FIELD duration_days ON coupon TYPE option<int>;
DEFINE FIELD status ON coupon TYPE string;
DEFINE FIELD owner_id ON coupon TYPE string ASSERT $value != NONE;
DEFINE FIELD issued_at ON coupon TYPE int;
DEFINE FIELD expires_at ON coupon TYPE int;
DEFINE FIELD is_transferable ON coupon TYPE bool;

-- Create VIP Config table
DEFINE TABLE vip_config SCHEMAFULL;
DEFINE FIELD level ON vip_config TYPE string;
DEFINE FIELD max_ai_partners ON vip_config TYPE int;
DEFINE FIELD daily_chat_limit ON vip_config TYPE int;
DEFINE FIELD daily_lio_limit ON vip_config TYPE int;
DEFINE FIELD max_companion_ai ON vip_config TYPE int;
DEFINE FIELD max_creative_ai ON vip_config TYPE int;
DEFINE FIELD max_work_ai ON vip_config TYPE int;
DEFINE FIELD max_service_ai ON vip_config TYPE int;
DEFINE FIELD free_mapping_quota ON vip_config TYPE int;

-- 积分与货币系统 - 创建钱包交易记录表
DEFINE TABLE wallet_tx SCHEMAFULL;
DEFINE FIELD id ON wallet_tx TYPE string ASSERT $value != NONE;
DEFINE FIELD user_id ON wallet_tx TYPE string ASSERT $value != NONE;
DEFINE FIELD tx_type ON wallet_tx TYPE string ASSERT $value IN ['Recharge', 'GiftSend', 'GiftReceive', 'Reward', 'PointsEarned', 'PointsSpent'];
DEFINE FIELD amount ON wallet_tx TYPE int ASSERT $value > 0;
DEFINE FIELD currency ON wallet_tx TYPE string ASSERT $value IN ['HP', 'LC'];
DEFINE FIELD timestamp ON wallet_tx TYPE int;
DEFINE FIELD related_entity_id ON wallet_tx TYPE option<string>;
DEFINE FIELD remark ON wallet_tx TYPE option<string>;

-- 创建礼物表
DEFINE TABLE gift SCHEMAFULL;
DEFINE FIELD id ON gift TYPE string ASSERT $value != NONE;
DEFINE FIELD name ON gift TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON gift TYPE option<string>;
DEFINE FIELD price_lc ON gift TYPE int ASSERT $value > 0;
DEFINE FIELD emotional_value ON gift TYPE int;
DEFINE FIELD effect_type ON gift TYPE string ASSERT $value IN ['Boost', 'Memory', 'Exclusive'];
DEFINE FIELD image_url ON gift TYPE option<string>;
DEFINE FIELD created_at ON gift TYPE int;
DEFINE FIELD is_limited ON gift TYPE bool;
DEFINE FIELD available_until ON gift TYPE option<int>;

-- 创建礼物记录表
DEFINE TABLE gift_record SCHEMAFULL;
DEFINE FIELD id ON gift_record TYPE string ASSERT $value != NONE;
DEFINE FIELD gift_id ON gift_record TYPE string ASSERT $value != NONE;
DEFINE FIELD sender_id ON gift_record TYPE string ASSERT $value != NONE;
DEFINE FIELD receiver_ai_id ON gift_record TYPE string ASSERT $value != NONE;
DEFINE FIELD sent_at ON gift_record TYPE int;
DEFINE FIELD message ON gift_record TYPE option<string>;

-- 创建幸运卡表
DEFINE TABLE lucky_card SCHEMAFULL;
DEFINE FIELD id ON lucky_card TYPE string ASSERT $value != NONE;
DEFINE FIELD level ON lucky_card TYPE string ASSERT $value IN ['A', 'B', 'C', 'D', 'E'];
DEFINE FIELD owner_id ON lucky_card TYPE string ASSERT $value != NONE;
DEFINE FIELD multiplier ON lucky_card TYPE float;
DEFINE FIELD created_at ON lucky_card TYPE int;
DEFINE FIELD expires_at ON lucky_card TYPE int;
DEFINE FIELD is_used ON lucky_card TYPE bool DEFAULT false;
DEFINE FIELD used_at ON lucky_card TYPE option<int>;
DEFINE FIELD issued_by_ai_id ON lucky_card TYPE option<string>;

-- 创建积分商城商品表
DEFINE TABLE shop_item SCHEMAFULL;
DEFINE FIELD id ON shop_item TYPE string ASSERT $value != NONE;
DEFINE FIELD name ON shop_item TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON shop_item TYPE string;
DEFINE FIELD item_type ON shop_item TYPE string ASSERT $value IN ['AIDecoration', 'UserTitle', 'LIOAccessTicket', 'AISlotExpansion', 'ExclusiveStory'];
DEFINE FIELD price_hp ON shop_item TYPE int ASSERT $value > 0;
DEFINE FIELD image_url ON shop_item TYPE option<string>;
DEFINE FIELD is_limited ON shop_item TYPE bool;
DEFINE FIELD available_until ON shop_item TYPE option<int>;
DEFINE FIELD created_at ON shop_item TYPE int;
DEFINE FIELD stock ON shop_item TYPE option<int>;
DEFINE FIELD category ON shop_item TYPE string ASSERT $value IN ['Coupon', 'Decoration', 'Function'];
DEFINE FIELD visible ON shop_item TYPE bool DEFAULT true;
DEFINE FIELD linked_coupon_id ON shop_item TYPE option<string>;
DEFINE FIELD monthly_limit ON shop_item TYPE option<int>;
DEFINE FIELD vip_discount ON shop_item TYPE option<bool> DEFAULT false;

-- 创建购买记录表
DEFINE TABLE purchase_record SCHEMAFULL;
DEFINE FIELD id ON purchase_record TYPE string ASSERT $value != NONE;
DEFINE FIELD user_id ON purchase_record TYPE string ASSERT $value != NONE;
DEFINE FIELD item_id ON purchase_record TYPE string ASSERT $value != NONE;
DEFINE FIELD price_paid ON purchase_record TYPE int;
DEFINE FIELD purchased_at ON purchase_record TYPE int;
DEFINE FIELD is_activated ON purchase_record TYPE bool DEFAULT false;
DEFINE FIELD activated_at ON purchase_record TYPE option<int>;
DEFINE FIELD expires_at ON purchase_record TYPE option<int>;
DEFINE FIELD remark ON purchase_record TYPE option<string>;

-- 创建月度兑换统计表
DEFINE TABLE monthly_redemption_stat SCHEMAFULL;
DEFINE FIELD id ON monthly_redemption_stat TYPE string ASSERT $value != NONE;
DEFINE FIELD user_id ON monthly_redemption_stat TYPE string ASSERT $value != NONE;
DEFINE FIELD year_month ON monthly_redemption_stat TYPE string;
DEFINE FIELD item_type_counts ON monthly_redemption_stat TYPE object;
DEFINE FIELD total_points_spent ON monthly_redemption_stat TYPE int DEFAULT 0;
DEFINE FIELD updated_at ON monthly_redemption_stat TYPE int;

-- 创建关系定义
-- 用户与钱包交易记录的关系
DEFINE INDEX wallet_tx_user_idx ON TABLE wallet_tx COLUMNS user_id;
-- 用户与幸运卡的关系
DEFINE INDEX lucky_card_owner_idx ON TABLE lucky_card COLUMNS owner_id;
-- 用户与购买记录的关系
DEFINE INDEX purchase_user_idx ON TABLE purchase_record COLUMNS user_id;
-- 礼物记录与发送者的关系
DEFINE INDEX gift_sender_idx ON TABLE gift_record COLUMNS sender_id;
-- 礼物记录与接收AI的关系
DEFINE INDEX gift_receiver_idx ON TABLE gift_record COLUMNS receiver_ai_id;
-- 用户与月度兑换统计的关系
DEFINE INDEX monthly_redemption_user_idx ON TABLE monthly_redemption_stat COLUMNS user_id;
-- 月度兑换统计的月份索引
DEFINE INDEX monthly_redemption_month_idx ON TABLE monthly_redemption_stat COLUMNS year_month;

-- 初始化一些基础礼物数据
CREATE gift:heart SET 
    name = '爱心', 
    description = '表达你的爱意', 
    price_lc = 10, 
    emotional_value = 5, 
    effect_type = 'Boost', 
    image_url = '/images/gifts/heart.png', 
    created_at = time::now(), 
    is_limited = false;

CREATE gift:flower SET 
    name = '鲜花', 
    description = '芬芳的祝福', 
    price_lc = 20, 
    emotional_value = 10, 
    effect_type = 'Boost', 
    image_url = '/images/gifts/flower.png', 
    created_at = time::now(), 
    is_limited = false;

CREATE gift:ring SET 
    name = '戒指', 
    description = '永恒的承诺', 
    price_lc = 100, 
    emotional_value = 50, 
    effect_type = 'Memory', 
    image_url = '/images/gifts/ring.png', 
    created_at = time::now(), 
    is_limited = false;

-- 初始化一些基础商城商品数据

-- Pro 2天体验券
CREATE shop_item:pro_2day_trial SET 
    name = 'Pro 2天体验券',
    description = '获得2天Pro会员体验权限',
    item_type = 'LIOAccessTicket',
    category = 'Coupon',
    price_hp = 100,
    image_url = '/images/shop/pro_trial.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true,
    monthly_limit = 1;

-- Pro 一周体验券
CREATE shop_item:pro_1week_trial SET 
    name = 'Pro 一周体验券',
    description = '获得7天Pro会员体验权限',
    item_type = 'LIOAccessTicket',
    category = 'Coupon',
    price_hp = 300,
    image_url = '/images/shop/pro_trial.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true,
    monthly_limit = 1;

-- Premium 一周体验券
CREATE shop_item:premium_1week_trial SET 
    name = 'Premium 一周体验券',
    description = '获得7天Premium会员体验权限',
    item_type = 'LIOAccessTicket',
    category = 'Coupon',
    price_hp = 900,
    image_url = '/images/shop/premium_trial.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true,
    monthly_limit = 1;

-- 95折折扣券
CREATE shop_item:discount_95 SET 
    name = '95折折扣券',
    description = '购买会员时享受95折优惠',
    item_type = 'LIOAccessTicket',
    category = 'Coupon',
    price_hp = 300,
    image_url = '/images/shop/discount.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = false,
    monthly_limit = 2;

-- 9折折扣券
CREATE shop_item:discount_90 SET 
    name = '9折折扣券',
    description = '购买会员时享受9折优惠',
    item_type = 'LIOAccessTicket',
    category = 'Coupon',
    price_hp = 500,
    image_url = '/images/shop/discount.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = false,
    monthly_limit = 1;

-- $5 现金券
CREATE shop_item:cash_5 SET 
    name = '$5 现金券',
    description = '价值5美元的现金抵扣券',
    item_type = 'LIOAccessTicket',
    category = 'Coupon',
    price_hp = 500,
    image_url = '/images/shop/cash.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true,
    monthly_limit = 2;

-- AI星空皮肤
CREATE shop_item:ai_skin_starry SET 
    name = 'AI星空皮肤',
    description = '为你的AI伴侣添加梦幻星空背景',
    item_type = 'AIDecoration',
    category = 'Decoration',
    price_hp = 500,
    image_url = '/images/shop/ai_skin_starry.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true;

-- 彩虹城贵宾称号
CREATE shop_item:title_vip SET 
    name = '彩虹城贵宾称号',
    description = '专属称号，在社区中展示你的身份',
    item_type = 'UserTitle',
    category = 'Decoration',
    price_hp = 1000,
    image_url = '/images/shop/title_vip.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true;

-- AI扩展名额
CREATE shop_item:ai_slot_expansion SET 
    name = 'AI伴侣扩展名额',
    description = '增加一个AI伴侣名额',
    item_type = 'AISlotExpansion',
    category = 'Function',
    price_hp = 10000,
    image_url = '/images/shop/ai_slot.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true,
    monthly_limit = 1;

-- 更换AI伴侣名额
CREATE shop_item:ai_replacement SET 
    name = '更换AI伴侣名额',
    description = '更换一个现有的AI伴侣',
    item_type = 'AISlotExpansion',
    category = 'Function',
    price_hp = 10000,
    image_url = '/images/shop/ai_replacement.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true,
    monthly_limit = 1;

-- 限定剧情解锁
CREATE shop_item:exclusive_story SET 
    name = '限定剧情解锁',
    description = '解锁专属AI互动内容和故事',
    item_type = 'ExclusiveStory',
    category = 'Function',
    price_hp = 2000,
    image_url = '/images/shop/exclusive_story.png',
    is_limited = false,
    created_at = time::now(),
    visible = true,
    vip_discount = true;
