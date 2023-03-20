use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3
2
1 2
2
1 3
1
2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 2
2
1 2
2
1 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 6
3
2 3 6
3
2 4 6
2
3 6
3
1 5 6
3
1 3 6
2
1 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "18
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 5
        1
        1
        1
        1
        1
        1
        1
        1
        1
        1"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "31\n");
    assert!(output.stderr_str().is_empty());
}

