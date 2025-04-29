pub fn greet() -> &'static str {
    // Replace the following line to return "Hello, world!"
    // Then the test below will pass.
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, world!");
    }
}
