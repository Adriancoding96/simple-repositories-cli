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
    
    if config.verbose {
        start_debug(&config);
    } else {
        start(&config);
    }
                
}

fn start(config: &Config) {
    match &config.command {
        Commands::List { path } => {
            let root_dir: DirectoryNode = traverse_directory(Path::new(&path))
                .expect("Failed to traverse directory");
            println!("No need to print a list structure");            //Output result
        }
        Commands::Tree { path } => {
            let root_dir: DirectoryNode = traverse_directory(Path::new(&path))
                .expect("Failed to traverse directory");
            print_tree(&root_dir, 0);
        }
    }
}

fn start_debug(config: &Config) {
    println!("Debug mode enabled");
    match &config.command {
        Commands::List { path } => {
            println!("Executing 'List' command on path: {}", path);
            let root_dir: DirectoryNode = traverse_directory(Path::new(&path))
                .expect("Failed to traverse directory");
            println!("No need to print a list structure");
        }
        Commands::Tree { path } => {
            println!("Executing 'Tree' command on path: {}", path);
            let root_dir: DirectoryNode = traverse_directory(Path::new(&path))
                .expect("Failed to traverse directory");
            print_tree(&root_dir, 0);
        }
    }
}
