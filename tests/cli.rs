//! Command-line scaffold integration tests.

use std::process::Command;

#[test]
fn cli_starts_successfully() {
    let result = Command::new(env!("CARGO_BIN_EXE_daoji"))
        .status()
        .map(|status| status.success());

    assert!(matches!(result, Ok(true)), "CLI failed: {result:?}");
}
