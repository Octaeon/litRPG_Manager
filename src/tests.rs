use super::*;

#[test]
fn commandParsing() {
    assert_eq!(
        parseCommand("test".to_string()),
        Err(Error::InvalidNumberOfArguments)
    );

    assert_eq!(
        parseCommand("another test whatever".to_string()),
        Err(Error::UnrecognizedCommand("another".to_string()))
    );

    assert_eq!(
        parseCommand("let it be...".to_string()),
        Err(Error::Parse("be...".parse::<i32>().expect_err("")))
    );

    assert_eq!(
        parseCommand("subtract zero -1".to_string()),
        Ok(Command::Subtract("zero".to_string(), -1))
    );
}

#[test]
fn wholeTextParsing() {
    let _input = "";
}
