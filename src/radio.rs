use crate::song::Song;
use crate::song_queue::SongQueue;
use actix::*;
use std::path::Path;
use std::process::Command;
use std::{thread, time};

#[derive(Default)]
pub struct Radio {
    script_path: String,
}

impl Radio {
    pub fn new() -> Self {
        // panic if script isn't avialable
        let script_path = get_script_path().unwrap();
        Radio { script_path }
    }

    fn play_song(&self, song_path: &str, song_duration: i32) {
        // println!("sleeping");
        // let timeout = time::Duration::from_secs(30);
        // thread::sleep(timeout);
        Command::new("timeout")
            .arg(song_duration.to_string())
            .arg("./play_song.sh")
            .arg(song_path)
            .arg(self.script_path.clone())
            .output();
    }
}

pub struct PlaySong {
    pub song: Song,
    pub queue_addr: Addr<SongQueue>,
}

pub struct NextSong;
impl Message for NextSong {
    type Result = ();
}

impl Message for PlaySong {
    type Result = ();
}

impl Actor for Radio {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {}
}

impl Handler<PlaySong> for Radio {
    type Result = ();
    fn handle(&mut self, msg: PlaySong, ctx: &mut Self::Context) -> Self::Result {
        // let timeout = time::Duration::from_secs(msg.song.duration as u64);
        self.play_song(&msg.song.path, msg.song.duration);
        msg.queue_addr.do_send(NextSong {});
    }
}

pub fn get_script_path() -> Result<String, ()> {
    let path = Path::new("../fm_transmitter/fm_transmitter");
    let script_exists = path.exists();
    if script_exists {
        Ok(std::fs::canonicalize(&path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned())
    } else {
        Err(())
    }
}
