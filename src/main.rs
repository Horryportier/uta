use std::{io, num::ParseIntError, string::FromUtf8Error};

use clap::Parser;
use thiserror::Error;

use crate::args::Args;

#[macro_use]
extern crate log;

mod api;
mod args;
mod env;
mod utils;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Executer errror issue at {0}")]
    ExecuteErr(String),
    #[error("mpvipc error: {0}")]
    MpvError(#[from] mpvipc::Error),
    #[error("io error: {0}")]
    IoErr(#[from] io::Error),
    #[error("serde error: {0}")]
    SerdeErr(#[from] serde_json::Error),
    #[error("utf8 error: {0}")]
    UtfErr(#[from] FromUtf8Error),
    #[error("parse int error:  {0}")]
    IntErr(#[from] ParseIntError),
}

fn main() {
    pretty_env_logger::init();
    let mut args = Args::parse();

    match args.execute()  {
        Ok(_) => {},
        Err(e) => error!("uta failed at: {}", e)
    }
}
