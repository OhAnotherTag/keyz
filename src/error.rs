use std::io;

use thiserror::Error;


#[derive(Error, Debug)]
pub enum KvsError {
  #[error("{}", _0)]
  Io(#[from] io::Error),

  #[error("{}", _0)]
  Serde(#[from] serde_json::Error),

  #[error("Key not found")]
  KeyNotFound,

  #[error("Unexpected command type")]
  UnexpectedCommandType,

  #[error("Unexpected command type")]
  CommandLineParsing(#[from] clap::Error)
}

pub type Result<T> = anyhow::Result<T, KvsError>;