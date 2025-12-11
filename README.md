# A Rust testing framework for MPI applications

This crate provides the `#[mpi_test]` procedural macro which simplifies writing and running tests for MPI-based parallel applications.

## Features

- **Easy MPI Testing**: Write tests with the `#[mpi_test]` attribute
- **Multiple Process Counts**: Test with different numbers of MPI processes using `np` parameter
- **Integration with rstest**: Combine with parametric tests using `#[rstest]`
- **Compatible with standard tests**: Coexists with regular tests `#[test]`in the same file

## Usage

Following are some basic examples for using the `#[mpi_test]` attribute. More examples can be found in the documentation.

### Basic Test

```rust
use mpi_test::mpi_test;

#[mpi_test(np = [2, 4])]
fn simple_test() {
    // This test will run with 2 processes, then with 4 processes
    // Your MPI test code here

    // example with using the mpi-crate
    use mpi::traits::*;
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    println!("Hello world from rank {}", world.rank());
}
```

### With Parametric Tests (with rstest)

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

## Requirements

MPI is obviously required, but only for running the tests. This means that this crate can be added as a regular `dev-dependency`, even if MPI is an optional feature/dependency of the including project. No other requirements exist.

It is recommended to use the [mpi-crate](https://github.com/rsmpi/rsmpi) and run the tests with [cargo nextest](https://nexte.st/) for proper reporting.

## Implementation details

As of now it is not possible to configure which launcher is used; `mpiexec` is hardcoded.

Furthermore the reporting of the test-outputs is happening on all ranks. This is fully displayed (and thus rather cluttered) when using `cargo test`. It is therefore recommended to use `cargo nextest run`, which by default only shows the overview.

In order to encapsulate the tests, they are each launched separately with MPI. This prevents earlier tests from altering or crashing tests that are run later. This approach, however, comes with a small overhead of launching an MPI application once per MPI test.

Standard and mpi tests can be mixed in the same file.

Currently the default test harness is used, which means that each test is compiled into a standard test executable, from which it is called with MPI. The base test is ignored by default (since it should only be run with MPI), which unfortunately shows up in the reporting of the test runner.

See [this](https://github.com/rsmpi/rsmpi/issues/143) and [this](https://github.com/nextest-rs/nextest/issues/605) discussion for more details, and the [original idea](https://github.com/nextest-rs/nextest/issues/605#issuecomment-1330189704) for this crate.

## Outlook

If all goes well, this crate could be merged into the [mpi-crate](https://github.com/rsmpi/rsmpi). Until then it is intended to explore how feasible the current approach for running the tests is. It has already been successfully tested in some projects. The overall interface with the `#[mpi_test]` attribute should not change though.