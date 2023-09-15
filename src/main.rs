#[mutants::skip]
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_should_not_panic() {
        main();
    }
}
