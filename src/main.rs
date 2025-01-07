#![allow(non_snake_case)]

use std::env;
use std::fs;

use crate::types::{Command, Content, Error};

pub mod types;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // TODO : Parse the args vector to accept optional commands (like --input "filename.txt" --output "anotherfilename.txt")

    if args.len() < 2 {
        return Err(Error::MissingInput);
    }

    let input_filename: &String = &args[1];
    let output_filename: String = "output.txt".to_string();
    let loaded_file = fs::read_to_string(input_filename);

    match loaded_file {
        Ok(file_contents) => {
            let parsed_file: Vec<Content> = parseFile(file_contents)?;

            fs::write(output_filename, "test")?;
            Ok(())
        }
        Err(e) => {
            println!("{e}");
            Err(Error::IO(e.to_string()))
        }
    }
}

fn parseFile(input: String) -> Result<Vec<Content>> {
    let mut result: Vec<Content> = vec![];

    let mut reading_command: bool = false;
    let mut buffer: String = "".to_string();

    for char in input.chars() {
        match char {
            '$' => {
                let chunk = if reading_command {
                    parseCommand(buffer.clone()).map(|o| Content::Command(o))
                } else {
                    Ok(Content::Text(buffer.clone()))
                };

                result.push(chunk?);
                reading_command = !reading_command;
                buffer = "".to_string();
            }
            _ => buffer.push(char),
        }
    }

    if reading_command {
        // If all the characters have been read and we're still in the 'reading commands' state, it means that someone opened
        // a command statement but didn't close it, so we throw an error.
        Err(Error::CommandLeftOpen)
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
fn parseCommand(input: String) -> Result<Command> {
    let words: Vec<&str> = input.split(' ').filter(|c| !c.is_empty()).collect();

    let amount_of_words = words.len();

    if amount_of_words != 3 {
        // As of the moment, all of the commands have exactly three words in them, so if we try to parse anything that has
        // either more or less words than three, we know it's invalid.
        return Err(Error::InvalidNumberOfArguments);
    }

    match words[0] {
        "let" => Ok(Command::Let(words[1].to_string(), words[2].parse::<i32>()?)),
        "add" => Ok(Command::Add(words[1].to_string(), words[2].parse::<i32>()?)),
        "subtract" => Ok(Command::Subtract(
            words[1].to_string(),
            words[2].parse::<i32>()?,
        )),
        "set" => Ok(Command::Set(words[1].to_string(), words[2].parse::<i32>()?)),
        other_command => Err(Error::UnrecognizedCommand(format!("{other_command}"))),
    }
}
