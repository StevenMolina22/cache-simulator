mod common;
use common::run_and_compare_output;

#[test]
fn test_blowfish_4096_4_16_summary() {
    run_and_compare_output(
        &["./traces/blowfish.xex", "4096", "4", "16"],
        "./output/blowfish_4096-4-16_summary.txt",
        true
    );
}

#[test]
fn test_blowfish_4096_direct_mapped() {
    run_and_compare_output(
        &["./traces/blowfish.xex", "4096", "1", "256", "-v", "0", "10000"],
        "./output/blowfish_4096-1-256.txt",
        true
    );
}

#[test]
fn test_blowfish_1024_2_32() {
    run_and_compare_output(
        &["./traces/blowfish.xex", "1024", "2", "32", "-v", "0", "15000"],
        "./output/blowfish_1024-2-32.txt",
        true
    );
}