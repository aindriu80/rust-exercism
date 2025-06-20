use macros::hashmap;
use std::collections::HashMap;

#[test]
fn empty() {
    let expected: HashMap<u32, u32> = HashMap::new();
    let computed: HashMap<u32, u32> = hashmap!();
    assert_eq!(computed, expected);
}

#[test]
fn single() {
    let mut expected = HashMap::new();
    expected.insert(1, "one");
    assert_eq!(hashmap!(1 => "one"), expected);
}

#[test]
fn no_trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert(1, "one");
    expected.insert(2, "two");
    assert_eq!(hashmap!(1 => "one", 2 => "two"), expected);
}

#[test]
fn trailing_comma() {
    let mut expected = HashMap::new();
    expected.insert('h', 89);
    expected.insert('a', 1);
    expected.insert('s', 19);
    expected.insert('h', 8);
    assert_eq!(
        hashmap!(
            'h' => 89,
            'a' => 1,
            's' => 19,
            'h' => 8,
        ),
        expected
    );
}

#[test]
fn nested() {
    let mut expected = HashMap::new();
    expected.insert("non-empty", {
        let mut subhashmap = HashMap::new();
        subhashmap.insert(23, 623);
        subhashmap.insert(34, 21);
        subhashmap
    });
    expected.insert("empty", HashMap::new());
    assert_eq!(
        hashmap!(
            "non-empty" => hashmap!(
                23 => 623,
                34 => 21
            ),
            "empty" => hashmap!()
        ),
        expected
    );
}

mod test {
    #[test]
    fn type_not_in_scope() {
        use macros::hashmap;

        let _empty: ::std::collections::HashMap<(), ()> = hashmap!();
        let _without_comma = hashmap!(23=> 623, 34 => 21);
        let _with_trailing = hashmap!(23 => 623, 34 => 21,);
    }

    #[test]
    fn macro_out_of_scope() {
        let _empty: ::std::collections::HashMap<(), ()> = macros::hashmap!();
        let _without_comma = macros::hashmap!(23=> 623, 34 => 21);
        let _with_trailing = macros::hashmap!(23 => 623, 34 => 21,);
    }
}

#[test]
fn type_override() {
    // The macro should always use std::collections::HashMap and ignore crate::std::collections::HashMap
    mod std {
        pub mod collections {
            pub struct HashMap;

            impl HashMap {
                #[allow(dead_code)]
                pub fn new() -> Self {
                    panic!("Do not allow users to override which HashMap is used");
                }

                #[allow(dead_code)]
                pub fn insert<K, V>(&mut self, _key: K, _val: V) {
                    panic!("Do not allow users to override which HashMap is used");
                }
            }
        }
    }

    let _empty: ::std::collections::HashMap<(), ()> = hashmap!();
    let _without_comma = hashmap!(1 => 2, 3 => 4);
    let _with_trailing = hashmap!(1 => 2, 3 => 4,);
}

#[test]
fn compile_fails_comma_sep() {
    simple_trybuild::compile_fail("comma_sep.rs");
}

#[test]
fn compile_fails_double_commas() {
    simple_trybuild::compile_fail("double_commas.rs");
}

#[test]
fn compile_fails_only_comma() {
    simple_trybuild::compile_fail("only_comma.rs");
}

#[test]
fn compile_fails_single_argument() {
    simple_trybuild::compile_fail("single_argument.rs");
}

#[test]
fn compile_fails_triple_arguments() {
    simple_trybuild::compile_fail("triple_arguments.rs");
}

#[test]
fn compile_fails_only_arrow() {
    simple_trybuild::compile_fail("only_arrow.rs");
}

#[test]
fn compile_fails_two_arrows() {
    simple_trybuild::compile_fail("two_arrows.rs");
}

#[test]
fn compile_fails_leading_comma() {
    simple_trybuild::compile_fail("leading_comma.rs");
}

#[test]
fn compile_fails_no_comma() {
    simple_trybuild::compile_fail("no_comma.rs");
}

#[test]
fn compile_fails_missing_argument() {
    simple_trybuild::compile_fail("missing_argument.rs");
}

mod simple_trybuild {
    use std::path::PathBuf;
    use std::process::Command;

    pub fn compile_fail(file_name: &str) {
        let invalid_path: PathBuf = ["tests", "invalid"].iter().collect::<PathBuf>();

        let mut file_path = invalid_path.clone();
        file_path.push(file_name);
        assert!(
            file_path.exists(),
            "{:?} does not exist.",
            file_path.into_os_string()
        );

        let test_name = file_name.strip_suffix(".rs").unwrap();
        let macros_dir = ["..", "..", "target", "tests", "macros"]
            .iter()
            .collect::<PathBuf>();

        let result = Command::new("cargo")
            .current_dir(invalid_path)
            .arg("build")
            .arg("--offline")
            .arg("--target-dir")
            .arg(macros_dir)
            .arg("--bin")
            .arg(test_name)
            .output();

        if let Ok(result) = result {
            assert!(
                !result.status.success(),
                "Expected {file_path:?} to fail to compile, but it succeeded."
            );
        } else {
            panic!("Running subprocess failed.");
        }
    }
}
