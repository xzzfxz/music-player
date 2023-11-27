#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub mod deal_file {
    use std::{
        error::Error,
        fs::{self, OpenOptions},
        path::{Path, PathBuf},
    };

    use tauri::api::path;

    use csv::{DeserializeRecordsIntoIter, Writer, WriterBuilder};

    use crate::{
        song::{self, get_song_model, SongInfo},
        window::CurrentWindow,
    };

    const APP_PATH: &str = "MusicPlayer";

    pub fn get_data_path() -> Option<PathBuf> {
        if let Some(mut x) = path::data_dir() {
            x.push(APP_PATH);
            Some(x)
        } else {
            None
        }
    }

    pub fn write_song_csv(
        cur_window: &CurrentWindow,
        paths: Vec<PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        let mut song_list: Vec<SongInfo> = vec![];
        for song_path in paths.iter() {
            match get_song_model(song_path) {
                Ok(song_item) => song_list.push(song_item),
                Err(info) => println!("这是打开文件的错误信息: {}", info),
            }
        }
        println!("这是最终的结果: {:?}", song_list);
        // 先判断本地文件是否存在
        let mut file_path = get_data_path().unwrap();
        file_path.push("assets/local_song_list.csv");
        let file_path: &Path = Path::new(&file_path);
        let mut wtr;
        let mut last_list = song_list;
        if file_path.exists() {
            // 文件存在，先读取已存在的，再根据path去重，最后追加
            let rdr = csv::Reader::from_path(file_path).unwrap();
            let iter: DeserializeRecordsIntoIter<fs::File, SongInfo> = rdr.into_deserialize();
            let pre_lis: Vec<SongInfo> = iter.map(|song| song.unwrap()).collect();

            last_list = last_list
                .into_iter()
                .filter(|song| {
                    return !pre_lis.iter().any(|pre| pre.path == song.path);
                })
                .collect();

            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_path)?;
            wtr = WriterBuilder::new().has_headers(false).from_writer(file);
        } else {
            // 文件不存在，生成文件并直接把列表写入
            fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            wtr = Writer::from_path(file_path)?;
        }
        for song in last_list.iter() {
            wtr.serialize(song)?;
        }
        println!("写入磁盘");
        wtr.flush()?;
        // 通知前端更新列表
        cur_window.front_reload_song();
        Ok(())
    }
}
