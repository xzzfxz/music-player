pub use super::dialog::dialog;
pub use super::song::SongInfo;

pub mod outer_apis {

    use tauri::Window;

    use crate::window::CurrentWindow;

    use super::dialog;
    use super::SongInfo;

    #[tauri::command]
    pub fn open_song_dialog(window: Window, file_type: String, extensions: Vec<&str>) {
        let cur_window = CurrentWindow::new(window);
        dialog::open_local_music(cur_window, file_type, extensions);
    }

    #[tauri::command]
    pub fn save_local_song_info(list: Vec<SongInfo>) -> Result<(), String> {
        println!("测试: {:#?}", list);
        // let mut rdr = Reader::from_path("../../assets/local_song_list.csv");
        Ok(())
    }
}
