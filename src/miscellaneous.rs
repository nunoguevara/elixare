pub fn help() {
    println!("Usage: boilgen[EXE] [OPTIONS/LANGUAGE] [FILENAME]");
    println!("                                                 ");
    println!("Options:                                         ");
    println!("  --help          Show help menu                 ");
    println!("  --version       Show version menu              ");
    println!("                                                 ");
    println!("Languages:                                       ");
    println!("  --c             Create a boilerplate for C     ");
    println!("  --cpp           Create a boilerplate for C++   ");
}

pub fn version() {
    println!("boilgen v0.1.0");
}
