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
    Ok(())
}

/*
* Tries to create a env directory inside the simrep directory
*/
fn init_env_dir(simrep_dir: &PathBuf) -> std::io::Result<()> {
    let env_dir: PathBuf = simrep_dir.join("env");
    fs::create_dir_all(&env_dir)?;
    Ok(())
}

/*
* Tries to create a commit directory inside the simrep directory
*/
fn init_commit_dir(simrep_dir: &PathBuf) -> std::io::Result<()> {
    let commit_dir: PathBuf = simrep_dir.join("commit");
    fs::create_dir_all(&commit_dir)?;
    Ok(())
}
