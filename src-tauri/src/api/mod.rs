pub use super::song::SongInfo;

pub mod outer_apis {
    use super::SongInfo;

    #[tauri::command]
    pub fn save_local_song_info(list: Vec<SongInfo>) -> Result<(), String> {
        println!("测试: {:#?}", list);
        // println!("{:#?}", list);
        Ok(())
    }
}
