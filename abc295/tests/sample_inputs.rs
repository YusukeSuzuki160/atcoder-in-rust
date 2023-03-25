use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20230322
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
        .output_with_stdin(r#"0112223333444445555556666666777777778888888889999999999
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "185
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3141592653589793238462643383279502884197169399375105820974944
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9
");
    assert!(output.stderr_str().is_empty());
}

/*
#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 6
#.#3#.
###.#.
##.###
#1..#.
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "......
#.....
#....#
....#.
");
    assert!(output.stderr_str().is_empty());
}
*/
