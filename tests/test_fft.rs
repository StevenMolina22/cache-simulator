mod common;
use common::run_and_compare_output;

#[test]
fn test_fft_1024_4_32_summary() {
    run_and_compare_output(
        &["./traces/FFT.xex", "1024", "4", "32"],
        "./output/fft_1024-4-32_summary.txt",
        true
    );
}

#[test]
fn test_fft_1024_4_32_verbose() {
    run_and_compare_output(
        &["./traces/FFT.xex", "1024", "4", "32", "-v", "0", "15000"],
        "./output/fft_1024-4-32.txt",
        true
    );
}

#[test]
fn test_fft_8192_direct_mapped() {
    run_and_compare_output(
        &["./traces/FFT.xex", "8192", "1", "256", "-v", "0", "10000"],
        "./output/fft_8192-1-256.txt",
        true
    );
}