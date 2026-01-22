use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // will change as more functionality is added
    if args.len() != 3 {
        println!("Usage: minigrep <query> <file_path>");
        return;
    }

    // initial implementation of argument parsing, will refactor
    // let query = &args[1];
    let file_path = &args[2];

    // print!("Searching for {}", query);
    // println!(" in file {}", file_path);

    let content = fs::read_to_string(file_path)
        .expect("File not readable");

    println!("File content: \n\n{}", content);
}
