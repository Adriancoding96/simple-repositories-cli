use crate::cli::{Commands, OutputFormat};

pub struct Config {
    pub command: Commands,
    pub output_format: OutputFormat,
    pub verbose: bool,
}
