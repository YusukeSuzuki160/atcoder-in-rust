use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 1 1
1 2
4 1
1 4
1 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "50.000000000000000
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 2 2
6 4
10 1
5 8
9 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "62.500000000000000
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 5 10
5 4
1 6
7 4
9 8
2 2
5 6
6 7
5 3
8 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "54.166666666666664
");
    assert!(output.stderr_str().is_empty());
}
