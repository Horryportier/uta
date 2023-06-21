
use std::{fs::{File, self}, io::{self, Write}, path::Path, process::{Command, Output, Child}};

use serde::{Serialize, Deserialize};
use mpvipc::{Mpv, SeekOptions};
use serde_partial::SerializePartial;


static  TMPATH: &str= "~/tmp/uta";

pub struct  Player {
  data: Data,
  path: String,
}

#[derive(Serialize, Deserialize  ,SerializePartial, Debug)]
pub struct  Data {
  pub is_runing: bool,
  pub log: bool,
  pub mpv_args: Vec<String>,

  pub previous_video: (),
  pub time: usize,
}
    

impl Player {
    /// path or youtube link
   pub fn new(url: &str) -> Player {
       let def_args:Vec<String> = [
       "--ytdl-format=best",
       "--no-video",  "--volume=50",
       "--input-ipc-server=/tmp/mpvsocket", url].map(String::from).to_vec();
       let data = Data { is_runing: false, log: false, 
           mpv_args: def_args ,previous_video: (), time: 0 };
       Player { data , path: TMPATH.to_string()}
   } 
   pub fn save(&self) -> io::Result<()>{
       let mut file: File = File::create(TMPATH)?; 
       if !Path::new(&self.path).exists() {
           file = File::create(TMPATH)?;
       }
       let json = serde_json::to_string(&self.data)?;
       file.write(json.as_bytes())?;

       Ok(())
   }

   pub fn load(&mut self) -> io::Result<()> {
       if !Path::new(&self.path).exists() {
           self.save()?
       }

       let file =  fs::read(TMPATH)?;
       serde_json::from_slice::<Data>(&file)?;

       Ok(())
   }

   pub fn start(&self) -> io::Result<Child> {
      let cmd = Command::new("mpv").args(&self.data.mpv_args).spawn()?;
      println!("{:?}", cmd);
       
       Ok(cmd)
   }
   
   pub fn kill(&self) -> Result<(), mpvipc::Error>{
       Mpv::connect(&self.path)?.kill()?;
       Ok(())
   }
   pub fn next(&self) -> Result<(), mpvipc::Error>{
       Mpv::connect(&self.path)?.next()?;
       Ok(())
   }
   pub fn prev(&self) -> Result<(), mpvipc::Error>{
       Mpv::connect(&self.path)?.prev()?;
       Ok(())
   }

   pub fn seek(&self,time: f64, opt: SeekOptions) -> Result<(), mpvipc::Error> {
       Mpv::connect(&self.path)?.seek(time, opt)?;
       Ok(())
   }
}
