mod languages;
mod miscellaneous;
mod run;
mod status;

use std::fs;
use std::env;
use std::io::Write;
use std::process::{ExitCode, Termination};

// fn create_c(name: &String) -> status::Status {
//     let file_c = match fs::File::create_new(name) {
//         Ok(file_c) => file_c,
//         Err(_e) => return status::Status::CreationFailure,
//     };
//
//     let _ = match writeln!(
//         &file_c, 
//         concat!(
//             r#"#include <stdio.h>                  "#, "\n\n",
//             r#"int main(int argc, char *argv[]) {{ "#, "\n",
//             r#"    printf(\"Hello world!\");       "#, "\n",
//             r#"    return 0;                       "#, "\n",
//             r#"}}                                  "#
//         )
//     ) {
//         Ok(_) => (),
//         Err(_e) => return status::Status::WriteFailure,
//     };
// }

fn main() -> ExitCode {
    match run::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(status) => ExitCode::from(status as u8),
    }
}
