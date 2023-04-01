use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100000 10000000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10000000000
");
    assert!(output.stderr_str().is_empty());
}

