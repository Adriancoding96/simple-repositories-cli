use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {

    /*
    * Allow user to set verbose flag with -v or --verbose, for debugging capabilities
    */
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    #[arg(short, long, default_value = "text" )]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputFormat {
    Text,
    Json,
    File,
}

#[derive(Subcommand)]
pub enum Commands {
    Origin {
        #[arg(short, long, default_value = ".")]
        url: String,
    },
    Add {
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    Commit {
        #[arg(short, long)]
        message: String,
    },
    Push {
        hash: Option<String>,
    },
    Tree {
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    Init {
    }
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

