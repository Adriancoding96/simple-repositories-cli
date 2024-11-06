use simrep_cli::cli::{parse_args, Commands};

fn main() {
    let args = parse_args();

    let verbose = args.verbose;
    let output_format = args.output;

    match args.command {
        Commands::List { path } => {
            println!("Listing directory: {}", path);
        }
        Commands::Tree { path } => {
            println!("Listing tree for directory: {}", path);
        }
    }
}
