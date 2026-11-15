pub fn help() {
    println!("Usage: boilgen[EXE] [OPTIONS/LANGUAGE] [FILENAME]                ");
    println!("                                                                 ");
    println!("Options:                                                         ");
    println!("  --help          Show help menu                                 ");
    println!("  --version       Show version menu                              ");
    println!("  --lang          Show officiallysupported languages             ");
    println!("  --custom-lang   Show custom supported languages from the cofig ");
}

pub fn lang() {
    println!("Officially supported languages:    ");
    println!("  --c                              ");
    println!("  --cpp                            ");
}

pub fn version() {
    println!("boilgen v0.1.0");
}
