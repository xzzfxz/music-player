pub use super::song::SongInfo;

pub mod outer_apis {

    use super::SongInfo;
    use csv::Reader;
    use tauri::api::dialog::FileDialogBuilder;

    #[tauri::command]
    pub fn open_song_dialog(file_type: String, extensions: Vec<&str>) -> String {
        println!("打开文件对话框 {}, {:?}", file_type, extensions);
        FileDialogBuilder::new()
            .add_filter(file_type, &extensions)
            .pick_files(|file_paths| {
                let res = match file_paths {
                    None => "none",
                    Some(paths) => {
                        println!("文件路径为：{:?}", paths);
                        "test"
                    }
                };
                println!("最终的res: {}", res);
            });
        "test".into()
    }

    #[tauri::command]
    pub fn save_local_song_info(list: Vec<SongInfo>) -> Result<(), String> {
        println!("测试: {:#?}", list);
        // let mut rdr = Reader::from_path("../../assets/local_song_list.csv");
        Ok(())
    }
}
