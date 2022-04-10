use std::env;
use std::fs;
use std::io;
use std::process;

struct Config {
    command: String,
    filename: String,
}

impl Config {
    // a &'static str will always be string literals that have the 'static lifetime.
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Not enough argmuents");
        }
        let command = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { command, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {}", err);
        process::exit(1);
    });

    if let Err(e) = do_command(config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}

fn do_command(config: Config) -> Result<(), io::Error> {
    let command = config.command;
    let filename = config.filename;
    // if command == "read" {
    //     let content = fs::read_to_string(filename).expect("Can't read a file");
    //     println!("Read content : \n{}", content);
    //     Ok(())
    // } else if command == "write" {
    //     let new_content = "This is a new content to write in the file";
    //     fs::write(filename, new_content).expect("Can't write a file.");
    //     println!("Written!");
    //     Ok(())
    // } else if command == "del" {
    //     fs::remove_file(filename)
    // } else {
    //     println!("No command found.");
    //     Ok(())
    // }

    match command.as_str() {
        "read" => {
            let content = fs::read_to_string(filename).expect("Can't read a file");
            println!("Read content : \n{}", content);
            Ok(())
        }
        "write" => {
            let new_content = "This is a new content to write in the file";
            fs::write(filename, new_content).expect("Can't write a file.");
            println!("Written!");
            Ok(())
        }
        "del" => fs::remove_file(filename),
        _ => {
            println!("No command found.");
            Ok(())
        }
    }
}

// fn parse_config(args: &[String]) -> Config {
//     // Trade-ff using clone
//     // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#the-trade-offs-of-using-clone
//     let command = args[1].clone();
//     let filename = args[2].clone();

//     Config { command, filename }
// }
