use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RyazonError {
    NoPath,
    EmptyChain,
    MaxMinWords,
    TerminatorNotFound(String),
    IoError(String),
}

impl fmt::Display for RyazonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RyazonError::NoPath => write!(f, "no path provided"),
            RyazonError::EmptyChain => write!(f, "the chain is empty"),
            RyazonError::MaxMinWords => write!(f, "max words must be greater than min words"),
            RyazonError::TerminatorNotFound(msg) => write!(f, "terminator not found: {}", msg),
            RyazonError::IoError(err) => write!(f, "io error: {}", err),
        }
    }
}

impl std::error::Error for RyazonError {}

impl From<std::io::Error> for RyazonError {
    fn from(err: std::io::Error) -> Self {
        RyazonError::IoError(err.to_string())
    }
}
