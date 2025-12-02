// src/lib.rs of crate: mpi_test_runner

use std::io;
use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub enum MpiTestError {
    IoError(io::Error),
    MissingBinary,
    FailedStatus(ExitStatus),
}

impl From<io::Error> for MpiTestError {
    fn from(e: io::Error) -> Self {
        MpiTestError::IoError(e)
    }
}

/// Runs a named test inside an MPI environment.
///
/// # Example MPI invocation:
///
/// mpiexec -n <np> <binary> <test_name> --nocapture --exact
///
pub fn run_mpi(test_name: &str, np: u32) -> Result<(), MpiTestError> {
    // Determine the test binary (the currently-running binary).
    let test_binary = std::env::args().next().ok_or(MpiTestError::MissingBinary)?;

    let mut cmd = Command::new("mpiexec");

    cmd.args(&[
        "-n",
        &np.to_string(),
        &test_binary,
        test_name,
        "--nocapture",
        "--exact",
    ]);

    let status = cmd.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(MpiTestError::FailedStatus(status))
    }
}
