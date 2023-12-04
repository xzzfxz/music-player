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
     * @description: 将本地文件转为音频流
     * @param {String} file_path 文件路径
     * @return {*}
     */
    #[tauri::command]
    pub async fn read_local_song(file_path: String) -> Vec<u8> {
        match file::deal_file::read_local_song(file_path).await {
            Ok(res) => res,
            Err(info) => {
                println!("读取音频流发生错误: {}", info);
                vec![]
            }
        }
    }
}
