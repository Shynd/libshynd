#![crate_name = "libshynd"]
#![crate_type = "lib"]

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