use serde::{Deserialize, Serialize};

/*
* FileNode implements Debug and Serialize
* for printing and serialization from the crate serde
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
}
