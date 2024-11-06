use simrep_cli::cli::parse_args;

fn main() {
    let args = parse_args();

    let path = args.path;
    let verbose = args.verbose;
    let output_format = args.output;

    //For debugging purposes
    println!("Path: {}", path);
    println!("Verbose: {}", verbose);
    println!("Output Format: {}", output_format);
}
