use std::{
    fs::{self},
    io::{stdin, stdout},
    process::Command,
    sync::Mutex,
};

use crossterm::{
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use mpvipc::{ipc::PlaylistEntry, Mpv, SeekOptions};
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

    // https://i.ytimg.com/vi/t0zooRIpAe4/hqdefault.jpg
    pub fn safe_thumbnail(&self, safe_path: String) -> Result<(), Error> {
        let url = self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property_string("path")
            .map_err(|e| eprintln!("{}", e)).unwrap();
        let id = url.split("watch?v=").last();
        let final_url = format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", id.unwrap_or("dQw4w9WgXcQ"));

        let img_bytes  = reqwest::blocking::get(final_url.clone())?.bytes()?;
        let img = image::load_from_memory(&img_bytes)?;
        let img_type = image::guess_format(&img_bytes)?;
        println!("{:?}", img_type);
        img.save_with_format(safe_path, img_type)?;
        Ok(())
    }
    
    pub fn get_link(&self) -> Result<(), Error> {
        let url = self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property_string("path")?;

        println!("{url}");
        Ok(())
    }

    pub fn print(&self) -> Result<(), Error> {
        let name = match self.mpv.lock().unwrap().as_ref() {
            Some(m) => match m.get_property::<String>("media-title") {
                Ok(f) => f,
                Err(..) => "none".into(),
            },
            None => "none".into(),
        };
        let text = format!("{name}");
        println!("{}", text);
        Ok(())
    }

    pub fn get_procentage(&self) -> Result<f64, Error> {
        let curr = match self.mpv.lock().unwrap().as_ref() {
            Some(m) => match m.get_property::<f64>("playback-time") {
                Ok(f) => f,
                Err(..) => 100.,
            },
            None => 100.,
        }
        .floor();

        let len = match self.mpv.lock().unwrap().as_ref() {
            Some(m) => match m.get_property::<f64>("duration") {
                Ok(f) => f,
                Err(..) => 100.,
            },
            None => 100.,
        }
        .floor();

        Ok(((curr / len) * 100.).floor())
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

    pub fn print_volume(&self) -> Result<(), Error> {
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

    #[allow(dead_code)]
    pub fn get_playlist_info(&self) -> Result<Vec<PlaylistEntry>, Error> {
        let playlist = self
            .mpv
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_property::<Vec<PlaylistEntry>>("playlist")?;

        Ok(playlist)
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

        stdout().execute(EnterAlternateScreen)?;
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

        let url = entiers.get(final_choice - 1).unwrap().get(1).unwrap();

        let p = self;

        p.kill()?;

        let b = url.to_string();
        println!("running: {b}");
        stdout().execute(LeaveAlternateScreen)?;

        p.start(Some(b))?;

        Ok(())
    }
}
