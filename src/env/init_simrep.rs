use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

/*
* Checks if home directory exists on system, if it does it tries to create
* a new directory called simrep.
*/
pub fn init_home_dir() -> std::io::Result<()> {
    let home_dir: PathBuf = home_dir().expect("Could not located home directory");
    let simrep_dir: PathBuf = home_dir.join("simrep");
    fs::create_dir_all(&simrep_dir)?;
    init_env_dir(&simrep_dir)?;
    init_commit_dir(&simrep_dir)?;
    init_auth_dir(&simrep_dir)?;
    Ok(())
}

/*
* Attempts to create a env directory inside the simrep directory
*/
fn init_env_dir(simrep_dir: &PathBuf) -> std::io::Result<()> {
    let env_dir: PathBuf = simrep_dir.join("env");
    fs::create_dir_all(&env_dir)?;
    Ok(())
}

/*
* Attempts to create a commit directory inside the simrep directory
*/
fn init_commit_dir(simrep_dir: &PathBuf) -> std::io::Result<()> {
    let commit_dir: PathBuf = simrep_dir.join("commit");
    fs::create_dir_all(&commit_dir)?;
    Ok(())
}

/*
* Initiates location for hidden auth directroy, calls one of two functions that creates the directory
* based on operating system of user.
*/
fn init_auth_dir(simrep_dir: &PathBuf) -> std::io::Result<()> {
    let auth_dir: PathBuf = simrep_dir.join(".auth");
    
    #[cfg(unix)]
    init_auth_dir_unix(&auth_dir)?;

    #[cfg(windows)]
    init_auth_dir_windows(&auth_dir)?;

    Ok(())
}

/*
* Attempts to create hidden auth directory on unix machine 
*
* @param auth_dir: contains path for atuh directory to be created in
*/
#[cfg(unix)]
fn init_auth_dir_unix(auth_dir: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(auth_dir)?;
    Ok(())
}

/*
* TODO Test on windows machine with code outcomented in function
* 
*
* Attempts to create auth directory on windows machine, currently not hidden
* because i need to test the out commented code on a windows machine.
*
* @param auth_dir: contains path for auth directory to be created in 
*/
#[cfg(windows)]
fn init_auth_dir_windows(auth_dir: &PathBuf) -> std::io::Result<()> {
 
    fs::create_dir_all(auth_dir)?;
    Ok(())


    /*
    * Code to create a hidden folder on windows, need to test on a windows machine
    *
    use std::{fs::OpenOptions, os::windows::fs::MetadataExt}; 
    use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
    use std::os::windows::ffi::OsStrExt;
    use std::fs::OpenOptions;
    use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;
    use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;
    use winapi::um::fileapi::SetFileAttributesW;

    //Creates authentication directory
    let _file = OpenOptions::new()
        .write(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(&auth_dir)?;

    //Gets the current attributes of directory, changing hidden attribute to true
    let metadata: MetadataExt = fs::metadata(&auth_dir)?;
    let attributes = metadata.file_attributes();
    let hidden_attr = attributes | FILE_ATTRIBUTE_HIDDEN;

    //Converts Rusts standard OsStr to a wide String required by windows
    let path_wide: Vec<u16> = auth_dir
        .as_os_str()
        .encode_wide()
        .chain(some(0))
        .collect();

    //Calls c function to set directory attributes, unsafe because function not handled by Rust
    //compiler
    let result = unsafe { SetFileAttributesW(path_wide.as_ptr(), hidden_attr) };
    if result == 0 {
        Err(io::Error::last_os_error());
    }
    Ok(())
*/
}


