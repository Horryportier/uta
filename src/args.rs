use std::{env::args, thread, time::Duration};
use crossterm::style::Stylize;

use clap::Parser;

macro_rules! if_arg {
    ($bool:expr, $msg:literal, $fun:expr) => {
        if $bool {
            println!("{}", $msg.cyan());
            $fun;
            println!("{}", "succes".green());
        }
    };
    ($bool:expr, $fun:expr) => {
        if $bool {
            $fun;
        }
    };
}

const LONG_ABOUT: &str = r#"
uta is mpv wrapper specialized in being youtube music player. 

Env Vars:

UTA_VOLUME=<0-100>      uta default volume is set to 50.
UTA_VIDEO=true/false    set to false by default.
UTA_DOWNLAND=<args>     var to specify yt-dlp args like safe path etc.
"#;

use crate::{api::Player, Error};
///uta is mpv wrapper specialized in being youtube music player
#[derive(Debug, Parser, Default)]
#[command(author="Horyyportier", version,long_about=Some(LONG_ABOUT))]
pub struct Args {
    /// link to youtube video/playlist
    #[arg(short, long)]
    link: Option<String>,
    /// jump to timestamp in video by procantage
    #[arg(short, long)]
    seek: Option<f64>,
    /// change volume
    #[arg(short, long)]
    volume: Option<f64>,
    /// prints volume
    #[arg(long)]
    p_volume: bool,
    /// prints percentage of the video
    #[arg(long)]
    percentage: bool,
    /// kill's mpv process
    #[arg(short, long)]
    kill: bool,
    /// jump to next entry in playlist
    #[arg(short, long)]
    next: bool,
    /// jump to previous entry in playlist
    #[arg(short, long)]
    prev: bool,
    /// toggles betwenn paused and unpuased
    #[arg(short, long)]
    toogle: bool,
    /// sets mpv to loop current playlist
    #[arg(long)]
    loop_playlist: bool,
    /// sets mpv to loop current video
    #[arg(long)]
    loop_single: bool,
    /// jumps to random entry in the playlist
    #[arg(short, long)]
    rand: bool,
    /// dowlands current video
    #[arg(long)]
    downland: bool,
    #[arg(long)]
    runnig: bool,
    /// prints current running video title
    #[arg(long)]
    print: bool,
    /// saves thumbnail at said path if not provided will save at curr dir
    #[arg(long)]
    thumbnail: Option<String>,
    /// prints link to current track
    #[arg(long)]
    get_link: bool,
}

impl Args {
    pub fn execute(&mut self) -> Result<(), Error> {
        let mut player = Player::new()?;

        match &self.link {
            Some(link) => player.data.url = Some(link.into()),
            None => {
                if args().len() == 1 {
                    player.chose_from_list()?
                }
            }
        }

        if_arg!(player.data.url.is_some(), {
            player.start(None)?;
            thread::sleep(Duration::from_secs(1))
        });

        match self.seek {
            Some(seek) => player.seek(seek, mpvipc::SeekOptions::AbsolutePercent)?,
            None => {}
        }

        match self.volume {
            Some(volume) => player.volume(volume)?,
            None => {}
        }

        if_arg!(self.kill, "killing current instance of mpv", player.kill()?);

        if_arg!(self.next, "playing next entry in playlist", player.next()?);
        if_arg!(self.prev, "playing prev entry in playlist", player.prev()?);
        if_arg!(self.toogle, "toogle player on/off", player.toggle()?);
        if_arg!(
            self.loop_playlist,
            "looping playlist",
            player.loop_playlist()?
        );
        if_arg!(
            self.loop_single,
            "looping current entry",
            player.loop_single()?
        );
        if_arg!(
            self.rand,
            "jumping to random entry in playlist",
            player.rand()?
        );
        if_arg!(self.p_volume, player.print_volume()?);

        if_arg!(
            self.downland,
            "dowlandnig current entry",
            player.downland(None)?
        );
        if_arg!(self.runnig, println!("{}", player.is_paused()?));
        if_arg!(self.print, player.print()?);
        if_arg!(
            self.percentage,
            print!("{}", player.get_procentage()? as usize)
        );
        if_arg!(self.get_link, player.get_link()?);

        match &self.thumbnail {
            Some(th) => player.safe_thumbnail(th.into())?,
            None => (),
        }

        Ok(())
    }
}
