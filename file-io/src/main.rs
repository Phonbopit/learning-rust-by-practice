use std::env;
use std::fs;

struct Config {
    command: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let command = args[1].clone();
        let filename = args[2].clone();

        Config { command, filename }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let config: Config = Config::new(&args);
    do_command(config);
}

fn do_command(config: Config) {
    let command = config.command;
    let filename = config.filename;
    if command == "read" {
        let content = fs::read_to_string(filename).expect("Can't read a file");
        println!("Read content : \n{}", content);
    } else if command == "write" {
        let new_content = "This is a new content to write in the file";
        fs::write(filename, new_content).expect("Can't write a file.");
        println!("Written!");
    } else if command == "del" {
        fs::remove_file(filename).expect("File can't delete.");
    }
}

// fn parse_config(args: &[String]) -> Config {
//     // Trade-ff using clone
//     // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#the-trade-offs-of-using-clone
//     let command = args[1].clone();
//     let filename = args[2].clone();

//     Config { command, filename }
// }
