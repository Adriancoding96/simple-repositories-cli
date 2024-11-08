use serde::{Deserialize, Serialize};
use crate::model::file_node::FileNode;

/*
* FileNode implements Debug and Serialize
* for printing and serialization from the crate serde
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryNode {
    pub name: String,
    pub path: String,
    pub files: Vec<FileNode>,
    pub subdirectories: Vec<DirectoryNode>,
}
