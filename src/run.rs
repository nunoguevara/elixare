use std::env;

use crate::languages;
use crate::miscellaneous;
use crate::status::Status;

pub fn run() -> Result<(), Status> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        miscellaneous::help();
        return Ok(());
    }

    if args[1] == "--version" {
        miscellaneous::version();
        return Ok(());
    }

    let language: &str = args.get(1).ok_or(Status::InsufficientArgs)?;
    let filename: &str = args.get(2).ok_or(Status::NoName)?;
    
    languages::write_file(&filename, &language)?;
    return Ok(());
}
