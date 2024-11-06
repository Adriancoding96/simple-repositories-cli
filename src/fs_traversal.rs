use std::path::Path;
use::std::fs;

use crate::model::directory_node::DirectoryNode;
use crate::model::file_node::FileNode;

/*
* traverse_directory walks through the file system from a path passed in
* as a input parameter, constructs a file tree containing directory and file nodes.
*
* @param path: contains path reference to current directory in file system
* @return directory_node || std::io::Error: if successfull returns fully constructed directory
* node, if not successful returns a I/O error. 
*/
pub fn traverse_directory(path: &Path) -> Result<DirectoryNode, std::io::Error> {
    
    /*
    * Initializes starting directory with current directory,
    * borrows value of input parameter path, creates empty
    * vector for files and subdirectories.
    */
   
    let mut directory_node = DirectoryNode {
        name: path.file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("")
            .to_string(),
        path: path.to_string_lossy().to_string(),
        files: Vec::new(),
        subdirectories: Vec::new(),
    };

    for entry_result in fs::read_dir(path)? { //? propegates errors
        let entry = entry_result?;
        let entry_path = entry.path();
        
        /*
        * Checks if path is a directory, if true recursivly call traverse_directory by
        * passing path as a reference, match handles result based on the returned value,
        * if ok push subdirectory to directory vector, if not print a I/O Error.
        *
        * If not a directory it is a file, then constructs a file node and 
        * and pushes it to file vector of directory
        */
        if entry_path.is_dir() {
            match traverse_directory(&entry_path) {
                Ok(sub_dir_node) => {
                    directory_node.subdirectories.push(sub_dir_node);
                }
                Err(e) => {
                    eprintln!("Error reading directory {:?}: {}", entry_path, e);
                }
            }
        } else if entry_path.is_file() {
            match entry.metadata() {
                Ok(metadata) => {
                    let file_node = FileNode {
                        name: entry.file_name().to_string_lossy().to_string(),
                        path: entry_path.to_string_lossy().to_string(),
                        size: metadata.len(),
                    };
                    directory_node.files.push(file_node);
                }
                Err(e) => {
                    eprintln!("Error reading file metadata: {:?}: {}", entry_path, e);
                }
            }
        }
        
    }
    Ok(directory_node)
}
