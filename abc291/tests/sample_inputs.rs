use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1 2
4 2
3 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1 5
2 6
3 7
4 8
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
        .output_with_stdin(r#"8
877914575 602436426
861648772 623690081
476190629 262703497
971407775 628894325
822804784 450968417
161735902 822804784
161735902 822804784
822804784 161735902
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "48
");
    assert!(output.stderr_str().is_empty());
}

