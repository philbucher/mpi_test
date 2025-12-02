use mpi_test_macros::mpi_test;
use rstest::rstest;

#[mpi_test(np = [2, 4])]
fn simple_test() {
    use mpi::traits::*;
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    assert!(world.size() >= 2);
}

#[rstest]
#[case(1)]
#[case(2)]
#[mpi_test(np = [2, 4])]
#[ignore]
fn my_param_test(#[case] value: usize) {
    // Normal Rust test logic
    assert!(value == 1 || value == 2);

    use mpi::traits::*;

    // MPI logic if needed
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    assert!(world.rank() < world.size());

    println!(
        "XXXXX Process {} of {}: value = {}",
        world.rank(),
        world.size(),
        value
    );
}
