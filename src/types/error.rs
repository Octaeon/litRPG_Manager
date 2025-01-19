use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IO(String),
    Parse(ParsingErr),
    Runtime(RunErr),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingErr {
    UnrecognizedExpression(String),
    ExpressionParsing,
    StringOverflow,
    InvalidNumberOfArguments,
    CommandLeftOpen,
    UnrecognizedCommand(String),
    NumberParsing(std::num::ParseIntError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunErr {
    MissingInput,
    TriedToInitializeExistingVariable,
    TriedToModifyNonexistentVariable,
    TriedToGetNonexistentVariable,
}

impl From<RunErr> for Error {
    fn from(err: RunErr) -> Self {
        Error::Runtime(err)
    }
}

impl From<ParsingErr> for Error {
    fn from(err: ParsingErr) -> Self {
        Error::Parse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(format!("{err}"))
    }
}

impl From<std::num::ParseIntError> for ParsingErr {
    fn from(err: std::num::ParseIntError) -> ParsingErr {
        ParsingErr::NumberParsing(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(io_err) => write!(f, "[IO] {io_err}"),
            Error::Parse(parsing_err) => write!(f, "[Parsing] {parsing_err}"),
            Error::Runtime(run_err) => write!(f, "[Runtime] {run_err}"),
        }
    }
}

impl Display for ParsingErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParsingErr::UnrecognizedExpression(expr) =>
                    format!("Unrecognized expression: {expr}"),
                ParsingErr::ExpressionParsing =>
                    String::from("There was an error with expression parsing!"),
                ParsingErr::StringOverflow =>
                    String::from("Somehow, the contents of the file caused a string overflow."),
                ParsingErr::InvalidNumberOfArguments =>
                    String::from("Invalid number of arguments provided to a command"),
                ParsingErr::CommandLeftOpen =>
                    String::from("The command at the end of the file was left open"),
                ParsingErr::UnrecognizedCommand(command) =>
                    format!("Unrecognized command: {command}"),
                ParsingErr::NumberParsing(parse_int_error) =>
                    format!("Failed conversion to i32: '{parse_int_error}'"),
            }
        )
    }
}

impl Display for RunErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RunErr::MissingInput => "Input the name of the file as the first argument",
                RunErr::TriedToInitializeExistingVariable =>
                    "Tried to initialize already existing variable",
                RunErr::TriedToModifyNonexistentVariable =>
                    "Tried to modify a nonexistent variable",
                RunErr::TriedToGetNonexistentVariable => "Tried to get a nonexistent variable",
            }
        )
    }
}

impl std::error::Error for Error {}
impl std::error::Error for ParsingErr {}
impl std::error::Error for RunErr {}
