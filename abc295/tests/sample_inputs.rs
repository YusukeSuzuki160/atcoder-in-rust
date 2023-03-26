use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 5 2
2 0 4
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
        .output_with_stdin(r#"2 3 1
0 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "221832080
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 20 7
6 5 0 2 0 0 0 15 0 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "617586310
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
