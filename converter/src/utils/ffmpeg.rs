use std::env::consts::OS;

pub fn get_ffmpeg() -> String {
    if OS == "windows" {
        "exec/ffmpeg.exe".to_string()
    } else {
        "ffmpeg".to_string()
    }
}
