use std::io::stdin;

pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    println!("Hello, world!");
    let result = 2 + 3;
    let name = String::from("Chuck Norris");

    let formatted = format!("Hello {}", name);
    println!("name {}", formatted);
    println!("Result {:?}", result);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read input");

    if input == "5" {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
    println!("{:#?}", input);

    if input.trim() == "5" {
        println!("trim() Correct!");
    } else {
        println!("trim() incorrect!");
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_add() {
        let sum = add(5, 10);
        assert_eq!(sum, 15);
    }
}
