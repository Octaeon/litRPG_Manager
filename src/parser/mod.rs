use std::cmp::Ordering;

use crate::types::error::ParsingErr;
use crate::types::{BiOperation, Command, Content, Expression, UnOperation};

mod tests;

/// At the moment, this function takes in a String (which is allocated on the heap)
/// but doesn't do operation on that specific variable, but instead assigns individuals chars of the input
/// to the buffer depending on their value.
///
/// A better way to do this would be to instead keep track of the locations of the breakpoints, and when
/// we find the next, take that chunk of text and use it to do whatever.
///
/// If the end result were to be a string slice (`&str`), there would be the question of lifetimes, but it would
/// be easier to simply create a new string with that content, essentially cloning it.
///
/// Importantly for optimization, the input string shouldn't be touched at all and simply copied.
pub fn parseFile(inputString: String) -> Result<Vec<Content>, ParsingErr> {
    let mut result: Vec<Content> = vec![];

    let getter = |a: usize, b: usize| -> Result<String, ParsingErr> {
        inputString
            .get(a..b)
            .map(|slice| slice.to_owned())
            .ok_or(ParsingErr::StringOverflow)
    };

    let mut reading_command: bool = false;

    let mut chunk_start = 0;
    let mut current = 0;

    for char in inputString.clone().chars() {
        current += 1;

        match char {
            '$' => {
                // If we encounter a $, it means that we're either at the beginning of the command, or at the end
                // If `reading_command` flag is true, it means we were at the end of one, so try and parse it and set the flag to false.
                // If it's false, then we're at the beginning of one, so set the flag to true.
                let t = getter(chunk_start, current - 1)?;

                let mut chunk = if reading_command {
                    parseCommand(t).map(|commands| {
                        commands
                            .iter()
                            .map(|o| Content::Command(o.clone()))
                            .collect()
                    })
                } else {
                    Ok(vec![Content::Text(t)])
                }?;

                reading_command = !reading_command;
                result.append(&mut chunk);
                chunk_start = current;
            }
            _ => {}
        }
    }

    if reading_command {
        // If all the characters have been read and we're still in the 'reading commands' state, it means that someone opened
        // a command statement but didn't close it, so we throw an error.
        Err(ParsingErr::CommandLeftOpen)
    } else {
        if chunk_start < current {
            result.push(Content::Text(getter(chunk_start, current)?));
        }
        Ok(result)
    }
}

fn matchWhitespace(character: char) -> bool {
    match character {
        ' ' => true,
        '\n' => true,
        _ => false,
    }
}

fn matchCommandEnd(character: char) -> bool {
    match character {
        '\n' => true,
        ';' => true,
        _ => false,
    }
}

/// The program is meant to work on numbers, which are all stored as integers. No floating point numbers.
///
/// List of commands:
/// - let : creates a variable and initializes it with the given value. Example: ```let variable 0```
/// - add : adds a value to a variable. Example: ```add variable 10```
/// - subtract : subtracts a value from a variable. Example: ```sub variable 10```
/// - set : sets a variable to a new value. Example: ```set variable -10```
fn parseCommand(input: String) -> Result<Vec<Command>, ParsingErr> {
    let statements: Vec<&str> = input
        .split(matchCommandEnd)
        .filter(|c| !c.is_empty())
        .collect();

    let mut result_commands: Vec<Command> = Vec::new();

    for com in statements {
        let words: Vec<&str> = com
            .split(matchWhitespace)
            .filter(|c| !c.is_empty())
            .collect();

        let amount_of_words = words.len();

        // This is a tiny function I made to check if the number of words is equal to the expected.
        // This could of course be done in each match case, but this way it's much less code repetition.
        let checkNumOfArguments = |expectedNumOfWords: usize| {
            if expectedNumOfWords == amount_of_words - 1 {
                Ok(())
            } else {
                Err(ParsingErr::InvalidNumberOfArguments)
            }
        };

        // First, before even trying to match the first command, we must ensure there is at least one. We could always get
        // an empty string as input, after all.
        if amount_of_words == 0 {
            return Err(ParsingErr::InvalidNumberOfArguments);
        }

        let command = match words[0] {
            "let" => {
                checkNumOfArguments(2)?;
                Ok(Command::Let(words[1].to_string(), words[2].parse::<i32>()?))
            }

            "add" => {
                checkNumOfArguments(2)?;
                Ok(Command::Add(words[1].to_string(), words[2].parse::<i32>()?))
            }
            "subtract" => {
                checkNumOfArguments(2)?;
                Ok(Command::Subtract(
                    words[1].to_string(),
                    words[2].parse::<i32>()?,
                ))
            }
            "set" => {
                checkNumOfArguments(2)?;
                Ok(Command::Set(words[1].to_string(), words[2].parse::<i32>()?))
            }
            "write" => {
                // As the write command can take in expressions now, there is no check for the
                // number of arguments.
                let _a = parseExpression(words[1..].concat())?;
                Ok(Command::Write(words[1].to_string()))
            }
            other_command => Err(ParsingErr::UnrecognizedCommand(other_command.to_string())),
        }?;

        result_commands.push(command);
    }

    Ok(result_commands)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Binary(BiOperation),
    Unary(UnOperation),
    LeftBracket,
    RightBracket,
}

impl PartialOrd for Op {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            match self {
                Op::Binary(BiOperation::Add) | Op::Binary(BiOperation::Subtract) => {
                    if other == &Op::Binary(BiOperation::Add)
                        || other == &Op::Binary(BiOperation::Subtract)
                    {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Less)
                    }
                }
                Op::Binary(BiOperation::Multiply) => {
                    if other == &Op::Binary(BiOperation::Add)
                        || other == &Op::Binary(BiOperation::Subtract)
                    {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Less)
                    }
                }
                Op::Binary(BiOperation::Exponentiate) => {
                    if other == &Op::LeftBracket {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                Op::Unary(UnOperation::Minus) => {
                    if other == &Op::LeftBracket || other == &Op::Binary(BiOperation::Exponentiate)
                    {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                }
                Op::LeftBracket => Some(Ordering::Greater),
                Op::RightBracket => Some(Ordering::Less),
            }
        }
    }
}

