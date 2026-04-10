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

#[test]
fn binary_with_w_flag_counts_words() {
    cargo_bin_cmd!("count")
        .args(["-w", "tests/data/three_lines.txt"])
        .assert()
        .success()
        .stdout("tests/data/three_lines.txt: 6 words\n");
}

#[test]
fn binary_with_b_flag_counts_bytes() {
    cargo_bin_cmd!("count")
        .args(["-b", "tests/data/three_lines.txt"])
        .assert()
        .success()
        .stdout("tests/data/three_lines.txt: 20 bytes\n");
}

#[test]
fn binary_with_w_and_b_flag_counts_words_and_bytes() {
    cargo_bin_cmd!("count")
        .args(["-w", "-b", "tests/data/three_lines.txt"])
        .assert()
        .success()
        .stdout("tests/data/three_lines.txt: 6 words\ntests/data/three_lines.txt: 20 bytes\n");
}