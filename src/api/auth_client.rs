use crate::model::auth::Auth;
use crate::model::user::User;
use crate::error::error_handler::RequestError;

const URL: &str = "http://localhost:8080/login";


/*
* Creates reqwest http client that requests user information from simrep
* backend, if successfull returns information containing authentication token and email
*
* @param user: contains user email and password
* @return auth || RequestError: if succesfull return auth data, else returns a request error
*/
pub async fn auth_request(user: &User) -> Result<Auth, RequestError> {
    let json_data = serde_json::to_string(user)?;

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;

    if response.status() != 200 {
        println!("Status code: {}", response.status());
    }

    let response_body: String = response.text().await?;
    let auth: Auth = serde_json::from_str(response_body.as_str())?;
    Ok(auth)
}

