use chrono;
use std::usize;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMeSchema {
    /// This schema used for getting current user
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    /// This schema used for creating user and fields are mentioned field in body
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub page: Option<usize>,
}
