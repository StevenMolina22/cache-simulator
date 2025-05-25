mod common;
use common::verify_error_output;

#[test]
fn test_c_is_power_of_2() {
    verify_error_output(&["empty.xex", "2047", "2", "64"]);
}

#[test]
fn test_e_is_power_of_2() {
    verify_error_output(&["empty.xex", "2048", "9", "64"]);
}

#[test]
fn test_s_is_power_of_2() {
    verify_error_output(&["empty.xex", "2048", "2", "60"]);
}

#[test]
fn test_invalid_n_m_range() {
    verify_error_output(&["empty.xex", "2048", "2", "64", "-v", "200", "150"]);
}

#[test]
fn test_invalid_ces_combination() {
    verify_error_output(&["empty.xex", "2048", "32", "256"]);
}