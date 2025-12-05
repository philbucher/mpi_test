# mpi_test

[![Rust](https://github.com/philbucher/mpi_test/actions/workflows/rust.yml/badge.svg)](https://github.com/philbucher/mpi_test/actions/workflows/rust.yml)

A Rust testing framework for MPI (Message Passing Interface) applications. This crate provides the `#[mpi_test]` procedural macro which simplifies writing and running tests for MPI-based parallel applications.

## Features

- **Easy MPI Testing**: Write tests with the `#[mpi_test]` attribute
- **Multiple Process Counts**: Test with different numbers of MPI processes using `np` parameter
- **Integration with rstest**: Combine with parametric tests using `#[rstest]`

## Usage

### Basic Test

```rust
use mpi_test::mpi_test;

#[mpi_test(np = [2, 4])]
fn simple_test() {
    // This test will run with 2 processes, then with 4 processes
    // Your MPI test code here
}
```

### With Parametric Tests

```rust
use mpi_test::mpi_test;
use rstest::rstest;

#[rstest]
#[case(10)]
#[case(20)]
#[mpi_test(np = [2, 4])]
fn parametric_test(#[case] value: usize) {
    // This generates 4 tests total:
    // - case 10 with 2 processes
    // - case 10 with 4 processes
    // - case 20 with 2 processes
    // - case 20 with 4 processes
}
```
