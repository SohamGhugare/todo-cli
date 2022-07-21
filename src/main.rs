mod cli;
mod file_handler;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cli::parse_cli()
}
