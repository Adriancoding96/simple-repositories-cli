use std::fs::File;
use std::io::Write;
use serde_json;

use crate::model::branch::Branch;
use crate::env::system_search::locate_dir;


/*
* Writes branch information to file inside env directory
*
* @param branch: reference to branch struct containing branch information
*/
pub fn write_branch(branch: &Branch) -> std::io::Result<()> {
    let json: String = convert_branch_to_json(&branch);
    let dir_path: Vec<&str> = vec!["simrep", "env"];
    let env_dir = locate_dir(&dir_path)?;
    let file_path = env_dir.join("branch.json");
    let mut file = File::create(&file_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

/*
* Helper function to convert branch struct to JSON
*
* @param branc: reference to branch struct containing branch information
*/
fn convert_branch_to_json(branch: &Branch) -> String {
    let json = serde_json::to_string_pretty(branch)
        .expect("Failed to serialize commit to JSON");
    json
}
