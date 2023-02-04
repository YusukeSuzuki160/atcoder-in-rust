use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7 3
3 -1 1 -2 2 0 5
2
1 6
2 7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Yes
No
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20 4
-19 -66 -99 16 18 33 32 28 26 11 12 0 -16 4 21 21 37 17 55 -19
5
13 16
4 11
3 12
13 18
4 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No
Yes
No
Yes
No
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
