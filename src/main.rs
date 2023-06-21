use std::env::args;

use args::ArgsManger;

use crate::api::Player;

mod api;
mod args;
mod utils;

enum Error {
   ExecuteErr(String) 
}


fn main() {
    let args = args().collect::<Vec<String>>();
    let args = &args[1..];
    let arg_menager = ArgsManger::new(&args.to_vec());
    println!("{:?}", arg_menager.args);
    arg_menager.execute();

    // let p = Player::new("https://www.youtube.com/playlist?list=PL3AyWHvr_UThgm3gIBroTMRKEhV5zrQEl");
    // println!("{:?}", p.start());
}

