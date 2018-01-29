/// This function returns true if the left-hand integer
/// is divisible by the right-hand integer.
///
/// # Description
///
/// * `lh` - Left-hand side integer.
/// * `rh` - Right-hand side integer.
pub fn is_divisible_by(lh: i32, rh: i32) -> bool {
    if rh == 0 {
        return true
    }

    lh % rh == 0
}

#[test]
fn test_is_divisible_by() {
    assert_eq!(is_divisible_by(4, 2), true);
}

/// Takes in an integer and returns the correct
/// fizzbuzz string.
pub fn fizzbuzz(n: i32) -> String {
    if is_divisible_by(n, 15) {
        "fizzbuzz".to_string()
    } else if is_divisible_by(n, 3) {
        "fizz".to_string()
    } else if is_divisible_by(n, 5) {
        "buzz".to_string()
    } else {
        format!("{}", n)
    }
}

pub fn do_fizzbuzz(n: i32) {
    for i in 0..n {
        println!("{}", i);
    }
}

#[test]
fn test_fizzbuzz_fizz() {
    assert_eq!(fizzbuzz(3), "fizz");
}

#[test]
fn test_fizzbuzz_buzz() {
    assert_eq!(fizzbuzz(5), "buzz");
}

#[test]
fn test_fizzbuzz_fizzbuzz() {
    assert_eq!(fizzbuzz(15), "fizzbuzz");
}

#[test]
fn test_fizzbuzz_number() {
    assert_eq!(fizzbuzz(1), "1");
}