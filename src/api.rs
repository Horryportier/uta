use std::{
    env::temp_dir,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

use mpvipc::{Mpv, SeekOptions};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_partial::SerializePartial;

use crate::{Error, env::get_env_mpv_args};

static TMPFILE: &str = "uta_tmp";

#[derive(Debug)]
pub struct Player {
    pub data: Data,
    path: String,
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
            // "--ytdl-format=best",
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

        Ok(Player {
            data,
            path: socket.into(),
            mpv,
        })
    }
    pub fn save(&self) -> Result<(), Error> {
        let tmp_dir = temp_dir();

        let mut file: File = File::create(format!("{}/{}", tmp_dir.to_string_lossy(), TMPFILE))
            .map_err(|e| Error::IoErr(e))?;
        if !Path::new(&self.path).exists() {
            let exit_status = Command::new("touch")
                .arg("uta_tmp")
                .current_dir(&tmp_dir)
                .status()
                .map_err(|e| Error::IoErr(e))?;
            assert!(exit_status.success());
        }
        let json = serde_json::to_string(&self.data).map_err(|e| Error::SerdeErr(e))?;
        file.write(json.as_bytes()).map_err(|e| Error::IoErr(e))?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<(), Error> {
        if !Path::new(&self.path).exists() {
            self.save()?;
        }
        let tmp_dir = temp_dir();

        info!("{}/{}", tmp_dir.to_string_lossy(), TMPFILE);

        let file = fs::read(format!("{}/{}", tmp_dir.to_string_lossy(), TMPFILE))
            .map_err(|e| Error::IoErr(e))?;
        serde_json::from_slice::<Data>(&file).map_err(|e| Error::SerdeErr(e))?;

        Ok(())
    }

    pub fn start(&self) -> Result<(), Error> {
        match &self.data.url {
            Some(url) => {
                let cmd = Command::new("mpv")
                    .args(&self.data.mpv_args)
                    .arg(url)
                    .spawn()
                    .map_err(|e| Error::IoErr(e))?;
                info!("start cmd {:?}", cmd);
            }
            None => {
                return Err(Error::ExecuteErr("url is empty".into()));
            }
        }
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

    pub fn volume(&self, volume: f64) -> Result<(), Error> {
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
