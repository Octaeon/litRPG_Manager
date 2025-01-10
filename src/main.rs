#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::ops::Not;

use types::engine::Storage;
use types::error::ParsingErr;
use types::error::RunErr;

use crate::types::error::Error;
use crate::types::{Command, Content};

#[cfg(test)]
mod tests;
mod types;
mod util;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // This is a hack but it looks nice
        Err(RunErr::MissingInput)?;
    }

    // Get the locations where we should read the file from and where to save it.
    let input_filename: &String = &args[1];
    let output_filename: String = "output.txt".to_string();

    // Try to load the file and parse it into `Content`
    let loaded_file = fs::read_to_string(input_filename)?;
    let parsed_file = parseFile(loaded_file)?;

    // Start interpreting the file
    let mut output: String = String::new();

    let mut storage: Storage = Storage::new();

    for chunk in parsed_file {
        match chunk {
            Content::Text(t) => {
                // If the `Content` is just a chunk of text, simply add it to the output.
                output += &t;
                Ok(())
            }

            // If the `Content` is a command, execute it.
            Content::Command(Command::Let(variable, val)) => storage.createVariable(variable, val),

            Content::Command(Command::Set(variable, val)) => {
                storage.modifyVariable(variable, |_| val)
            }
            Content::Command(Command::Add(variable, val)) => {
                storage.modifyVariable(variable, |og| og + val)
            }

            Content::Command(Command::Subtract(variable, val)) => {
                storage.modifyVariable(variable, |og| og - val)
            }
            Content::Command(Command::Write(variable)) => match storage.getValue(variable) {
                Ok(val) => {
                    output += &val.to_string();
                    Ok(())
                }
                Err(err) => Err(err),
            },
        }?;
    }

    // Save the output to file
    fs::write(output_filename, output)?;

    Ok(())
}

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
fn parseFile(input: String) -> Result<Vec<Content>, ParsingErr> {
    let mut result: Vec<Content> = vec![];

    let mut reading_command: bool = false;
    let mut buffer: String = String::new();

    for char in input.chars() {
        match char {
            '$' => {
                // If we encounter a $, it means that we're either at the beginning of the command, or at the end
                // If `reading_command` flag is true, it means we were at the end of one, so try and parse it and set the flag to false.
                // If it's false, then we're at the beginning of one, so set the flag to true.
                let chunk = if reading_command {
                    parseCommand(buffer.clone()).map(|o| Content::Command(o))
                } else {
                    Ok(Content::Text(buffer.clone()))
                }?;

                reading_command = !reading_command;
                result.push(chunk);
                buffer = String::new();
            }
            _ => buffer.push(char),
        }
    }

    if reading_command {
        // If all the characters have been read and we're still in the 'reading commands' state, it means that someone opened
        // a command statement but didn't close it, so we throw an error.
        Err(ParsingErr::CommandLeftOpen)
    } else {
        result.push(Content::Text(buffer));
        Ok(result)
    }
}

/// The program is meant to work on numbers, which are all stored as integers. No floating point numbers.
///
/// List of commands:
/// - let : creates a variable and initializes it with the given value. Example: ```let variable 0```
/// - add : adds a value to a variable. Example: ```add variable 10```
/// - subtract : subtracts a value from a variable. Example: ```sub variable 10```
/// - set : sets a variable to a new value. Example: ```set variable -10```
fn parseCommand(input: String) -> Result<Command, ParsingErr> {
    let words: Vec<&str> = input.split(' ').filter(|c| !c.is_empty()).collect();

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

    match words[0] {
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
            checkNumOfArguments(1)?;
            Ok(Command::Write(words[1].to_string()))
        }
        other_command => Err(ParsingErr::UnrecognizedCommand(other_command.to_string())),
    }
}
