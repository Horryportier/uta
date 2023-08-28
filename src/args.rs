use crate::api::is_runing;
use crate::{api::Player, Error};

use crate::utils::print_help;

#[derive(Debug)]
pub enum Arg {
    Link(String),
    Seek(usize),
    Volume(Option<f64>),

    HelpFlag,

    Start,
    Kill,
    Next,
    Prev,
    Toogle,
    LoopPlaylist,
    Loop,
    Rand,
    Downland,

    Print,

    Other,
}

#[derive(Debug)]
pub struct ArgsManger {
    pub args: Vec<Arg>,
}

impl ArgsManger {
    pub fn new(input: &Vec<String>) -> ArgsManger {
        let args = ArgsManger::parse(input.to_vec());
        ArgsManger { args }
    }

    fn parse(args: Vec<String>) -> Vec<Arg> {
        let mut a: Vec<Arg> = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            a.push(match arg.as_str() {
                "-h" | "--help" => Arg::HelpFlag,
                "--start" => Arg::Start,
                "-k" | "--kill" => Arg::Kill,
                "-n" | "--next" => Arg::Next,
                "-p" | "--prev" => Arg::Prev,
                "-t" | "--toogle" => Arg::Toogle,
                "-r" | "--rand" => Arg::Rand,
                "-l" | "--loop" => Arg::Loop,
                "-lp" | "--loop_playlist" => Arg::LoopPlaylist,
                "-d"  => Arg::Downland,
                "--print" => Arg::Print,
                "-v" | "--volume" => Arg::Volume(match args.get(i + 1) {
                    None => None,
                    Some(f) => Some(f.parse::<f64>().unwrap_or(50.)),
                }),
                "-s" | "-seek" => Arg::Seek(
                    args.get(i + 1)
                        .unwrap_or(&"0".into())
                        .parse::<usize>()
                        .unwrap_or(0),
                ),
                arg if is_numeric(arg) => Arg::Other,
                //arg if is_valid_link(&arg) => Arg::Link(arg.into()), impl later
                _ => Arg::Link(arg.into()),
            });
        }
        a
    }

    pub fn execute(&self) -> Result<(), Error> {
        let mut player: Player = Player::new().map_err(|e| Error::MpvError(e))?;

        if !is_runing() {
            let link = self.args.iter().find_map(|f| match f {
                Arg::Link(link) => Some(link.clone()),
                _ => None,
            });
            match link {
                None => Err(Error::ExecuteErr(
                    "no link ink args\n mvp is not runing so you need to provide url".into(),
                )),
                Some(_) => Ok(())
            }?;
        }

        
        if self.args.len() == 0 {
            player.chose_from_list()?;
        }
        info!("Player =>{:?}", player);

        for (i,arg) in self.args.iter().enumerate() {
            match arg {
                Arg::Link(link) => player.data.url = Some(link.into()),
                Arg::HelpFlag => print_help(),
                Arg::Start => {
                    player.start(None)?;
                    if player.mpv.is_none() {
                        player.mpv = Some(
                            mpvipc::Mpv::connect("/tmp/mpvsocket")
                                .map_err(|e| Error::MpvError(e))?,
                        );
                    }
                }
                Arg::Kill => player.kill()?,
                Arg::Next => player.next()?,
                Arg::Prev => player.prev()?,
                Arg::Toogle => player.toggle()?,
                Arg::Print => player.print()?,
                Arg::Seek(dest) => player.seek(
                    dest.to_string().parse::<f64>().unwrap_or(0.),
                    mpvipc::SeekOptions::AbsolutePercent,
                )?,
                Arg::Loop => player.loop_single()?,
                Arg::LoopPlaylist => player.loop_playlist()?,
                Arg::Rand => player.rand()?,
                Arg::Volume(volume) => match volume {
                    Some(volume) => player.volume(*volume)?,
                    None => player.get_volume()?,
                },
                Arg::Downland => {
                    let url: Option<&str> = match self.args.get(i+1) {
                        None => None,
                        Some(a) => match  a {
                            Arg::Link(l) => Some(l),
                            _ => None
                        }
                    };
                    player.downland(url)?;
                },
                _ => {}
            }
        }

        Ok(())
    }
}

pub fn is_numeric(input: &str) -> bool {
    match input.parse::<usize>() {
        Ok(_) => true,
        Err(..) => false,
    }
}

#[allow(dead_code)]
fn is_valid_link(url: &str) -> bool {
    let s: Vec<&str> = url.split("/").collect();
    info!("{:?}", s);
    true
}
