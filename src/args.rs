use std::{thread, time::Duration, env::args};

use clap::Parser;

use crate::{api::Player, Error};

#[derive(Debug, Parser, Default)]
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
    #[arg(long)]
    rand: bool,
    /// dowlands current video
    #[arg(long)]
    downland: bool,
    #[arg(long)]
    runnig: bool,
    /// prints current running video title
    #[arg(long)]
    print: bool,
}

impl Args {
    pub fn execute(&mut self) -> Result<(), Error> {
        let mut player = Player::new()?;

        match &self.link {
            Some(link) => player.data.url = Some(link.into()),
            None => { if args().len() == 1 {  player.chose_from_list()? } } ,
        }

        if player.data.url.is_some() {
            player.start(None)?;
            thread::sleep(Duration::from_secs(1));
        }

        match self.seek {
            Some(seek) => player.seek(seek, mpvipc::SeekOptions::Absolute)?,
            None => {}, 
        }

        match self.volume {
            Some(volume) => player.volume(volume)?,
            None => {},
        }

        if self.p_volume { player.print_volume()? }

        if self.kill { player.kill()? }
        if self.next { player.next()? }
        if self.prev { player.prev()? }
        if self.toogle { player.toggle()? }
        if self.loop_playlist { player.loop_playlist()? } 
        if self.loop_single { player.loop_single()? } 
        if self.rand  { player.rand()? } 

        if self.downland { player.downland(None)? }
        if self.runnig { println!("{}", player.is_paused()?) }
        if self.print { player.print()? }

        Ok(())
    }
}
