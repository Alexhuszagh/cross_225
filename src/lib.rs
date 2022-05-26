/// ```
/// use issue225::square;
/// assert_eq!(square(3), 9);
/// ```
pub fn square(x: u32) -> u32 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = square(2);
        assert_eq!(result, 4);
    }
}
