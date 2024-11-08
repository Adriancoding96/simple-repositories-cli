use serde::{Deserialize, Serialize};
use crate::model::directory_node::DirectoryNode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    project_name: String,
    project_information: String,
    directories: Vec<DirectoryNode>,
}
