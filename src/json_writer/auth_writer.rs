use std::fs::File;
use std::io::Write;
use serde_json;
use pwhash::bcrypt;

use crate::env::system_search::locate_dir;
use crate::model::user::User;


/*
* Writes auth information to auth directory, this is a temporary solution
* will implement safer user credential handling like secret-service/keyring.
*
* @param email_str: contains user email
* @param password: contains user password
*/
pub fn write_auth(email_str: &str, password: &str) -> std::io::Result<()> { 
    let encrypted_password = bcrypt::hash(password).expect("Failed to encrypt password");
    let email: String = email_str.to_owned();
    let user = User {
        email,
        password: encrypted_password,    
    };
    let json = convert_user_to_json(&user);

    let dir_path: Vec<&str> = vec!["simrep", ".auth"];
    let auth_dir = locate_dir(&dir_path)?;
    let file_path = auth_dir.join("auth.json");
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/*
* Converts user struct to json
*/
fn convert_user_to_json(user: &User) -> String {
    let json = serde_json::to_string_pretty(user)
        .expect("Failed to serialize commit to JSON");
    json
}

