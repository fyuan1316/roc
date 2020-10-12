// #[macro_use]
extern crate pretty_assertions;

extern crate bumpalo;
extern crate inlinable_string;
extern crate roc_collections;
extern crate roc_load;
extern crate roc_module;

mod helpers;

#[cfg(test)]
mod cli_run {
    use crate::helpers::{example_file, extract_valgrind_errors, run_roc, run_with_valgrind, Out};

    #[test]
    fn run_hello_world() {
        fn check_hello_world_output(out: Out) {
            if !out.stderr.is_empty() {
                panic!(out.stderr);
            }
            assert!(out.status.success());

            let valgrind_out =
                run_with_valgrind(&[example_file("hello-world", "app").to_str().unwrap()]);
            assert!(&valgrind_out.stdout.ends_with("Hello, World!!!!!!!!!!!!!\n"));
            assert!(valgrind_out.status.success());
            let memory_errors = extract_valgrind_errors(&valgrind_out.stderr);
            if !memory_errors.is_empty() {
                panic!("{:?}", memory_errors);
            }
        }
        check_hello_world_output(run_roc(&[
            "build",
            example_file("hello-world", "Hello.roc").to_str().unwrap(),
        ]));
        check_hello_world_output(run_roc(&[
            "build",
            "--optimize",
            example_file("hello-world", "Hello.roc").to_str().unwrap(),
        ]));
    }

    #[test]
    fn run_quicksort() {
        fn check_quicksort_output(out: Out) {
            if !out.stderr.is_empty() {
                panic!(out.stderr);
            }
            assert!(out.status.success());

            let valgrind_out =
                run_with_valgrind(&[example_file("quicksort", "app").to_str().unwrap()]);
            assert!(&valgrind_out
                .stdout
                .ends_with("[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]\n"));
            assert!(valgrind_out.status.success());
            let memory_errors = extract_valgrind_errors(&valgrind_out.stderr);
            if !memory_errors.is_empty() {
                panic!("{:?}", memory_errors);
            }
        }

        check_quicksort_output(run_roc(&[
            "build",
            example_file("quicksort", "Quicksort.roc").to_str().unwrap(),
        ]));
        check_quicksort_output(run_roc(&[
            "build",
            "--optimize",
            example_file("quicksort", "Quicksort.roc").to_str().unwrap(),
        ]));
    }
}
