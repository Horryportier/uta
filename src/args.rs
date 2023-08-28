use std::fmt::Display;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use crate::{api::Player, Error};

#[derive(Debug)]
pub enum Arg {
    Link(String),
    Seek(usize),
    Volume(Option<f64>),

    HelpFlag,

    Kill,
    Next,
    Prev,
    Toogle,
    LoopPlaylist,
    Loop,
    Rand,
    Downland,

    Runnig,

    Print,

    Other(String),

    None,
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match &self {
            Arg::Runnig => format!("-rn will print if mpv is paused"),
            Arg::Kill => format!("-k --kill will kill mpv procceses"),
            Arg::Seek(_) => format!("-s --seek given number betwwen 0..100\n\twill jump to that point in video"),
            Arg::Next => format!("-n --next will jump to next entry in playlist"),
            Arg::Prev => format!("-p --prev will jump to prev entry in playlist"),
            Arg::Volume(_)=> format!("-v --volume when passed number will change volume\n\t to that number clamped betwwen 0..100\n\t if not passed will print current volume"),
            Arg::Toogle => format!("-t --toogle toggels betwwen paused/not paused"),
            Arg::LoopPlaylist => format!("-lp --loop_playlist sets playlist to looping mode"),
            Arg::Loop => format!("-l --loop sets current entry to looping mode"),
            Arg::Link(_) => format!("passing link will start mpv with socket at `/tmp/mpvsocket`"),
            Arg::Other(_) => format!(""),
            Arg::HelpFlag => format!("-h --help shows help"),
            Arg::Rand => format!("-r -rand jumps to random entry in playlist"),
            Arg::Print => format!("--print prints current entry name"),
            Arg::Downland => format!("-d if prowided link will downland that video using yt-dlp\n\t if not and mpv is runnig then will downland current entry as video"),
            Arg::None => format!("passing non arguments will print\n\t list of playlist from UTA_LIST_FILE env if set")
        };
        writeln!(f, "{}", text)
    }
}

#[derive(Debug)]
pub struct ArgsManger {
    pub args: Vec<Arg>,
}

impl Display for ArgsManger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text: Vec<String> = vec![
            "\t\tUta".into(),
            "uta is an basic mpv music player".into(),
            "you can use your own local playlist or yt url".into(),
            "ENV:".into(),
            "UTA_DOWNLAND args for yt-dlp".into(),
            "UTA_LIST_FILE when set will read from file and give you optinos of playlist to start"
                .into(),
            "file has to be in format of".into(),
            "playlist1 URL1".into(),
            "playlist2 URL2".into(),
            "ARGS:".into(),
        ];

        let args = vec![
            Arg::None,
            Arg::Link("".into()),
            Arg::Runnig,
            Arg::Kill,
            Arg::Seek(0),
            Arg::Next,
            Arg::Prev,
            Arg::Toogle,
            Arg::LoopPlaylist,
            Arg::Loop,
            Arg::Rand,
            Arg::Downland,
            Arg::Runnig,
            Arg::Print,
            Arg::Other("".into()),
            Arg::HelpFlag,
            Arg::Volume(None),
        ];

        args.iter().for_each(|f| text.push(format!("{}", f)));

        write!(f, "{}", text.join("\n"))
    }
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
                "-k" | "--kill" => Arg::Kill,
                "-n" | "--next" => Arg::Next,
                "-p" | "--prev" => Arg::Prev,
                "-t" | "--toogle" => Arg::Toogle,
                "-r" | "--rand" => Arg::Rand,
                "-l" | "--loop" => Arg::Loop,
                "--lp" | "--loop_playlist" => Arg::LoopPlaylist,
                "-d" => Arg::Downland,
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
                arg if is_numeric(arg) => Arg::Other(arg.into()),
                arg if is_valid_link(&arg) => Arg::Link(arg.into()),
                "-rn" => Arg::Runnig,
                _ => Arg::Other(arg.into()),
            });
        }
        a
    }

    pub fn execute(&self) -> Result<(), Error> {
        let mut player: Player = Player::new()?;

        if self.args.len() == 0 {
            player.chose_from_list()?;
        }

        info!("Player =>{:?}", player);

        for (i, arg) in self.args.iter().enumerate() {
            match arg {
                Arg::None => {}
                Arg::HelpFlag => match self.args.get(i + 1) {
                    None => println!("{}", self),
                    Some(a) => {
                        println!("{}", a);
                        return Ok(());
                    }
                },
                Arg::Link(url) => {
                    player.data.url = Some(url.into());
                    player.start(None)?;
                    if player.mpv.lock().unwrap().is_none() {
                        sleep(Duration::from_secs(1));
                        player.mpv = Some(mpvipc::Mpv::connect("/tmp/mpvsocket")?).into();
                    }
                }
                Arg::Kill => player.kill()?,
                Arg::Next => player.next()?,
                Arg::Prev => player.prev()?,
                Arg::Toogle => {
                    player.toggle()?;
                    sleep(Duration::from_secs(1));
                    println!(
                        "mpv is {}",
                        match player.is_paused()? {
                            false => "runnig",
                            true => "not runnig",
                        }
                    )
                }

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
                    let url: Option<&str> = match self.args.get(i + 1) {
                        None => None,
                        Some(a) => match a {
                            Arg::Link(l) => Some(l),
                            _ => None,
                        },
                    };
                    player.downland(url)?;
                }
                Arg::Runnig => println!(
                    "mpv is {}",
                    match player.is_paused()? {
                        false => "runnig",
                        true => "not runnig",
                    }
                ),
                Arg::Other(a) => println!("{} is an invalid argument\n use \"uta -h\" for help", a),
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

fn is_valid_link(url: &str) -> bool {
    if Path::new(url).exists() {
        return true;
    }

    let mut is_valid = false;
    let valid_links_prefixes = ["https://youtube.com/", "youtube.com/"];
    valid_links_prefixes.iter().for_each(|f| {
        is_valid = url.contains(f);
    });
    is_valid
}
