use crate::languages;

pub fn help() {
    println!("Usage: boilgen[EXE] [OPTIONS/LANGUAGE] [FILENAME]                               ");
    println!("                                                                                ");
    println!("Options:                                                                        ");
    println!("  --help,         -h    Show help menu                                          ");
    println!("  --version,      -v    Show version menu                                       ");
    println!("  --lang,         -l    Show officially supported languages                     ");
    println!("  --custom-lang,  -cl   Show custom supported languages from user extension(s)  ");
}

pub fn lang() {
    println!("Officially supported languages:    ");

    let binding = languages::registry();
    let mut languages: Vec<_> = binding.keys().collect();
    languages.sort();
    
    for language in languages{
        println!("  {}", language);
    }
}

pub fn custom_lang() {
    println!("User-extended supported languages:  ");
    
    let binding = languages::custom::registry();
    let mut languages: Vec<_> = binding.keys().collect();
    languages.sort();
    
    for language in languages{
        println!("  {}", language);
    }
}

pub fn version() {
    println!("elixare v0.1.0");
    println!("(c) 2026 Nuno G. Permana");
    println!("Licensed under Apache License 2.0");
}
