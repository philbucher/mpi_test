use mpi_test::mpi_test;
use rstest::rstest;

#[mpi_test(np = [2, 4])]
fn simple_test() {
    log_test_execution();
}

#[mpi_test(np = [2, 4])]
#[should_panic = "assertion failed: false"]
fn simple_test_failure() {
    log_test_execution();

    assert!(false);
}

#[rstest]
#[case(1)]
#[case(2)]
#[mpi_test(np = [2, 4])]
fn test_parametric(#[case] value: usize) {
    assert!(value == 1 || value == 2);

    log_test_execution();
}

#[rstest]
#[case(1)]
#[case(2)]
#[should_panic = "assertion failed: false"] // must be located after rstest stuff!
#[mpi_test(np = [2, 4])]
fn test_parametric_fail(#[case] _value: usize) {
    log_test_execution();

    assert!(false);
}

#[rstest]
#[mpi_test(np = [2, 3])]
fn test_with_values(#[values(10, 20, 30)] count: usize) {
    assert!(count == 10 || count == 20 || count == 30);

    log_test_execution();
}

#[rstest]
#[mpi_test(np = [2,4])]
fn test_cartesian_matrix(#[values(1, 2)] x: usize, #[values("a", "b", "c")] y: &str) {
    assert!(x == 1 || x == 2);
    assert!(y == "a" || y == "b" || y == "c");

    log_test_execution();
}

mod my_module {
    use super::*;

    #[mpi_test(np = [2, 4])]
    fn module_test() {
        log_test_execution();
    }
}

/// Logs the execution of the test from all MPI ranks.
/// used to check that all ranks reach the test body
fn log_test_execution() {
    use mpi::traits::*;
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    eprintln!("RANK {} reached test body", world.rank());
}
