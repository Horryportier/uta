use std::{fs, io::stdin, process::Command};

use mpvipc::{Mpv, SeekOptions};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_partial::SerializePartial;

use crate::{env::get_env_mpv_args, Error};

#[derive(Debug)]
pub struct Player {
    pub data: Data,
    pub mpv: Option<Mpv>,
}

#[derive(Serialize, Deserialize, SerializePartial, Debug)]
pub struct Data {
    pub is_runing: bool,
    pub log: bool,
    pub mpv_args: Vec<String>,
    pub url: Option<String>,

    pub previous_video: (),
    pub time: usize,
}

impl Player {
    /// path or youtube link
    pub fn new() -> Result<Player, mpvipc::Error> {
        let socket = "/tmp/mpvsocket";

        let mut def_args: Vec<String> = [
            "--no-terminal",
            format!("--input-ipc-server={}", socket).as_str(),
        ]
        .map(String::from)
        .to_vec();

        def_args.append(&mut get_env_mpv_args());

        let mpv = match Mpv::connect(socket) {
            Ok(mpv) => Some(mpv),
            Err(_) => None,
        };

        let data = Data {
            is_runing: false,
            log: false,
            mpv_args: def_args,
            previous_video: (),
            time: 0,
            url: None,
        };

        Ok(Player { data, mpv })
    }

    pub fn start(&self, url: Option<String>) -> Result<(), Error> {
        let mut final_url = String::default();
        if url.is_none() {
            match &self.data.url {
                Some(url) => final_url = url.to_string(),
                None => {
                    return Err(Error::ExecuteErr("url is empty".into()));
                }
            }
        }

        match url {
            None => {}
            Some(s) => final_url = s,
        }
        let cmd = Command::new("mpv")
            .args(&self.data.mpv_args)
            .arg(final_url)
            .spawn()
            .map_err(|e| Error::IoErr(e))?;
        info!("start cmd {:?}", cmd);
        Ok(())
    }

    pub fn kill(&self) -> Result<(), Error> {
        let cmd = Command::new("pkill")
            .arg("mpv")
            .output()
            .map_err(|e| Error::IoErr(e))?;
        info!("kill cmd {:?}", cmd);
        Ok(())
    }
    pub fn next(&self) -> Result<(), Error> {
        self.mpv
            .clone()
            .unwrap()
            .next()
            .map_err(|e| Error::MpvError(e))?;
        Ok(())
    }
    pub fn prev(&self) -> Result<(), Error> {
        self.mpv
            .clone()
            .unwrap()
            .prev()
            .map_err(|e| Error::MpvError(e))?;
        Ok(())
    }

    pub fn seek(&self, time: f64, opt: SeekOptions) -> Result<(), Error> {
        self.mpv
            .clone()
            .unwrap()
            .seek(time, opt)
            .map_err(|e| Error::MpvError(e))?;
        Ok(())
    }

    pub fn toggle(&self) -> Result<(), Error> {
        self.mpv
            .clone()
            .unwrap()
            .toggle()
            .map_err(|e| Error::MpvError(e))?;
        Ok(())
    }

    pub fn print(&self) -> Result<(), Error> {
        let curr: f64 = self
            .mpv
            .as_ref()
            .unwrap()
            .get_property("playback-time")
            .unwrap_or(0.)
            .floor();
        let len: f64 = self
            .mpv
            .as_ref()
            .unwrap()
            .get_property("duration")
            .unwrap_or(100.)
            .floor();
        let procent = ((curr / len) * 100.).floor();
        let text = format!(
            "{procent}/100% | {}",
            self.mpv
                .as_ref()
                .unwrap()
                .get_property::<String>("media-title")
                .map_err(|e| Error::MpvError(e))?
        );
        println!("{}", text);
        Ok(())
    }

    pub fn loop_single(&self) -> Result<(), Error> {
        self.mpv
            .clone()
            .unwrap()
            .set_loop_file(mpvipc::Switch::Toggle)
            .map_err(|e| Error::MpvError(e))?;
        Ok(())
    }

