use crate::types::engine::Storage;

use crate::interpreter;
use crate::parser::parseFile;

#[test]
fn integrated() {
    let whole_tests: Vec<(String, String)> = vec![
        ("test.txt", "expected_output.txt"),
        ("test.md", "expected_output.md"),
    ]
    .iter()
    .map(|(a, b)| {
        let path_to_tests = String::from("tests/");
        (path_to_tests.clone() + a, path_to_tests + b)
    })
    .collect();

    for (input, expect) in whole_tests {
        let text_input = std::fs::read_to_string(input).expect("Couldn't read the input file");
        let expected_output =
            std::fs::read_to_string(expect).expect("Couldn't read the expected output file");

        let mut storage: Storage = Storage::new();

        let output = interpreter::run(&mut storage, parseFile(text_input).expect("Parsing error"))
            .expect("Runtime error");

        assert_eq!(expected_output, output);
    }
}
