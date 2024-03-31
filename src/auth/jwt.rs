use actix_web::error::ErrorUnauthorized;
use actix_web::HttpMessage;
use actix_web::{dev::Payload, http, web, Error as ActixWebError, FromRequest, HttpRequest};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    user_id: Uuid,
    exp: usize,
    iat: usize,
}

pub async fn generate_token(user_id: Uuid, secret: &str) -> String {
    // Getting token claims
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = iat + 3600;

    let token_claims = TokenClaims { user_id, exp, iat };

    // Generating Token
    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    );
    token.unwrap()
}

fn decode_token(token: &str, secret: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub struct JWTMiddleware {
    pub user_id: Uuid,
}

impl FromRequest for JWTMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();
        let token = req
            .cookie("__tk_sess")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized("Please provide valid token")));
        }

        let claims = decode_token(&token.unwrap(), &data.secret).unwrap();
        let user_id: Uuid = claims.user_id;

        req.extensions_mut().insert(user_id.to_owned());
        ready(Ok(JWTMiddleware { user_id }))
    }
}
