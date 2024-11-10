use crate::model::commit::Commit;
use crate::model::branch::Branch;
use crate::model::user::User;
use crate::model::directory_node::DirectoryNode;
use crate::model::file_node::FileNode;
use crate::json_writer::commit_writer::write_commit;
use crate::env::system_search::get_file_value;

/*
* Initiates commit, gets data from data.json file in env directory,
* determines if value represents a directory or file by checking if it contains
* the word "files".
*
* @param commit_message: contains message for the commit
* @param branch: contains branch information for commit
*/
pub fn initiate_commit(commit_message: &str) -> std::io::Result<()> {
    let commit_value: serde_json::Value = get_file_value(&vec!["simrep", "env", "data.json"])?; 
    let branch_value: serde_json::Value = get_file_value(&vec!["simrep", "env", "branch.json"])?; 
    let user_value: serde_json::Value = get_file_value(&vec!["simrep", ".auth", "auth.json"])?; 
    if commit_value.get("files").is_some() {
        create_directory_commit(commit_value, commit_message, user_value, branch_value)?;
        Ok(())
    
    } else {
        create_file_commit(commit_value, commit_message, user_value, branch_value)?;
        Ok(())
    }
}

/*
* Creates commit containing a directory and passes it commit_writer to save on system
*
* @param commit_message: contains message for the commit
* @param user_value: contains user information
* @param branch: contains branch information for commit
*/
fn create_directory_commit(commit_value: serde_json::Value, message: &str,
        user_value: serde_json::Value, branch_value: serde_json::Value) -> std::io::Result<()> {

    let directory: DirectoryNode = serde_json::from_value(commit_value)?;
    let user: User = serde_json::from_value(user_value)?;
    let branch: Branch = serde_json::from_value(branch_value)?;
    let commit: Commit = Commit::new_directory_commit(
        directory, message.to_owned(), user.email, branch
    );
    write_commit(&commit)?; 
    Ok(())
}

/*
* Creates commit containing a file and passes it commit_writer to save on system
*
* @param commit_message: contains message for the commit
* @param user_value: conatain user information
* @param branch: contains branch information for commit
*/
fn create_file_commit(commit_value: serde_json::Value, message: &str,
        user_value: serde_json::Value, branch_value: serde_json::Value) -> std::io::Result<()> {

    let file: FileNode = serde_json::from_value(commit_value)?;
    let user: User = serde_json::from_value(user_value)?;
    let branch: Branch = serde_json::from_value(branch_value)?;
    let commit: Commit = Commit::new_file_commit(
        file, message.to_owned(), user.email, branch
    );
    write_commit(&commit)?;
    Ok(())
}
