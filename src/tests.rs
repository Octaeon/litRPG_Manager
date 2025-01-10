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
    let _input = "";
}
