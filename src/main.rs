#![allow(non_snake_case)]

use std::env;
use std::fs;

use types::engine::Storage;
use types::error::RunErr;

use crate::types::error::Error;

#[cfg(test)]
mod tests;

mod interpreter;
mod parser;
mod types;
mod util;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // This is a hack but it looks nice (better than using the return keyword)
        Err(RunErr::MissingInput)?;
    }

    // Get the locations where we should read the file from and where to save it.
    let input_filename: String = args[1].to_owned();
    let output_filename: &str = "output.txt";

    // Try to load the file and parse it into `Content`
    let loaded_file = fs::read_to_string(input_filename)?;

    // Try and parse the file
    let parsed_file = parser::parseFile(loaded_file)?;

    // Initialize the storage
    let mut storage: Storage = Storage::new();

    // Run the interpreter on the parsed file and pass in the mutable reference to the storage.
    // This is going to be useful later, when we're going to be parsing and interpreting multiple files in the row
    // while needing to retain the memory of variables initialized and modified in previous files.
    let output = interpreter::run(&mut storage, parsed_file)?;

    // Save the output to file
    fs::write(output_filename, output)?;

    Ok(())
}
