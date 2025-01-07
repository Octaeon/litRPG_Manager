#![allow(non_snake_case)]

use std::env;
use std::fs;

use crate::types::{Command, Content};

pub mod types;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Input the name of the file as the first argument!");
        return Err(());
    }

    let filename: &String = &args[1];
    let loaded_file = fs::read_to_string(filename);

    match loaded_file {
        Ok(contents) => {
            let output: Vec<Content> = parseFile(contents)?;

            for chunk in output {
                println!("{:?}", chunk);
            }

            fs::write("output.txt", "test")
                .expect("For some reason, couldn't write the output to file");
            Ok(())
        }
        Err(e) => {
            println!("{e}");
            Err(())
        }
    }
}

fn parseFile(input: String) -> Result<Vec<Content>, ()> {
    let mut result: Vec<Content> = vec![];

    let mut reading_command: bool = false;
    let mut buffer: String = "".to_string();

    for char in input.chars() {
        match char {
            '$' => {
                let chunk = if reading_command {
                    parseCommand(buffer.clone())
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
        Err(())
    } else {
        result.push(Content::Text(buffer));
        Ok(result)
    }
}

fn parseCommand(_input: String) -> Result<Content, ()> {
    Ok(Content::Command(Command::AddOne))
}
