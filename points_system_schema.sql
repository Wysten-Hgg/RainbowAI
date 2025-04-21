-- 更新User表，添加积分和货币相关字段
ALTER TABLE user ADD FIELD hp TYPE int DEFAULT 0;
ALTER TABLE user ADD FIELD lc_balance TYPE int DEFAULT 0;
ALTER TABLE user ADD FIELD daily_checkin_streak TYPE int DEFAULT 0;
ALTER TABLE user ADD FIELD last_checkin_date TYPE option<int>;
ALTER TABLE user ADD FIELD total_invites TYPE int DEFAULT 0;
ALTER TABLE user ADD FIELD invited_by TYPE option<string>;

-- 创建钱包交易记录表
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

-- 初始化一些基础商城商品
CREATE shop_item:ai_skin_1 SET 
    name = 'AI星空皮肤', 
    description = '为你的AI伴侣添加梦幻星空背景', 
    item_type = 'AIDecoration', 
    price_hp = 500, 
    image_url = '/images/shop/ai_skin_1.png', 
    is_limited = false, 
    created_at = time::now();

CREATE shop_item:user_title_1 SET 
    name = '彩虹城贵宾', 
    description = '专属称号，在社区中展示你的身份', 
    item_type = 'UserTitle', 
    price_hp = 1000, 
    image_url = '/images/shop/user_title_1.png', 
    is_limited = false, 
    created_at = time::now();

CREATE shop_item:lio_ticket SET 
    name = 'LIO访问券', 
    description = '获得额外的LIO互动机会', 
    item_type = 'LIOAccessTicket', 
    price_hp = 300, 
    image_url = '/images/shop/lio_ticket.png', 
    is_limited = false, 
    created_at = time::now();
