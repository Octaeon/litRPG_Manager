// I don't know why, but for some reason without this allow, the rust-analyzer
// thinks that the imports that ARE NECESSARY are unused. No idea why.
#![allow(unused_imports)]

use crate::types::error::ParsingErr;
use crate::types::{BiOperation, Command, Content, Expression};

use super::{parseCommand, parseExpression, parseFile, parseToken, tokenizeExpression, Token};

#[test]
fn expression() {
    assert_eq!(
        parseExpression("1 + 2".to_string()),
        Ok(Expression::Binary(
            BiOperation::Add,
            Box::new(Expression::Value(1)),
            Box::new(Expression::Value(2))
        ))
    );

    assert_eq!(
        parseExpression("1 + 2 * 3".to_string()),
        Ok(Expression::Binary(
            BiOperation::Add,
            Box::new(Expression::Value(1)),
            Box::new(Expression::Binary(
                BiOperation::Multiply,
                Box::new(Expression::Value(2)),
                Box::new(Expression::Value(3))
            ))
        ))
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        tokenizeExpression("1+2".to_string()),
        Ok(vec![
            Token::Number(1),
            Token::Operator('+'),
            Token::Number(2)
        ])
    );
    assert_eq!(
        tokenizeExpression("1  +    2".to_string()),
        Ok(vec![
            Token::Number(1),
            Token::Operator('+'),
            Token::Number(2)
        ])
    );

    assert_eq!(
        tokenizeExpression("1*2 + (3 1 - test))".to_string()),
        Ok(vec![
            Token::Number(1),
            Token::Operator('*'),
            Token::Number(2),
            Token::Operator('+'),
            Token::Operator('('),
            Token::Number(3),
            Token::Number(1),
            Token::Operator('-'),
            Token::Variable("test".to_string()),
            Token::Operator(')'),
            Token::Operator(')'),
        ])
    );
}

#[test]
fn command() {
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
        Ok(vec![Command::Subtract("zero".to_string(), -1)])
    );

    assert_eq!(
        parseCommand("subtract zero -1; let a 10".to_string()),
        Ok(vec![
            Command::Subtract("zero".to_string(), -1),
            Command::Let("a".to_string(), 10)
        ])
    );
}

#[test]
fn wholeText() {
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
