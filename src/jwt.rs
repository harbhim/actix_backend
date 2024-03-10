use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct TokenClaims {
    user_id: Uuid,
    exp: usize,
    iat: usize,
}

pub async fn generate_token(user_id: Uuid) -> String {
    // Getting token claims
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = iat + 3600;

    let token_claims = TokenClaims { user_id, exp, iat };

    // Generating Token
    let secret = dotenvy::var("JWT_SECRET_KEY").unwrap();
    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    );
    token.unwrap()
}
