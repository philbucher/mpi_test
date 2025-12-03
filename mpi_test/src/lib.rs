//! # MPI Test Framework
//!
//! A testing framework for MPI-based code in Rust.
//!
//! ## Usage
//!
//! ```rust
//! use mpi_test::mpi_test;
//!
//! #[mpi_test(np = [2, 4])]
//! fn my_parallel_test() {
//!     // Your MPI test code here
//! }
//! ```

// Re-export the procedural macro
pub use mpi_test_macros::mpi_test;
