#![allow(unused_imports)]

use crate::types::engine::Storage;
use crate::types::error::RunErr;
use crate::types::{Command, Content};

#[test]
fn functionality() {
    let mut storage = Storage::new();

    // Test if trying to modify a nonexisting variable will result in the correct error
    assert_eq!(
        super::run(
            &mut storage,
            vec![Content::Command(Command::Add(String::new(), 1))]
        ),
        Err(RunErr::TriedToModifyNonexistentVariable)
    );

    // Test if trying to write a nonexisting variable will result in the corresponding error
    assert_eq!(
        super::run(
            &mut storage,
            vec![Content::Command(Command::Write(String::new()))]
        ),
        Err(RunErr::TriedToGetNonexistentVariable)
    );

    // Test if a viable series of commands (using all of the currently available ones)
    // will result in the predicted output
    let commands: Vec<Content> = (vec![
        Command::Let("test".to_string(), 1),
        Command::Write("test".to_string()),
        Command::Add("test".to_string(), 1),
        Command::Write("test".to_string()),
        Command::Set("test".to_string(), 10),
        Command::Write("test".to_string()),
        Command::Subtract("test".to_string(), 5),
        Command::Write("test".to_string()),
    ])
    .into_iter()
    .map(|c| Content::Command(c))
    .collect();

    assert_eq!(super::run(&mut storage, commands), Ok("12105".to_string()));

    // Test if initializing over existing variable will result in an error
    storage.clear();
    storage.createVariable("test".to_string(), 1).expect("WHAT");

    assert_eq!(
        super::run(
            &mut storage,
            vec![Content::Command(Command::Let(String::from("test"), 1))]
        ),
        Err(RunErr::TriedToInitializeExistingVariable)
    );
}
