use std::{env, fs, path::Path, process::Command};

pub fn main() {
    init_dir();
    init_ffmpeg();
}

fn init_dir() {
    let dir = Path::new("exec");
    if !dir.exists() {
        fs::create_dir(dir).expect("Exec dir létrehozása sikertelen")
    }
}

fn init_ffmpeg() {
    if env::consts::OS == "windows" {
        let exec = Path::new("exec/ffmpeg.exe");
        if !exec.exists() {
            panic!("ffmpeg.exe nem található")
        }
    } else if Command::new("ffmpeg").spawn().is_err() {
        panic!("ffmpeg nincs telepítve")
    }
}
