pub mod c;

use std::collections::HashMap;
use std::fs;
use std::io::Write;

use crate::status::Status;

type Generator = fn() -> &'static str;

pub fn registry() -> HashMap<&'static str, Generator> { 
    let registries: HashMap<&'static str, Generator> = HashMap::from([
        ("--c", c::generate as Generator),
    ]);
    return registries;
}

pub fn write_file(name: &str, language: &str) -> Result<(), Status> {
    let file = match fs::File::create_new(&name) {
        Ok(file) => Ok(file),
        Err(_) => return Err(Status::CreationFailure),
    };
    let generator = registry();
    let source = generator.get(&language).ok_or(Status::InvalidLanguage)?;
    match file?.write_all(source().as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => return Err(Status::CreationFailure),
    }
}
