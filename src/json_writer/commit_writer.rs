use std::fs::File;
use std::io::Write;
use serde_json;
use uuid::Uuid;

use crate::model::commit::Commit;
use crate::env::system_search::locate_dir;

 


/*
* Writes commit data to a commit file in home/simrep/commit directory
*
* @param commit: reference to commit struct containing commit data
*/ 
pub fn write_commit(commit: &Commit) -> std::io::Result<()> {
    let json = convert_commit_to_json(&commit);
    let dir_path: Vec<&str> = vec!["simrep", "commit"];
    let commit_dir = locate_dir(&dir_path)?;
    let file_name = generate_file_name();
    let file_path = commit_dir.join(file_name);
    let mut file = File::create(&file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/*
* Converts commit struct to JSON
*
* @param commit: reference to a commit struct containing commit data
*/
fn convert_commit_to_json(commit: &Commit) -> String {
    let json = serde_json::to_string_pretty(commit)
        .expect("Failed to serialize commit to JSON");
    json
}

/*
* Generates file name with a randomised uuid
*
* @return file_name: returns a file name as a String
*/
fn generate_file_name() -> String {
    let uuid: Uuid = Uuid::new_v4();
    let mut file_name: String = uuid.to_string();
    file_name.push_str(".json");
    file_name
}
