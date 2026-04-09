use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn binary_with_no_args_prints_usage() {
    cargo_bin_cmd!("count")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_counts_lines_in_named_files() {
    cargo_bin_cmd!("count")
        .args(["tests/data/two_lines.txt", "tests/data/three_lines.txt"])
        .assert()
        .success()
        .stdout("tests/data/two_lines.txt: 2 lines\ntests/data/three_lines.txt: 3 lines\n");
}
