use super::*;

#[test]
fn commandParsing() {
    assert_eq!(
        parseCommand("let a".to_string()),
        Err(ParsingErr::InvalidNumberOfArguments)
    );

    assert_eq!(
        parseCommand("another test whatever".to_string()),
        Err(ParsingErr::UnrecognizedCommand("another".to_string()))
    );

    assert_eq!(
        parseCommand("let it be...".to_string()),
        Err(ParsingErr::NumberParsing(
            "be...".parse::<i32>().expect_err("")
        ))
    );

    assert_eq!(
        parseCommand("subtract zero -1".to_string()),
        Ok(Command::Subtract("zero".to_string(), -1))
    );
}

#[test]
fn wholeTextParsing() {
    let wrongInput: String = String::from("$unclosed command");
    assert_eq!(parseFile(wrongInput), Err(ParsingErr::CommandLeftOpen));

    let goodInput: String = String::from(
        "This is text\n$let variable -1$More text\n$add variable 2$Variable is $write variable$",
    );
    assert_eq!(
        parseFile(goodInput),
        Ok(vec![
            Content::Text("This is text\n".to_string()),
            Content::Command(Command::Let("variable".to_string(), -1)),
            Content::Text("More text\n".to_string()),
            Content::Command(Command::Add("variable".to_string(), 2)),
            Content::Text("Variable is ".to_string()),
            Content::Command(Command::Write("variable".to_string()))
        ])
    );
}

#[test]
fn interpreterFunctionality() {
    let mut storage = Storage::new();

    // Test if trying to modify a nonexisting variable will result in the correct error
    assert_eq!(
        interpreter::run(
            &mut storage,
            vec![Content::Command(Command::Add(String::new(), 1))]
        ),
        Err(RunErr::TriedToModifyNonexistentVariable)
    );

    // Test if trying to write a nonexisting variable will result in the corresponding error
    assert_eq!(
        interpreter::run(
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

    assert_eq!(
        interpreter::run(&mut storage, commands),
        Ok("12105".to_string())
    );

    // Test if initializing over existing variable will result in an error
    storage.clear();
    storage.createVariable("test".to_string(), 1).expect("WHAT");

    assert_eq!(
        interpreter::run(
            &mut storage,
            vec![Content::Command(Command::Let(String::from("test"), 1))]
        ),
        Err(RunErr::TriedToInitializeExistingVariable)
    );
}
