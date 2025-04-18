use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, errors::Error};
use serde::{Serialize, Deserialize};
use time::{Duration, OffsetDateTime};
use std::env;
use once_cell::sync::Lazy;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub exp: i64,     // expiration time
    pub iat: i64,     // issued at
    pub refresh: bool, // 是否为refresh token
}

// 使用Lazy静态变量读取环境变量，确保只读取一次
static JWT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "rainbow_ai_secret".to_string())
        .into_bytes()
});

static REFRESH_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "rainbow_ai_secret".to_string());
    format!("{}_refresh", secret).into_bytes()
});

static JWT_EXPIRATION: Lazy<i64> = Lazy::new(|| {
    env::var("JWT_EXPIRATION")
        .ok()
        .and_then(|val| val.parse::<i64>().ok())
        .unwrap_or(3600) // 默认1小时
});

impl Claims {
    pub fn new(user_id: String, is_refresh: bool) -> Self {
        let now = OffsetDateTime::now_utc();
        let duration = if is_refresh {
            Duration::days(30)  // refresh token 30天有效
        } else {
            Duration::seconds(*JWT_EXPIRATION)  // 使用环境变量配置的过期时间
        };
        
        Self {
            sub: user_id,
            iat: now.unix_timestamp(),
            exp: (now + duration).unix_timestamp(),
            refresh: is_refresh,
        }
    }
}

pub fn create_token(user_id: &str) -> Result<String, Error> {
    let claims = Claims::new(user_id.to_string(), false);
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&JWT_SECRET),
    )
}

pub fn create_token_pair(user_id: &str) -> Result<(String, String), Error> {
    let access_claims = Claims::new(user_id.to_string(), false);
    let refresh_claims = Claims::new(user_id.to_string(), true);

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(&JWT_SECRET),
    )?;

    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(&REFRESH_SECRET),
    )?;

    Ok((access_token, refresh_token))
}

pub fn verify_access_token(token: &str) -> Result<Claims, Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&JWT_SECRET),
        &Validation::default(),
    )?;

    if claims.claims.refresh {
        return Err(Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
    }

    Ok(claims.claims)
}

pub fn verify_refresh_token(token: &str) -> Result<Claims, Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&REFRESH_SECRET),
        &Validation::default(),
    )?;

    if !claims.claims.refresh {
        return Err(Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
    }

    Ok(claims.claims)
}

pub fn refresh_access_token(refresh_token: &str) -> Result<String, Error> {
    let claims = verify_refresh_token(refresh_token)?;
    let access_claims = Claims::new(claims.sub, false);
    
    encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(&JWT_SECRET),
    )
}
