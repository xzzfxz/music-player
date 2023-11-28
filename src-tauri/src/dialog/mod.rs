pub use super::file::deal_file::write_song_csv;

pub mod dialog {
    use tauri::api::dialog::FileDialogBuilder;

    use crate::{file::deal_file::write_song_csv, window::CurrentWindow};

    /**
     * @description: 打开选取本地歌曲文件对话框
     * @param {CurrentWindow} cur_window
     * @param {String} file_type 文件类型名称
     * @param {Vec} extensions 文件后缀列表
     * @return {*}
     */
    pub fn open_local_music(cur_window: CurrentWindow, file_type: String, extensions: Vec<&str>) {
        FileDialogBuilder::new()
            .add_filter(file_type, &extensions)
            .pick_files(move |file_paths| {
                match file_paths {
                    Some(paths) => match write_song_csv(&cur_window, paths) {
                        Err(err_info) => {
                            println!("写入文件发生错误: {:?}", err_info);
                        }
                        _ => {}
                    },
                    _ => {}
                };
            });
    }
}
