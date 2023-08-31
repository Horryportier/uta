use std::{fs, io::stdin, process::Command, sync::Mutex};

use mpvipc::{Mpv, SeekOptions};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_partial::SerializePartial;

use crate::{env::get_env_mpv_args, Error};

#[derive(Debug)]
pub struct Player {
    pub data: Data,
    pub mpv: Mutex<Option<Mpv>>,
}

#[derive(Serialize, Deserialize, SerializePartial, Debug)]
pub struct Data {
    pub mpv_args: Vec<String>,
    pub url: Option<String>,
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
        let mpv = Mutex::new(mpvipc::Mpv::connect("/tmp/mpvsocket").ok());

        let data = Data {
            mpv_args: def_args,
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
            .spawn()?;
        info!("start cmd {:?}", cmd);
        Ok(())
    }

    pub fn kill(&self) -> Result<(), Error> {
        let cmd = Command::new("pkill").arg("mpv").output()?;
        info!("kill cmd {:?}", cmd);
        Ok(())
    }
    pub fn next(&self) -> Result<(), Error> {
        self.mpv.lock().unwrap().as_ref().unwrap().next()?;
        Ok(())
    }
    pub fn prev(&self) -> Result<(), Error> {
        self.mpv.lock().unwrap().as_ref().unwrap().prev()?;
        Ok(())
    }

    pub fn seek(&self, time: f64, opt: SeekOptions) -> Result<(), Error> {
        self.mpv.lock().unwrap().as_ref().unwrap().seek(time, opt)?;
        Ok(())
    }

    pub fn toggle(&self) -> Result<(), Error> {
        self.mpv.lock().unwrap().as_ref().unwrap().toggle()?;
        Ok(())
    }

    pub fn is_paused(&self) -> Result<bool, Error> {
        Ok(self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property::<bool>("pause")?)
    }

    pub fn print(&self) -> Result<(), Error> {
        let curr: f64 = self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property("playback-time")
            .unwrap_or(0.)
            .floor();
        let len: f64 = self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property("duration")
            .unwrap_or(100.)
            .floor();
        let procent = ((curr / len) * 100.).floor();
        let text = format!(
            "{procent}/100% | {}",
            self.mpv
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .get_property::<String>("media-title")?
        );
        println!("{}", text);
        Ok(())
    }

    pub fn loop_single(&self) -> Result<(), Error> {
        self.mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .set_loop_file(mpvipc::Switch::Toggle)?;
        Ok(())
    }

    pub fn loop_playlist(&self) -> Result<(), Error> {
        self.mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .set_loop_playlist(mpvipc::Switch::Toggle)?;
        Ok(())
    }

    pub fn rand(&self) -> Result<(), Error> {
        let count: usize = self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property("playlist-count")?;
        let rng = rand::thread_rng().gen_range(1..count);
        self.mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .playlist_play_id(rng)?;
        Ok(())
    }

    pub fn volume(&self, mut volume: f64) -> Result<(), Error> {
        volume = volume.clamp(0., 100.);
        self.mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .set_volume(volume, mpvipc::NumberChangeOptions::Absolute)?;
        Ok(())
    }
    pub fn get_volume(&self) -> Result<(), Error> {
        let volume: f64 = self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property("volume")?;
        println!("{volume}");
        Ok(())
    }

    pub fn downland(&self, opt_url: Option<&str>) -> Result<(), Error> {
        let mut url = self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property_string("filename")?;

        match opt_url {
            None => {}
            Some(u) => url = u.into(),
        }
        // UTA_DOWNLAND args for yt-dlp
        let yt_dlp_args = match option_env!("UTA_DOWNLAND") {
            None => "",
            Some(a) => a,
        };

        let mut args = yt_dlp_args.split(" ").collect::<Vec<_>>();
        let full_url = format!("youtube.com/{url}");
        args.push(&full_url);
        println!("{full_url}");
        println!("{:?}", args);
        let cmd = Command::new("yt-dlp").args(args).arg(url).spawn()?;

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

        let file = String::from_utf8(fs::read(list_file)?)?;

        let lines = file.split("\n").collect::<Vec<&str>>();
        let mut entiers = lines
            .iter()
            .map(|f| {
                f.split(" ")
                    .take(2)
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        entiers.pop();
        print!("choose: \n");
        for (i, v) in entiers.iter().enumerate() {
            println!("[{}] {}", i + 1, *v.get(0).unwrap_or(&"".to_string()))
        }

        let choice = || -> Result<usize, std::num::ParseIntError> {
            let mut input_string = String::new();
            stdin()
                .read_line(&mut input_string)
                .ok()
                .expect("Failed to read line");

            input_string.trim().parse::<usize>()
        };

        let final_choice = choice()?;
            
        let url = entiers.get(final_choice-1).unwrap().get(1).unwrap();

        let p = self;

        p.kill()?;

        let b = url.to_string();
        println!("running: {b}");

        p.start(Some(b))?;

        Ok(())
    }
}
