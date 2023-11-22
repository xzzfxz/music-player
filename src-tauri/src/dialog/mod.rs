use std::path::PathBuf;

/**
 * @description: 获取歌名，歌手和后缀名
 * @param {PathBuf} song_path
 * @param {*} String
 * @param {*} String
 * @return {*}
 */
fn get_song_info(song_path: PathBuf) -> Result<(String, String, String), Option<String>> {
    if !song_path.exists() {
        println!("文件不存在: {:?}", song_path);
        return Err(None);
    }
    // 后缀名
    let ext = song_path.extension().unwrap().to_string_lossy().to_string();
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
    println!("歌名名为：{:?}, 歌手为：{}", name, singer);
    Ok((singer, name, ext))
}

pub mod dialog {
    use tauri::api::dialog::FileDialogBuilder;

    use crate::dialog::get_song_info;

    pub fn open_local_music(file_type: String, extensions: Vec<&str>) -> String {
        println!("测试");
        FileDialogBuilder::new()
            .add_filter(file_type, &extensions)
            .pick_files(|file_paths| {
                match file_paths {
                    Some(paths) => {
                        for path in paths.into_iter() {
                            get_song_info(path);
                        }
                    }
                    _ => {}
                };
            });
        println!("这是测试返回的结果");
        format!("这是测试返回的结果")
    }
}
