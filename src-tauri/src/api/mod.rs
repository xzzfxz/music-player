pub use super::dialog::dialog;
pub use super::song::SongInfo;

pub mod outer_apis {

    use crate::file;
    use crate::window::CurrentWindow;
    use tauri::Window;

    use super::dialog;
    use super::SongInfo;

    /**
     * @description: 打开文件选择对话框
     * @param {Window} window
     * @param {String} file_type 文件类型
     * @param {Vec} extensions 文件后缀
     * @return {*}
     */
    #[tauri::command]
    pub fn open_song_dialog(window: Window, file_type: String, extensions: Vec<&str>) {
        let cur_window = CurrentWindow::new(window);
        dialog::open_local_music(cur_window, file_type, extensions);
    }

    /**
     * @description: 读取本地音频列表
     * @return {*}
     */
    #[tauri::command]
    pub fn get_local_song_list() -> Vec<SongInfo> {
        match file::deal_file::read_song_csv() {
            Ok(list) => list,
            Err(info) => {
                println!("发生错误: {}", info);
                vec![]
            }
        }
    }

    /**
     * @description: 从csv中删除歌曲，并删除对应文件
     * @param {String} song_path 歌曲路径
     * @param {bool} delete_file 是否删除文件
     * @return {*}
     */
    #[tauri::command]
    pub async fn delete_local_song(song_path: String, delete_file: bool) -> String {
        match file::deal_file::delete_local_song(song_path, delete_file).await {
            Ok(_) => "ok".to_string(),
            Err(info) => {
                println!("删除歌曲发生错误: {}", info);
                format!("{:?}", info)
            }
        }
    }

    /**
     * @description: 打开文件夹
     * @param {String} file_path 文件路径
     * @return {*}
     */
    #[tauri::command]
    pub async fn open_folder(file_path: String) -> bool {
        file::deal_file::open_folder(file_path);
        true
    }
}
