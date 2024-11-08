use std::fs::File;
use std::path::PathBuf;

use crate::model::commit::Commit;
use crate::model::branch::Branch;
use crate::env::system_search::locate_file;
use crate::model::directory_node::DirectoryNode;
use crate::model::file_node::FileNode;
use crate::json_writer::commit_writer::write_commit;

//TODO Implement getting user information from simrep.auth/user_information.json
/*
* Initiates commit, gets data from data.json file in env directory,
* determines if value represents a directory or file by checking if it contains
* the word "files".
*
* @param commit_message: contains message for the commit
* @param email: contains email of the application user
* @param branch: contains branch information for commit
*/
pub fn initiate_commit(commit_message: String, email: String, branch: Branch) -> std::io::Result<()> {
    let value: serde_json::Value = get_file_value()?; 
    if value.get("files").is_some() {
        create_directory_commit(value, commit_message, email, branch)?;
        Ok(())
    
    } else {
        create_file_commit(value, commit_message, email, branch)?;
        Ok(())
    }
}

/*
* Gets data from data.json file in env directory
*
* @return value || std::io::Error: if successfull returns data from data.json file, else
* return i/o error.
*/
fn get_file_value() -> std::io::Result<serde_json::Value> {
    let path: Vec<&str> = vec!["simrep", "env", "data.json"]; 
    let file_path: PathBuf = locate_file(&path)?;
    let file: File = File::open(&file_path)?;
    let value: serde_json::Value = serde_json::from_reader(file)?;
    Ok(value)
}

/*
* Creates commit containing a directory and passes it commit_writer to save on system
*
* @param commit_message: contains message for the commit
* @param email: contains email of the application user
* @param branch: contains branch information for commit
*/
fn create_directory_commit(value: serde_json::Value, message: String, email: String, branch: Branch) -> std::io::Result<()> {
    let directory: DirectoryNode = serde_json::from_value(value)?;
    let commit: Commit = Commit::new_directory_commit(
        directory, message, email, branch
    );
    write_commit(&commit)?; 
    Ok(())
}

/*
* Creates commit containing a file and passes it commit_writer to save on system
*
* @param commit_message: contains message for the commit
* @param email: contains email of the application user
* @param branch: contains branch information for commit
*/
fn create_file_commit(value: serde_json::Value, message: String, email: String, branch: Branch) -> std::io::Result<()> {
    let file: FileNode = serde_json::from_value(value)?;
    let commit: Commit = Commit::new_file_commit(
        file, message, email, branch
    );
    write_commit(&commit)?;
    Ok(())
}
