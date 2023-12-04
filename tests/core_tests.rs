
fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[should_panic(expected="assertion failed")]
    fn test_add() {
        assert_eq!(add(1, 2), 2);
    }
}