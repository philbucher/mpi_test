//! # MPI Test Framework
//!
//! A testing framework for MPI-based code in Rust that makes it easy to write
//! tests that run with different numbers of MPI processes.
//!
//! ## Features
//!
//! - Write MPI tests with a simple attribute macro
//! - Test with multiple process counts in a single test function
//! - Compatible with `rstest` for parametric testing
//! - Works with standard test attributes like `#[should_panic]`
//!
//! ## Basic Usage
//!
//! ```ignore
//! use mpi_test::mpi_test;
//!
//! #[mpi_test(np = [2, 4])]
//! fn test_mpi_communication() {
//!     use mpi::traits::*;
//!     let universe = mpi::initialize().unwrap();
//!     let world = universe.world();
//!
//!     assert!(world.size() >= 2);
//!     println!("Hello from rank {} of {}", world.rank(), world.size());
//! }
//! ```
//!
//! This generates two tests: `test_mpi_communication::mpi_np_2` and
//! `test_mpi_communication::mpi_np_4`, which run with 2 and 4 processes respectively.
//!
//! ## Using with rstest
//!
//! Combine with `rstest` for parametric testing:
//!
//! ```ignore
//! use mpi_test::mpi_test;
//! use rstest::rstest;
//!
//! #[rstest]
//! #[case(100)]
//! #[case(1000)]
//! #[mpi_test(np = [2, 4])]
//! fn test_different_sizes(#[case] size: usize) {
//!     use mpi::traits::*;
//!     let universe = mpi::initialize().unwrap();
//!     let world = universe.world();
//!
//!     // Test runs for each combination of size and process count
//!     let local_size = size / world.size() as usize;
//!     assert!(local_size > 0);
//! }
//! ```
//!
//! ## Expected Failures
//!
//! Use `#[should_panic]` for tests that are expected to fail:
//!
//! ```ignore
//! use mpi_test::mpi_test;
//!
//! #[mpi_test(np = [2, 4])]
//! #[should_panic(expected = "assertion failed")]
//! fn test_insufficient_processes() {
//!     use mpi::traits::*;
//!     let universe = mpi::initialize().unwrap();
//!     let world = universe.world();
//!
//!     // This will panic since we only run with 2 or 4 processes
//!     assert!(world.size() >= 100);
//! }
//! ```

// Re-export the procedural macro
pub use mpi_test_macros::mpi_test;
