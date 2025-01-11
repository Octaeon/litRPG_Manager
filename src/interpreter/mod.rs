use crate::types::{engine::Storage, error::RunErr, Command, Content};

pub fn run(storage: &mut Storage, parsed_file: Vec<Content>) -> Result<String, RunErr> {
    // Start interpreting the file
    let mut output: String = String::new();

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
        }?
    }

    Ok(output)
}
