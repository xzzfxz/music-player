pub mod file_error;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub mod deal_file {
    use super::file_error::FileError;
    use crate::{
        song::{get_song_model, SongInfo},
        window::{CurrentWindow, EventName},
    };
    use anyhow::{anyhow, Context, Result};
    use csv::{DeserializeRecordsIntoIter, Writer, WriterBuilder};
    use std::{
        fs::{self, OpenOptions},
        path::{Path, PathBuf},
        process::Command,
    };
    use tauri::api::path;

    const APP_PATH: &str = "MusicPlayer";

    /**
     * @description: 获取保存数据的根路径
     * @return {*}
     */
    pub fn get_data_path() -> Result<PathBuf> {
        path::data_dir()
            .map(|mut x| {
                x.push(APP_PATH);
                x
            })
            .ok_or(anyhow!(FileError::PathError))
    }

    /**
     * @description: 获取csv文件路径
     * @return {*}
     */
    fn get_song_csv_path() -> Result<PathBuf> {
        let mut file_path: PathBuf = get_data_path()?;
        file_path.push("assets/local_song_list.csv");
        Ok(file_path)
    }

    /**
     * @description: 读取本地歌曲csv文件
     * @return {*}
     */
    pub fn read_song_csv() -> Result<Vec<SongInfo>> {
        let file_path = get_song_csv_path()?;
        let file_path: &Path = Path::new(&file_path);
        let rdr = csv::Reader::from_path(file_path).context(FileError::FileReadFail)?;
        let iter: DeserializeRecordsIntoIter<fs::File, SongInfo> = rdr.into_deserialize();
        let list: Vec<SongInfo> = iter.map(|song| song.unwrap()).collect();
        Ok(list)
    }

    /**
     * @description: 写入歌曲列表文件
     * @param {*} cur_window 当前窗口实例
     * @param {Vec} paths 获取到的歌曲路径
     * @param {*} Result 写入结果
     * @return {*}
     */
    pub fn write_song_csv(cur_window: &CurrentWindow, paths: Vec<PathBuf>) -> Result<()> {
        let mut song_list: Vec<SongInfo> = vec![];
        for song_path in paths.iter() {
            let song_item = get_song_model(song_path)?;
            song_list.push(song_item)
        }
        // 先读取本地文件
        let file_path = get_song_csv_path()?;
        let file_path: &Path = Path::new(&file_path);

        let mut wtr;
        let mut last_list = song_list;

        if let Ok(pre_list) = read_song_csv() {
            last_list = last_list
                .into_iter()
                .filter(|song| {
                    return !pre_list.iter().any(|pre| pre.path == song.path);
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
        wtr.flush()?;
        // 通知前端更新列表
        cur_window.event_to_front(EventName::ReloadLocalSongList.to_string(), {});
        Ok(())
    }

    /**
     * @description: 从csv中删除歌曲，并删除对应文件
     * @param {String} song_path 歌曲路径
     * @param {bool} delete_file 是否删除文件
     * @return {*}
     */
    pub async fn delete_local_song(song_path: String, delete_file: bool) -> Result<bool> {
        let list = read_song_csv()?;
        let last_list: Vec<SongInfo> = list
            .into_iter()
            .filter(|song| song.path != song_path)
            .collect();
        let file_path = get_song_csv_path()?;
        let mut wtr = Writer::from_path(file_path)?;
        for song in last_list.iter() {
            wtr.serialize(song)?;
        }
        wtr.flush()?;
        if delete_file {
            // 删除本地文件
            let _ = tokio::fs::remove_file(song_path).await?;
            return Ok(true);
        }
        Ok(true)
    }

    /**
     * @description: 打开文件夹
     * @param {String} file_path 文件路径
     * @return {*}
     */
    pub fn open_folder(file_path: String) {
        let parent = Path::new(&file_path).parent().unwrap();
        let status = Command::new("open").arg(parent).status();
        match status {
            Ok(status) => println!("状态: {}", status),
            Err(info) => println!("发生错误: {}", info),
        }
    }
}
