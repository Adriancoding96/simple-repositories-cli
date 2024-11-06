use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {

    /*
    * Allows user to pass argument with short flag -p, and long flag --path.
    * If no argument is passed it defaults to '.'
    */
    #[arg(short, long, default_value = ".")]
    pub path: String,
    
    /*
    * Allow user to set verbose flag with -v or --verbose, for debugging capabilities
    */
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    #[arg(short, long, default_value="no output specified")]
    pub output: String,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

