mod cli;
mod fs_traversal;
mod core;
mod model;
mod output;
mod json_writer;
mod env;

use crate::cli::{parse_args, Commands};
use crate::fs_traversal::traverse_directory;
use crate::model::directory_node::DirectoryNode;
use crate::core::config::Config;
use crate::output::print::print_tree;
use crate::json_writer::directory_tree_writer::write_tree_to_file;
use crate::env::init_simrep::init_home_dir;

use std::path::Path;

fn main() -> std::io::Result<()> {
    let args = parse_args();
   
    let config = Config {
        command: args.command,
        output_format: args.output,
        verbose: args.verbose,
    };
    
    command(&config)?;
    Ok(())

}

fn command(config: &Config) -> std::io::Result<()> {
    match &config.command { 
        Commands::Origin { url } => {
            add_origin();
        }
        Commands::Add { path } => {
            create_tree_file(path)?;
        }
        Commands::Commit { message } => {
            create_commit();        
        }
        Commands::Push { hash } => { 
            push_commit();
        }
        Commands::Tree { path } => {
            print_tree_from_path(path);
        }
        Commands::Init { } => {
            initiate_env();
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

fn create_commit() {

}

fn push_commit() {

}

fn print_tree_from_path(path: &str) {
    let root_dir = create_tree_from_path(path);
    print_tree(&root_dir, 0);
}

fn create_tree_from_path(path: &str) -> DirectoryNode {
    let root_dir: DirectoryNode = traverse_directory(Path::new(&path))
        .expect("Failed to traverse directory tree");
    root_dir
}

fn initiate_env() -> std::io::Result<()> {
    init_home_dir()?;
    Ok(())
}
