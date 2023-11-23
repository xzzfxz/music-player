pub use super::file::deal_file::write_song_csv;

pub mod dialog {
    use tauri::api::dialog::FileDialogBuilder;

    use crate::{file::deal_file::write_song_csv, window::CurrentWindow};

    pub fn open_local_music(window: CurrentWindow, file_type: String, extensions: Vec<&str>) {
        FileDialogBuilder::new()
            .add_filter(file_type, &extensions)
            .pick_files(move |file_paths| {
                match file_paths {
                    Some(paths) => {
                        write_song_csv(&window, paths);
                    }
                    _ => {}
                };
            });
    }
}
