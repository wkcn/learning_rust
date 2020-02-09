pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    // use super::add_two;
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert!(true);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        assert!(false);
    }

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(3), 3 + 2);
    }
}
