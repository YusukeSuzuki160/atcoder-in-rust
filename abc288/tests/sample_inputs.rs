use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 2
3 1 4 1 5
9 2 6 5 3
3 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "17
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20 8
29 27 79 27 30 4 93 89 44 88 70 75 96 3 78 39 97 12 53 62
32 38 84 49 93 53 26 13 25 2 76 32 42 34 18 77 14 67 88 12
1 3 4 5 8 14 16 20
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "533
");
    assert!(output.stderr_str().is_empty());
}
/*
#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3
1 2
1 3
2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1
");
    assert!(output.stderr_str().is_empty());
}
*/
