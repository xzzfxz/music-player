use claxon::FlacReader;
use hound::WavReader;
use mp3_duration;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/**
 * @description: 歌曲信息
 * @return {*}
 */
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SongInfo {
    pub id: Uuid,
    pub singer: String,
    pub name: String,
    pub ext: String,
    pub path: String,
    pub time: String,
    pub duration: u32,
    pub mv: bool,
    pub album: bool,
    pub online: bool,
}

/**
 * @description: 格式化显示总时长为：HH:mm:ss
 * @param {u64} time
 * @return {*}
 */
fn format_song_time(time: u32) -> String {
    let mut last_time = String::new();
    let hour = time / 60 / 60;
    if hour > 0 {
        last_time += &format!("{:02}:", hour);
    }
    let min = (time - hour * 60 * 60) / 60;
    last_time += &format!("{:02}:", min);
    let sec: u32 = (time - hour * 60 * 60 - min * 60) % 60;
    last_time += &format!("{:02}", sec);
    last_time
}

/**
 * @description: 获取音频的总时长
 * @param {String} ext
 * @param {*} song_path
 * @return {*}
 */
fn get_song_duration(ext: &String, song_path: &PathBuf) -> (String, u32) {
    if ["mp3"].contains(&ext.as_str()) {
        let duration = mp3_duration::from_path(song_path).unwrap().as_secs() as u32;
        return (format_song_time(duration), duration);
    }
    if ["flac"].contains(&ext.as_str()) {
        let reader = FlacReader::open(song_path).unwrap();
        let stream_info = reader.streaminfo();
        let duration = stream_info.samples.unwrap() as u32 / stream_info.sample_rate;
        return (format_song_time(duration), duration);
    }
    if ["wav"].contains(&ext.as_str()) {
        let reader = WavReader::open(song_path).unwrap();
        let duration = reader.duration() / reader.spec().sample_rate;
        return (format_song_time(duration), duration);
    }
    ("".to_string(), 0)
}

/**
 * @description: 获取歌名，歌手和后缀名
 * @param {PathBuf} song_path
 * @param {*} String
 * @param {*} String
 * @return {*}
 */
fn get_song_info(song_path: &PathBuf) -> Result<(String, String, String), String> {
    if !song_path.exists() {
        println!("文件不存在: {:?}", song_path);
        return Err(format!("文件不存在: {:?}", song_path));
    }
    // 后缀名
    let ext = song_path.extension().unwrap().to_string_lossy().into();
    let song_singer = song_path.file_stem();
    // 歌名和歌手
    let (name, singer) = match song_singer {
        None => ("".to_string(), "".to_string()),
        Some(info) => {
            let mut info_arr: Vec<String> = info
                .to_string_lossy()
                .split("-")
                .map(|item| item.trim().to_string())
                .collect();
            let mut name = String::from("");
            if let Some(x) = info_arr.pop() {
                name = x;
            }
            let singer = info_arr.join("-");
            (name, singer)
        }
    };
    Ok((singer, name, ext))
}

/**
 * @description: 获取歌曲信息模型
 * @param {*} song_path
 * @return {*}
 */
pub fn get_song_model(song_path: &PathBuf) -> Result<SongInfo, String> {
    // 先获取歌手，歌名，后缀
    let (singer, name, ext);
    match get_song_info(song_path) {
        Ok(res) => (singer, name, ext) = res,
        Err(info) => return Err(info),
    }
    // 再获取时长
    let (time, duration) = get_song_duration(&ext, song_path);
    Ok(SongInfo {
        id: Uuid::new_v4(),
        singer,
        ext,
        name,
        path: String::from(song_path.to_string_lossy()),
        time,
        duration,
        mv: false,
        album: false,
        online: false,
    })
}