    pub fn loop_playlist(&self) -> Result<(), Error> {
        self.mpv
            .clone()
            .unwrap()
            .set_loop_playlist(mpvipc::Switch::Toggle)
            .map_err(|e| Error::MpvError(e))?;
        Ok(())
    }

    pub fn rand(&self) -> Result<(), Error> {
        let count: usize = self
            .mpv
            .as_ref()
            .unwrap()
            .get_property("playlist-count")
            .map_err(|e| Error::MpvError(e))?;
        let rng = rand::thread_rng().gen_range(1..count);
        self.mpv
            .as_ref()
            .unwrap()
            .playlist_play_id(rng)
            .map_err(|e| Error::MpvError(e))?;
        Ok(())
    }

    pub fn volume(&self, mut volume: f64) -> Result<(), Error> {
        volume = volume.clamp(0., 100.);
        self.mpv
            .clone()
            .unwrap()
            .set_volume(volume, mpvipc::NumberChangeOptions::Absolute)
            .map_err(|e| Error::MpvError(e))?;
        Ok(())
    }
    pub fn get_volume(&self) -> Result<(), Error> {
        let volume: f64 = self
            .mpv
            .clone()
            .unwrap()
            .get_property("volume")
            .map_err(|e| Error::MpvError(e))?;
        println!("{volume}");
        Ok(())
    }

    pub fn downland(&self, opt_url: Option<&str>) -> Result<(), Error> {
        let mut url = self
            .mpv
            .clone()
            .unwrap()
            .get_property_string("filename")
            .map_err(|e| Error::MpvError(e))?;

        match opt_url {
            None => {}
            Some(u) => url = u.into(),
        }
        let yt_dlp_args = match option_env!("UTA_DOWNLAND") {
            None => "",
            Some(a) => a,
        };

        let mut args = yt_dlp_args.split(" ").collect::<Vec<_>>();
        let full_url = format!("youtube.com/{url}");
        args.push(&full_url);
        println!("{full_url}");
        println!("{:?}", args);
        let cmd = Command::new("yt-dlp")
            .args(args)
            .arg(url)
            .spawn()
            .map_err(|e| Error::IoErr(e))?;

        info!("start cmd {:?}", cmd);
        Ok(())
    }

    pub fn chose_from_list(&self) -> Result<(), Error> {
        let list_file = match option_env!("UTA_LIST_FILE") {
            None => "",
            Some(l) => l,
        };
        if list_file == "" {
            return Ok(());
        }

        let file = String::from_utf8(fs::read(list_file).map_err(|e| Error::IoErr(e))?)
            .map_err(|e| Error::UtfErr(e))?;
        let lines = file.split("\n").collect::<Vec<&str>>();
        let entiers = lines
            .iter()
            .map(|f| {
                f.split(" ")
                    .take(2)
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        for (i, v) in entiers.iter().enumerate() {
            println!("[{i}] {}", *v.get(0).unwrap_or(&"".to_string()))
        }

        let choice = || -> Result<usize, std::num::ParseIntError> {
            let mut input_string = String::new();
            stdin()
                .read_line(&mut input_string)
                .ok()
                .expect("Failed to read line");

            input_string.trim().parse::<usize>()
        };

        let final_choice = choice().map_err(|e| Error::IntErr(e))?;

        let url = entiers.get(final_choice).unwrap().get(1).unwrap();

        let p = self.clone();

        p.kill()?;

        let b = url.to_string();
        println!("{b}");

        p.start(Some(b))?;

        Ok(())
    }
}

pub fn is_runing() -> bool {
    let cmd = Command::new("pgrep").arg("mpv").output();
    match cmd {
        Err(_) => false,
        Ok(child) => {
            if child.stdout.is_empty() {
                return false;
            }
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_runing() {
        let res = is_runing();
        assert_eq!(true, res)
    }
    #[test]
    fn test_not_runing() {
        let res = is_runing();
        assert_eq!(false, res)
    }
}
