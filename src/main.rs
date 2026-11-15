mod languages;
mod miscellaneous;
mod run;
mod status;

use std::process::ExitCode;

use crate::status::Status;

fn main() -> ExitCode {
    match run::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Status::InsufficientArgs) => {
            eprintln!("ERROR: Please specify an argument or use --help.");
            ExitCode::from(Status::InsufficientArgs as u8)
        },
        Err(Status::InvalidLanguage) => {
            eprintln!("ERROR: Not an available language, use --language to display available languages");
            ExitCode::from(Status::InvalidLanguage as u8)
        },
        Err(Status::NoName) => {
            eprintln!("ERROR: Please specify a file name.");
            ExitCode::from(Status::NoName as u8)
        },
        Err(Status::CreationFailure) => {
            eprintln!("ERROR: Failed to create the file, either permission denied or file already exists.");
            ExitCode::from(Status::CreationFailure as u8)
        },
        Err(Status::WriteFailure) => {
            eprintln!("ERROR: Failed to write on the file.");
            ExitCode::from(Status::WriteFailure as u8)
        },
    }
}
