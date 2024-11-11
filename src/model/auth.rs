use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Auth {
    email: String,
    token: String,
}
