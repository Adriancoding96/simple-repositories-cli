use std::path::PathBuf;
use dirs::home_dir;

/*
* Function to locate and return path to directory from path starting from home/user directory
* 
* @param dir_path: path is passed as a vector of strings, where each string is the next directory
* in path.
* @return expected_dir || std::io::Error: if successfull returns path to directory, else returns 
* i/o error.
*/
pub fn locate_dir(dir_path: &Vec<&str>) -> std::io::Result<PathBuf> {
    let home_dir: PathBuf = home_dir().expect("Could not locate [unix: home directory], [windows: user directory]");
    let mut expected_dir: PathBuf = home_dir;
    for dir in dir_path {
        expected_dir = expected_dir.join(dir); 
    }

    if expected_dir.is_dir() {
        Ok(expected_dir)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory {:?} does not exist", expected_dir),
        ))
    }
}

/*
* Function to located and return path to file starting from home/user directory
*
* @param file_path: vector of strings where each value represents the next node in path
* @return expected_file || std::io::Error: if successfull returns path to file, else
* return i/o error.
*/
pub fn locate_file(file_path: &Vec<&str>) -> std::io::Result<PathBuf> {
    let home_dir: PathBuf = home_dir().expect("Could not locate [unix: home directory], [windows: user directory]");
    let mut expected_file: PathBuf = home_dir;
    for node in file_path {
        expected_file = expected_file.join(node); 
    }

    if expected_file .is_file() {
        Ok(expected_file)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File {:?} does not exist", expected_file),
        ))
    }
}

/*
* Utility function to get env directory path
*/
pub fn get_env_path() -> std::io::Result<PathBuf> {
    let home_dir: PathBuf = home_dir().expect("Could not locate [unix: home directory], [windows: user directory]");
    home_dir.join("simrep").join("env");
    Ok(home_dir)
}

/*
* Utility function to get commit directory path
*/
pub fn get_commit_path() -> std::io::Result<PathBuf> {
    let home_dir: PathBuf = home_dir().expect("Could not locate [unix: home directory], [windows: user directory]");
    home_dir.join("simrep").join("commit");
    Ok(home_dir)
}
