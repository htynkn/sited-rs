use core::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct EngineError {}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EngineError")
    }
}
