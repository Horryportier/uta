use crate::{Error, utils::print_help, api::Player};


#[derive(Debug)]
pub enum Arg {
    Link(String),
    Seek(usize),

    HelpFlag,

    Start,
    Kill,
    Next,
    Prev,
    Toogle,
    LoopPlaylist,
    Loop,
    Rand,

    Print,

    Other
}

#[derive(Debug)]
pub struct ArgsManger {
    pub args: Vec<Arg>,
}


impl ArgsManger {
    pub fn new(input: &Vec<String>) -> ArgsManger{
        let args = ArgsManger::parse(input.to_vec());
        ArgsManger { args }
    }

    fn parse(args: Vec<String>) -> Vec<Arg>{
        let mut a: Vec<Arg> = Vec::new();
        for (i,arg) in  args.iter().enumerate() {
            a.push(match arg.as_str() {
                "-h" | "--help" => Arg::HelpFlag,
                "--start" => Arg::Start,
                "-k" | "--kill" => Arg::Kill,
                "-n" | "--next" => Arg::Next,
                "-p" | "--prev" => Arg::Prev,
                "-t" | "--toogle" => Arg::Toogle,
                "-r" | "--rand" => Arg::Rand,
                "-l" | "--loop" => Arg::Loop,
                "--loop_playlist"  => Arg::LoopPlaylist,
                "--print" => Arg::Print,
                "-s" | "-seek" =>   
                    Arg::Seek(args.get(i+1)
                        .unwrap_or(&"0".into())
                        .parse::<usize>().unwrap_or(0)),
                arg if is_numeric(arg) =>  Arg::Other,
                //arg if is_valid_link(&arg) => Arg::Link(arg.into()), impl later 
                _ =>  Arg::Link(arg.into())
            });
        }    
        a
    }

    pub fn  execute(&self) -> Result<(), Error>  {

        let link = self.args.iter().find_map(|f| match f {
            Arg::Link(link) => Some(link.clone()),
            _ => Some("".into()),
        });

        if let None = link {
           return Err( Error::ExecuteErr("no link specifived".into()));
        }
        let player = Player::new(&link.unwrap());

        for arg in &self.args {
            match arg {
                Arg::HelpFlag => {print_help();}, 
                Arg::Start =>  { 
                    println!("Start {:?}", player.start());} ,
                Arg::Kill => {
                    println!("Kill {:?}", player.kill());
                },
                _ => {}
            }
        }
        Ok(())
    }
}


pub fn is_numeric(input: &str) -> bool {
    match input.parse::<usize>(){
        Ok(_) => true,
        Err(..) => false,
    }
}

#[allow(dead_code)]
fn is_valid_link(url: &str) -> bool {
    let s: Vec<&str> = url.split("/").collect();
    println!("{:?}",s);
    true
    
}
