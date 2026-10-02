use super::*;

#[test]
fn test_main_compiles() {
    // Just a smoke test to ensure main module compiles
    assert!(true);
}


#[cfg(test)]
#[path = "tests.rs"]
mod tests;
