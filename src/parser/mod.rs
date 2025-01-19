use crate::types::error::ParsingErr;
use crate::types::{Command, Content};

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
pub fn parseCommand(input: String) -> Result<Vec<Command>, ParsingErr> {
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
                let _a = parseExpression(words[1..].to_vec())?;
                Ok(Command::Write(words[1].to_string()))
            }
            other_command => Err(ParsingErr::UnrecognizedCommand(other_command.to_string())),
        }?;

        result_commands.push(command);
    }

    Ok(result_commands)
}

fn parseExpression(_input: Vec<&str>) -> Result<(), ParsingErr> {
    Ok(())
}
