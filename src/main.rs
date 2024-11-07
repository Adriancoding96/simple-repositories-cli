mod cli;
mod fs_traversal;
mod core;
mod model;
mod output;

use crate::cli::{parse_args, Commands};
use crate::fs_traversal::traverse_directory;
use crate::model::directory_node::DirectoryNode;
use crate::core::config::Config;
use crate::output::print::print_tree;

use std::path::Path;

fn main() {
    let args = parse_args();
   
    let config = Config {
        command: args.command,
        output_format: args.output,
        verbose: args.verbose,
    };
    
    command(&config);

}

fn command(config: &Config) {
    match &config.command { 
        Commands::Origin { path } => {
            add_origin();
        }
        Commands::Add { path } => {
            create_tree_file();
        }
        Commands::Commit { path } => {
            create_commit();        
        }
        Commands::Push { path } => { 
            push_commit();
        }
        Commands::Tree { path } => {
            print_tree_from_path(path);
        }
    }
}

fn add_origin() {

}

fn create_tree_file() {

}

fn create_commit() {

}

fn push_commit() {

}

fn print_tree_from_path(path: &String) {
    let root_dir: DirectoryNode = traverse_directory(Path::new(&path))
        .expect("Failed to traverse directory tree");
    print_tree(&root_dir, 0);
}

