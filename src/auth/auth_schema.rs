use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}
