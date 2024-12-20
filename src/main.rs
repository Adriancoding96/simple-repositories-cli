mod cli;
mod fs_traversal;
mod core;
mod model;
mod output;
mod json_writer;
mod env;
mod error;
mod api;

use crate::cli::{parse_args, Commands};
use crate::fs_traversal::traverse_directory;
use crate::model::directory_node::DirectoryNode;
use crate::model::user::User;
use crate::model::auth::Auth;
use crate::model::branch::Branch;
use crate::core::config::Config;
use crate::output::print::print_tree;
use crate::json_writer::directory_tree_writer::write_tree_to_file;
use crate::json_writer::auth_writer::write_auth;
use crate::json_writer::branch_writer::write_branch;
use crate::env::init_simrep::init_home_dir;
use crate::error::error_handler::RequestError;
use crate::api::auth_client::auth_request;

use core::commit_builder::initiate_commit;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), RequestError> {
    let args = parse_args();
   
    let config = Config {
        command: args.command,
        output_format: args.output,
        verbose: args.verbose,
    };
    
    command(&config).await?;
    Ok(())

}

async fn command(config: &Config) -> Result<(), RequestError> {
    match &config.command { 
        Commands::Origin { url } => {
            add_origin();
        }
        Commands::Add { path } => {
            create_tree_file(path)?;
        }
        Commands::Commit { message } => {
            create_commit(message);        
        }
        Commands::Push { email, password } => { 
            push_commit(email, password).await?;
        }
        Commands::Tree { path } => {
            print_tree_from_path(path);
        }
        Commands::Init { } => {
            initiate_env();
        }
        Commands::Auth { email, password } => {
            auth(email, password);
        }
        Commands::Branch { branch } => {
            set_branch(branch);
        }
    
    }
    Ok(())
}

fn add_origin() {

}

fn create_tree_file(path: &str) -> std::io::Result<()>{
    let root_dir = create_tree_from_path(path);
    write_tree_to_file(&root_dir)?;
    Ok(())
}

fn create_commit(message: &str) -> std::io::Result<()>{
    initiate_commit(message)?;
    Ok(())
}

async fn push_commit(email: &str, password: &str) -> Result<(), RequestError> {
    println!("Sending auth request");
    let user: User = User {
        email: email.to_owned(),
        password: password.to_owned(),
    };
    let auth: Auth = auth_request(&user).await?; 
    println!("{:?}", auth);
    Ok(())
}

fn print_tree_from_path(path: &str) {
    let root_dir = create_tree_from_path(path);
    print_tree(&root_dir, 0);
}

fn create_tree_from_path(path: &str) -> DirectoryNode {
    let root_dir: DirectoryNode = traverse_directory(Path::new(&path), 0)
        .expect("Failed to traverse directory tree");
    root_dir
}

fn initiate_env() -> std::io::Result<()> {
    init_home_dir()?;
    Ok(())
}

fn auth(email: &str, password: &str) -> std::io::Result<()> {
    write_auth(email, password)?;
    Ok(())
}

fn set_branch(branch: &str) -> std::io::Result<()> {
    let branch: Branch = Branch {
        branch_name: branch.to_owned(),
    };
    write_branch(&branch)?;
    Ok(())
}
