use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    MissingInput,
    InvalidNumberOfArguments,
    CommandLeftOpen,
    UnrecognizedCommand(String),
    Parse(std::num::ParseIntError),
    IO(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    Text(String),
    Command(Command),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Let(String, i32),
    Set(String, i32),
    Add(String, i32),
    Subtract(String, i32),
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
        }
    }
}

impl std::error::Error for Error {}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(t) => write!(f, "{t}"),
            Self::Command(c) => write!(f, "{c}"),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Command] {}",
            match self {
                Self::Let(var, v) => format!("Let {var} be {v}"),
                Self::Set(var, v) => format!("Set {var} to be {v}"),
                Self::Add(var, v) => format!("Add {v} to {var}"),
                Self::Subtract(var, v) => format!("Subtract {v} from {var}"),
            }
        )
    }
}
