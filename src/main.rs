#![allow(non_snake_case)]

use std::collections::HashMap;
use std::env;
use std::fs;

use crate::types::{Command, Content, Error, RuntimeErr};
use crate::util::modifyVariable;

#[cfg(test)]
mod tests;
mod types;
mod util;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::MissingInput);
    }

    // Get the locations where we should read the file from and where to save it.
    let input_filename: &String = &args[1];
    let output_filename: String = "output.txt".to_string();

    // Try to load the file and parse it into `Content`
    let loaded_file = fs::read_to_string(input_filename)?;
    let parsed_file = parseFile(loaded_file)?;

    // Start interpreting the file
    let mut output: String = String::new();

    let mut variables: HashMap<String, i32> = HashMap::new();

    for chunk in parsed_file {
        match chunk {
            Content::Text(t) => {
                // If the `Content` is just a chunk of text, simply add it to the output.
                output += &t;
                Ok(())
            }

            // If the `Content` is a command, execute it.
            Content::Command(Command::Let(variable, val)) => {
                // This is a kinda weird bit of code (both because of the default rust formatting and because it's written that way)
                // but what I wanted to do was to simply return an error if the `insert` function returned Some().
                // This is because the `let` command is supposed to initialize a variable, and if it returns Some(), it means that there was a variable
                // with that name.
                // Frankly, I'm not sure if this divide between the `let` and `set` functions is necessary, but I'm gonna try and go with it for now.
                variables.insert(variable, val).map_or(Ok(()), |_| {
                    Err(Error::Runtime(
                        RuntimeErr::TriedToInitializeExistingVariable,
                    ))
                })
            }

            Content::Command(Command::Set(variable, val)) => modifyVariable(
                &mut variables,
                variable,
                |_| val,
                Error::Runtime(RuntimeErr::TriedToModifyNonexistentVariable),
            ),

            Content::Command(Command::Add(variable, val)) => modifyVariable(
                &mut variables,
                variable,
                |v| v + val,
                Error::Runtime(RuntimeErr::TriedToModifyNonexistentVariable),
            ),

            Content::Command(Command::Subtract(variable, val)) => modifyVariable(
                &mut variables,
                variable,
                |v| v - val,
                Error::Runtime(RuntimeErr::TriedToModifyNonexistentVariable),
            ),
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
fn parseFile(input: String) -> Result<Vec<Content>> {
    let mut result: Vec<Content> = vec![];

    let mut reading_command: bool = false;
    let mut buffer: String = String::new();

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
