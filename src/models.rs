use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use actix_web::web;

use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {
    pub async fn authenticate(
        email: &str,
        password: &[u8],
        data: web::Data<AppState>,
    ) -> Option<User> {
        let record = sqlx::query_as!(User, "SELECT * FROM users WHERE email=$1", email)
            .fetch_one(&data.db)
            .await;

        match record {
            Ok(user) => {
                let parsed_hash = PasswordHash::new(&user.password).unwrap();

                if Argon2::default()
                    .verify_password(password, &parsed_hash)
                    .is_ok()
                {
                    Some(user)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
