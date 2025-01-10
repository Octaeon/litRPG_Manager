use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    MissingInput,
    InvalidNumberOfArguments,
    CommandLeftOpen,
    UnrecognizedCommand(String),
    Runtime(RuntimeErr),
    Parse(std::num::ParseIntError),
    IO(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeErr {
    TriedToInitializeExistingVariable,
    TriedToModifyNonexistentVariable,
}

impl From<RuntimeErr> for Error {
    fn from(err: RuntimeErr) -> Self {
        Error::Runtime(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::Parse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(format!("{err}"))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingInput => write!(f, "Input the name of the file as the first argument"),
            Error::InvalidNumberOfArguments => {
                write!(f, "Invalid number of arguments provided to a command")
            }
            Error::UnrecognizedCommand(c) => write!(f, "Unrecognized command '{c}'"),
            Error::Parse(parse_int_error) => write!(f, "{parse_int_error}"),
            Error::IO(error) => write!(f, "{error}"),
            Error::CommandLeftOpen => write!(f, "The command at the end of the file was left open"),
            Error::Runtime(runtime_err) => match runtime_err {
                RuntimeErr::TriedToInitializeExistingVariable => {
                    write!(f, "[Runtime] Tried to initialize already existing variable")
                }
                RuntimeErr::TriedToModifyNonexistentVariable => {
                    write!(f, "[Runtime] Tried to modify a nonexistent variable")
                }
            },
        }
    }
}

impl std::error::Error for Error {}
