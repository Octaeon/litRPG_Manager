use std::fmt::Display;

pub mod engine;
pub mod error;

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
    Write(String),
}

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
            "[Command] {} |",
            match self {
                Self::Let(var, v) => format!("Let {var} be {v}"),
                Self::Set(var, v) => format!("Set {var} to be {v}"),
                Self::Add(var, v) => format!("Add {v} to {var}"),
                Self::Subtract(var, v) => format!("Subtract {v} from {var}"),
                Self::Write(var) => format!("Write the variable {var}"),
            }
        )
    }
}
