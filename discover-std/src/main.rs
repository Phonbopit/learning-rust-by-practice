fn sort_vec(mut vec: Vec<u32>) -> Vec<u32> {
    vec.sort();
    vec
}
fn main() {
    let vec = vec![9, 5, 1, 2, 12, 3, 4];

    let result = sort_vec(vec);
    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort_vec_test() {
        let result = sort_vec(vec![7, 5, 1, 2, 4, 3, 6]);
        assert_eq!(result, [1, 2, 3, 4, 5, 6, 7]);
    }
}
