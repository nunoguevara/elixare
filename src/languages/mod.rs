pub mod custom;

pub mod c;
pub mod cpp;
pub mod go;
pub mod java;
pub mod java21;
pub mod kotlin;
pub mod odin;
pub mod python;
pub mod rust;

use std::collections::HashMap;
use std::fs;
use std::io::Write;

use crate::status::Status;

type Generator = fn() -> &'static str;

pub fn registry() -> HashMap<&'static str, Generator> { 
    HashMap::from([
        ("--c", c::generate as Generator),
        ("--cpp", cpp::generate),
        ("--go", go::generate),
        ("--java", java::generate),
        ("--java21", java21::generate),
        ("--kotlin", kotlin::generate),
        ("--python", python::generate),
        ("--odin", odin::generate),
        ("--rust", rust::generate),
    ])
}

pub fn get_generator(language: &str) -> Result<Generator, Status> {
    if let Some(generator) = custom::registry().get(language) {
        return Ok(*generator);
    }

    if let Some(generator) = registry().get(language) {
        return Ok(*generator);
    }

    Err(Status::InvalidLanguage)
}

pub fn write_file(name: &str, language: &str) -> Result<(), Status> {
    let source = get_generator(&language)?;

    let file = match fs::File::create_new(&name) {
        Ok(file) => Ok(file),
        Err(_) => return Err(Status::CreationFailure),
    
    };

    match file?.write_all(source().as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => return Err(Status::WriteFailure),
    }
}
