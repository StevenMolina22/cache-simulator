//! Integration tests for the cachesim project.
//!
//! This file serves as an entry point for running all integration tests.
//! Tests are organized by categories matching the original Python test structure.

mod common;

// Import all the test modules
#[path = "test_args.rs"]
mod args_tests;

#[path = "test_adpcm.rs"]
mod adpcm_tests;

#[path = "test_fft.rs"]
mod fft_tests;

#[path = "test_blowfish.rs"]
mod blowfish_tests;

/// Run this test to verify the build is successful before running other tests
#[test]
fn test_binary_exists() {
    use std::path::Path;
    
    let binary_path = Path::new("./target/debug/cachesim");
    assert!(binary_path.exists(), "cachesim binary not found. Run 'cargo build' first.");
}

/// Main setup that ensures the environment is ready for testing
#[cfg(test)]
mod tests {
    use std::path::Path;
    
    /// Check that required test directories exist
    #[test]
    fn test_required_directories_exist() {
        assert!(Path::new("./traces").exists(), "Traces directory not found");
        assert!(Path::new("./output").exists(), "Output directory not found");
    }
    
    /// Verify that trace files exist
    #[test]
    fn test_trace_files_exist() {
        assert!(Path::new("./traces/FFT.xex").exists(), "FFT.xex trace file not found");
        assert!(Path::new("./traces/adpcm.xex").exists(), "adpcm.xex trace file not found");
        assert!(Path::new("./traces/blowfish.xex").exists(), "blowfish.xex trace file not found");
    }
}