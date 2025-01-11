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
