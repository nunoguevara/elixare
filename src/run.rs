use crate::miscellaneous;
use crate::status::Status;

use std::env;
use std::process::{ExitCode};

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

    let language: &String = args.get(1).ok_or(Status::InsufficientArgs)?;
    let file = args.get(2).ok_or(Status::NoName)?;

    return Ok(());
}
