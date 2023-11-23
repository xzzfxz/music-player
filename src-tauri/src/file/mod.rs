#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub mod deal_file {
    use std::path::PathBuf;

    use crate::{
        song::{get_song_model, SongInfo},
        window::CurrentWindow,
    };

    pub fn write_song_csv(window: &CurrentWindow, paths: Vec<PathBuf>) {
        let mut song_list: Vec<SongInfo> = vec![];
        for song_path in paths.iter() {
            match get_song_model(song_path) {
                Ok(song_item) => song_list.push(song_item),
                Err(info) => println!("这是打开文件的错误信息: {}", info),
            }
        }
        println!("这是最终的结果: {:?}", song_list);
        // 先获取本地文件列表
        // 通知前端更新列表
        window.front_reload_song();
    }
}
