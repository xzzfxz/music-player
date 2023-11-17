pub use super::song::SongInfo;

pub mod outer_apis {
    use super::SongInfo;

    #[tauri::command]
    pub fn save_local_song_info(test: Vec<i32>) -> Result<(), String> {
        println!("测试: {:#?}", test);
        // println!("{:#?}", list);
        Ok(())
    }
}
