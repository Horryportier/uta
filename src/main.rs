use std::{env::args, io};

use args::ArgsManger;

#[macro_use]
extern crate log;

mod api;
mod args;
mod utils;
mod env;

#[derive(Debug)]
pub enum Error {
    ExecuteErr(String),
    MpvError(mpvipc::Error),
    IoErr(io::Error),
    SerdeErr(serde_json::Error),
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
        Err(err) => error!("App failed at {:?}", err),
    }
}
