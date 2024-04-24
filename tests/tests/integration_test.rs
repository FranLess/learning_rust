use tests;
mod common;

// integrations tests are files in the tests directory which contain multiple tests
// so when we run cargo tests it will run test in tests path
// the way to run this sort of test is: cargo test --test *
// notice that we can replace the "*" for any specific test file name (cargo test --test integration_test)

// if we dont write * or not pass any specific test to run we'll get the next output
// cargo test --test
// error: "--test" takes one argument.
// Available test targets:
//     integration_test
//     integration_test2

#[test]
fn it_adds_two_inside_integrations_test() {
    common::function_for_tests_in_module(); // we should run cargo cargo test -- --show-output
                                            // to see the println statement inside the common
                                            // module
    assert_eq!(4, tests::add_two(2));
}
