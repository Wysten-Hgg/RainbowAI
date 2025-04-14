use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, errors::Error};
use serde::{Serialize, Deserialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub exp: i64,     // expiration time
    pub iat: i64,     // issued at
    pub refresh: bool, // 是否为refresh token
}

impl Claims {
    pub fn new(user_id: String, is_refresh: bool) -> Self {
        let now = OffsetDateTime::now_utc();
        let duration = if is_refresh {
            Duration::days(30)  // refresh token 30天有效
        } else {
            Duration::hours(1)  // access token 1小时有效
        };
        
        Self {
            sub: user_id,
            iat: now.unix_timestamp(),
            exp: (now + duration).unix_timestamp(),
            refresh: is_refresh,
        }
    }
}

const JWT_SECRET: &[u8] = b"rainbow_ai_secret";
const REFRESH_SECRET: &[u8] = b"rainbow_ai_refresh_secret";

pub fn create_token_pair(user_id: &str) -> Result<(String, String), Error> {
    let access_claims = Claims::new(user_id.to_string(), false);
    let refresh_claims = Claims::new(user_id.to_string(), true);

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )?;

    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(REFRESH_SECRET),
    )?;

    Ok((access_token, refresh_token))
}

pub fn verify_access_token(token: &str) -> Result<Claims, Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
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
        &DecodingKey::from_secret(REFRESH_SECRET),
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
        &EncodingKey::from_secret(JWT_SECRET),
    )
}