fn toOperation(lastOperation: bool, i: char) -> Result<Op, ParsingErr> {
    match i {
        '(' => Ok(Op::LeftBracket),
        ')' => Ok(Op::RightBracket),
        '-' => Ok(if lastOperation {
            Op::Unary(UnOperation::Minus)
        } else {
            Op::Binary(BiOperation::Subtract)
        }),
        '+' => Ok(Op::Binary(BiOperation::Add)),
        '*' => Ok(Op::Binary(BiOperation::Multiply)),
        '^' => Ok(Op::Binary(BiOperation::Exponentiate)),
        _ => Err(ParsingErr::ExpressionParsing),
    }
}

fn collapseOperation(
    expressions_stack: &mut Vec<Expression>,
    op_stack: &mut Vec<Op>,
) -> Result<Expression, ParsingErr> {
    let popped = |stack: &mut Vec<Expression>| -> Result<Box<Expression>, ParsingErr> {
        Ok(Box::new(stack.pop().ok_or(ParsingErr::ExpressionParsing)?))
    };

    let result = match op_stack.pop() {
        Some(Op::Binary(bi_operation)) => {
            let rhand = popped(expressions_stack)?;
            let lhand = popped(expressions_stack)?;
            Ok(Expression::Binary(bi_operation, lhand, rhand))
        }
        Some(Op::Unary(un_operation)) => {
            Ok(Expression::Unary(un_operation, popped(expressions_stack)?))
        }
        Some(Op::LeftBracket) => todo!(),
        Some(Op::RightBracket) => collapseOperation(expressions_stack, op_stack),
        None => Err(ParsingErr::ExpressionParsing),
    }?;

    Ok(result)
}

fn parseExpression(input: String) -> Result<Expression, ParsingErr> {
    let tokens: Vec<Token> = tokenizeExpression(input)?;

    let mut operations_stack: Vec<Op> = Vec::new();
    let mut operands_stack: Vec<Expression> = Vec::new();

    let mut last_token_was_operator = false;

    let shouldCollapse = |stack: &Vec<Op>, op: &Op| -> bool {
        match stack.last() {
            Some(stack_operation) => stack_operation < op,
            None => false,
        }
    };

    for t in tokens.iter() {
        match t {
            Token::Number(num) => {
                operands_stack.push(Expression::Value(*num));
                last_token_was_operator = false;
            }
            Token::Variable(var) => {
                operands_stack.push(Expression::Variable(var.to_owned()));
                last_token_was_operator = false;
            }
            Token::Operator(o) => {
                let operation = toOperation(last_token_was_operator, *o)?;

                if shouldCollapse(&operations_stack, &operation) {
                    // if the operation on the stack is of higher priority, pop the operands from the stack
                    // combine them into a new expression and put it back on the stack.
                    let new_op = collapseOperation(&mut operands_stack, &mut operations_stack)?;

                    operands_stack.push(new_op);
                } else {
                    // if the operation on the stack is of lower priority, continue onwards.
                    operations_stack.push(operation);
                }

                last_token_was_operator = true;
            }
        }
    }

    while operands_stack.len() > 1 {
        let new_op = collapseOperation(&mut operands_stack, &mut operations_stack)?;

        operands_stack.push(new_op);
    }

    operands_stack.pop().ok_or(ParsingErr::ExpressionParsing)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(i32),
    Operator(char),
    Variable(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Whitespace,
    ParsingNumber,
    ParsingVariable,
}

fn tokenizeExpression(input: String) -> Result<Vec<Token>, ParsingErr> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut buffer: String = String::new();
    let mut state: State = State::Whitespace;

    let createToken = |t: &mut Vec<Token>, buff: &mut String| -> Result<(), ParsingErr> {
        if !buff.is_empty() {
            Ok(t.push(parseToken(buff.drain(..).collect())?))
        } else {
            Ok(())
        }
    };

    for char in input.chars() {
        match (state, char) {
            (_, '+' | '-' | '*' | '^' | '(' | ')') => {
                createToken(&mut tokens, &mut buffer)?;

                tokens.push(Token::Operator(char));
            }

            (State::ParsingVariable, 'a'..='z' | 'A'..='Z') => {
                buffer.push(char);
            }
            (_, 'a'..='z' | 'A'..='Z') => {
                createToken(&mut tokens, &mut buffer)?;

                state = State::ParsingVariable;
                buffer.push(char);
            }

            (State::ParsingNumber, '0'..='9') => {
                buffer.push(char);
            }
            (_, '0'..='9') => {
                createToken(&mut tokens, &mut buffer)?;

                state = State::ParsingNumber;
                buffer.push(char);
            }

            (_, a) if matchWhitespace(a) => {
                state = State::Whitespace;

                createToken(&mut tokens, &mut buffer)?;
            }
            _ => return Err(ParsingErr::ExpressionParsing),
        }
    }

    createToken(&mut tokens, &mut buffer)?;

    Ok(tokens)
}

fn parseToken(input: String) -> Result<Token, ParsingErr> {
    if let Ok(num) = input.parse::<i32>() {
        Ok(Token::Number(num))
    } else if input.chars().all(|c| c.is_ascii_alphabetic()) {
        Ok(Token::Variable(input))
    } else {
        Err(ParsingErr::UnrecognizedExpression(input))
    }
}
