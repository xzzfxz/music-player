pub use super::file::deal_file::write_song_csv;

pub mod dialog {
    use tauri::api::dialog::FileDialogBuilder;

    use crate::file::deal_file::write_song_csv;

    pub fn open_local_music(file_type: String, extensions: Vec<&str>) -> String {
        println!("测试");
        FileDialogBuilder::new()
            .add_filter(file_type, &extensions)
            .pick_files(|file_paths| {
                match file_paths {
                    Some(paths) => {
                        write_song_csv(paths);
                    }
                    _ => {}
                };
            });
        println!("这是测试返回的结果");
        format!("这是测试返回的结果")
    }
}
