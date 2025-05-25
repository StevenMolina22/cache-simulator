mod common;
use common::run_and_compare_output;

#[test]
fn test_adpcm_2048_2_64_summary() {
    run_and_compare_output(
        &["./traces/adpcm.xex", "2048", "2", "64"],
        "./output/adpcm_2048-2-64_summary.txt",
        true
    );
}

#[test]
fn test_adpcm_2048_2_64_verbose() {
    run_and_compare_output(
        &["./traces/adpcm.xex", "2048", "2", "64", "-v", "0", "15000"],
        "./output/adpcm_2048-2-64.txt",
        true
    );
}

#[test]
fn test_adpcm_4096_direct_mapped() {
    run_and_compare_output(
        &["./traces/adpcm.xex", "4096", "1", "256", "-v", "0", "10000"],
        "./output/adpcm_4096-1-256.txt",
        true
    );
}