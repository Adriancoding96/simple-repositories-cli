use std::fs::File;
use std::io::Write;
use serde_json;
use std::path::PathBuf;
use dirs::home_dir;

use crate::model::commit::Commit;

/*
* Writes commit data to a commit file in home/simrep/commit directory
*
* @param commit: reference to commit struct containing commit data
*/ 
fn write_commit(commit: &Commit) -> std::io::Result<()> {
    let json = convert_commit_to_json(&commit);
    let commit_dir: PathBuf = locate_commit_dir()?;
    let mut file = File::create(commit_dir)?;
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
* Helper function to locate commit directory, if successfull returns path
* if not returns a error
*
* @return commit_dir || std::io::Error: returns path if successfull, else std::io::Error
*/
fn locate_commit_dir() -> std::io::Result<PathBuf> {
    let home_dir: PathBuf = home_dir().expect("Could not locate home directory");
    let commit_dir: PathBuf = home_dir.join("simrep").join("commit");
    if commit_dir.is_dir() {
        Ok(commit_dir)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory {:?} does not exisit", commit_dir),
        ))
    }
}

