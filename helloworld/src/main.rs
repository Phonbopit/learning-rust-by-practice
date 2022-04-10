pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    println!("Hello, world!");
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
