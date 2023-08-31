use std::{env::args, io, string::FromUtf8Error, num::ParseIntError};

use args::ArgsManger;
use thiserror::Error;

#[macro_use]
extern crate log;

mod api;
mod args;
mod utils;
mod env;

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
    IntErr(#[from] ParseIntError)
}

fn main() {
    pretty_env_logger::init();

    let args = args().collect::<Vec<String>>();
    let args = &args[1..];
    let arg_menager = ArgsManger::new(&args.to_vec());
    info!("{:?}", arg_menager.args);
    let res = arg_menager.execute();
    match res {
        Ok(_) => info!("App exeucted without issues"),
        Err(err) => error!("App failed at {}", err),
    }
}
