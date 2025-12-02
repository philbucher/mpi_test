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

#[rstest]
#[mpi_test(np = [2, 3])]
fn test_with_values(#[values(10, 20, 30)] count: usize) {
    // Verify the count value
    assert!(count == 10 || count == 20 || count == 30);

    use mpi::traits::*;
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    println!(
        "Process {} of {}: testing with count = {}",
        world.rank(),
        world.size(),
        count
    );
}

#[rstest]
#[mpi_test(np = [2])]
fn test_cartesian_matrix(#[values(1, 2)] x: usize, #[values("a", "b", "c")] y: &str) {
    // Verify the values
    assert!(x == 1 || x == 2);
    assert!(y == "a" || y == "b" || y == "c");

    use mpi::traits::*;
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    println!(
        "Process {} of {}: testing x={}, y={}",
        world.rank(),
        world.size(),
        x,
        y
    );
}
