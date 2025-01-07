#[derive(Debug, Clone)]
pub enum Content {
    Text(String),
    Command(Command),
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    AddOne,
}
