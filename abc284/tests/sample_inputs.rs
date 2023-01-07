use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 2
1 2
2 3
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
        .output_with_stdin(r#"4 6
1 2
1 3
1 4
2 3
2 4
3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "16
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8 21
2 6
1 3
5 6
3 8
3 6
4 7
4 6
3 4
1 5
2 4
1 2
2 7
1 4
3 5
2 5
2 3
4 5
3 7
6 7
5 7
2 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2023
");
    assert!(output.stderr_str().is_empty());
}

