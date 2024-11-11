use crate::model::user::User;
use crate::model::auth::Auth;
use crate::error::error_handler::RequestError;

const URL: &str = "http:://localhost:8080/login";

/*
async fn auth_request(user: &User) -> Result<(), RequestError> {
    let json_data = serde_json::to_string(user)?;
}*/

