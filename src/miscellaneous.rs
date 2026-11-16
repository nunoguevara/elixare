use crate::languages;

pub fn help() {
    println!("Usage: boilgen[EXE] [OPTIONS/LANGUAGE] [FILENAME]                     ");
    println!("                                                                      ");
    println!("Options:                                                              ");
    println!("  --help          Show help menu                                      ");
    println!("  --version       Show version menu                                   ");
    println!("  --lang          Show officiallysupported languages                  ");
    println!("  --custom-lang   Show custom supported languages from user extension ");
}

pub fn lang() {
    println!("Officially supported languages:    ");
    for languages in languages::registry().keys() {
        println!("  {}", languages);
    }
}

pub fn custom_lang() {
    println!("User-extended supported languages:  ");
    for languages in languages::custom::registry().keys() {
        println!("  {}", languages);
    }
}

pub fn version() {
    println!("boilgen v0.1.0");
}
