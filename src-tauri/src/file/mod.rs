#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub mod deal_file {
    use std::{
        error::Error,
        fs::File,
        path::{Path, PathBuf},
    };

    use csv::Writer;

    use crate::{
        file,
        song::{get_song_model, SongInfo},
        window::CurrentWindow,
    };

    pub fn write_song_csv(
        window: &CurrentWindow,
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
        let file_path = Path::new("assets/local_song_list.csv");
        if file_path.exists() {
            // 文件存在
        } else {
            // 文件不存在，生成文件并直接把列表写入
            File::create(file_path)?;
            println!("开始写入文件");
            let mut wtr = Writer::from_path(file_path)?;
            for song in song_list.iter() {
                wtr.serialize(song)?;
            }
            println!("写入磁盘");
            wtr.flush()?;
        }
        // 先获取本地文件列表
        let rdr = csv::Reader::from_path("").unwrap();
        println!("读取到的文件为: {:?}", rdr);
        // 通知前端更新列表
        // window.front_reload_song();
        Ok(())
    }
}
