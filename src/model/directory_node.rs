use serde::Serialize;
use super::file_node::FileNode;

/*
* FileNode implements Debug and Serialize
* for printing and serialization from the crate serde
*/
#[derive(Debug, Serialize)]
pub struct DirectoryNode {
    pub name: String,
    pub path: String,
    pub files: Vec<FileNode>,
    pub subdirectories: Vec<DirectoryNode>,
}
