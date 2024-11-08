use serde::{Deserialize, Serialize};
use crate::model::branch::Branch;
use crate::model::directory_node::DirectoryNode;
use crate::model::file_node::FileNode;
use crate::model::project::Project;

/*
* Content enum to make content in Commit struct to be able
* to accept either Project, DirectoryNode, or FileNode
*/
#[derive(Debug, Serialize, Deserialize)]
enum Content {
    Project(Project),
    Directory(DirectoryNode),
    File(FileNode),
}

#[derive(Debug, Serialize)]
pub struct Commit {
    commit_message: String,
    owner_email: String,
    branch: Branch,
    content: Content, 
}

impl Commit {
    
    pub fn new_project_commit(project: Project, commit_message: String, owner_email: String, branch: Branch) -> Self {
        Self {
            commit_message,
            owner_email,
            branch,
            content: Content::Project(project),
        }
    }

     pub fn new_directory_commit(directory: DirectoryNode, commit_message: String, owner_email: String, branch: Branch) -> Self {
        Self {
            commit_message,
            owner_email,
            branch,
            content: Content::Directory(directory),
        } 
    }

    pub fn new_file_commit(file: FileNode, commit_message: String, owner_email: String, branch: Branch) -> Self {
        Self {
            commit_message,
            owner_email,
            branch,
            content: Content::File(file),
        }               
    }

}
