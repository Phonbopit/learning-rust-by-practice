use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let filename = &args[2];

    println!("Args: {:?}", args);

    if command == "read" {
        let content = fs::read_to_string(filename).expect("Can't read a file");
        println!("Read content : {}", content);
    } else if command == "write" {
        let new_content = "This is a new content to write in the file";
        fs::write(filename, new_content).expect("Can't write a file.");
        println!("Written!");
    } else if command == "del" {
        fs::remove_file(filename).expect("File can't delete.");
    }
}
