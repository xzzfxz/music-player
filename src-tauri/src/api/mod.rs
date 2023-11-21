pub use super::dialog::dialog;
pub use super::song::SongInfo;

pub mod outer_apis {

    use super::dialog;
    use super::SongInfo;

    #[tauri::command]
    pub fn open_song_dialog(file_type: String, extensions: Vec<&str>) {
        println!("打开文件对话框 {}, {:?}", file_type, extensions);
        dialog::open_local_music(file_type, extensions);
    }

    #[tauri::command]
    pub fn save_local_song_info(list: Vec<SongInfo>) -> Result<(), String> {
        println!("测试: {:#?}", list);
        // let mut rdr = Reader::from_path("../../assets/local_song_list.csv");
        Ok(())
    }
}
