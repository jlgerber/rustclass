use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum MyError {
    NoInputError,
    IoError { problem: String },
    RuntimeError(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NoInputError => write!(f, "NoInputError()"),
            MyError::IoError { problem } => write!(f, "IoError - cause: {}", problem.as_str()),
            MyError::RuntimeError(inner) => write!(f, "RuntimeError({})", inner),
        }
    }
}

impl Error for MyError {}

fn test() -> Result<&'static str, MyError> {
    Err(MyError::RuntimeError("arggg".into()))
}

pub fn main() {
    let result = test();
    match result {
        Ok(s) => println!("ok {}", s),
        Err(e) => println!("err {}", e),
    }
}
