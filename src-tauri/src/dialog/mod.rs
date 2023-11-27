pub use super::file::deal_file::write_song_csv;

pub mod dialog {
    use tauri::api::dialog::FileDialogBuilder;

    use crate::{file::deal_file::write_song_csv, window::CurrentWindow};

    pub fn open_local_music(cur_window: CurrentWindow, file_type: String, extensions: Vec<&str>) {
        FileDialogBuilder::new()
            .add_filter(file_type, &extensions)
            .pick_files(move |file_paths| {
                match file_paths {
                    Some(paths) => {
                        let res = write_song_csv(&cur_window, paths);
                        println!("写入文件报错: {:?}", res);
                    }
                    _ => {}
                };
            });
    }
}
