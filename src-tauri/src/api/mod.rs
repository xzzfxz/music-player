pub use super::dialog::dialog;
pub use super::song::SongInfo;

pub mod outer_apis {

    use crate::file;
    use crate::network;
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

    /**
     * @description: 搜索提示
     * @param {String} keyword 关键词
     * @return {*}
     */
    #[tauri::command]
    pub async fn search_tips(keyword: String) -> String {
        match network::search_tips(keyword).await {
            Ok(body) => body,
            Err(info) => {
                println!("搜索提示请求错误: {:?}", info);
                "".to_string()
            }
        }
    }

    /**
     * @description: 在线搜索歌曲
     * @param {String} keyword 关键词
     * @param {String} channel 渠道
     * @return {*}
     */
    #[tauri::command]
    pub async fn search_songs(
        keyword: String,
        channel: network::song_struct::ChannelType,
    ) -> String {
        match network::search_songs(keyword, channel).await {
            Ok(res) => res,
            Err(info) => {
                println!("搜索提示请求错误: {:?}", info);
                "".to_string()
            }
        }
    }

    /**
     * @description: 获取歌曲信息
     * @param {String} hash 文件hash
     * @param {String} album_id 专辑id
     * @param {network} channel 渠道
     * @return {*}
     */
    #[tauri::command]
    pub async fn get_song_info(
        hash: String,
        album_id: String,
        channel: network::song_struct::ChannelType,
    ) -> String {
        match network::get_song_info(hash, album_id, channel).await {
            Ok(res) => res,
            Err(info) => {
                println!("搜索提示请求错误: {:?}", info);
                "".to_string()
            }
        }
    }

    /**
     * @description: 获取mv分类
     * @param {network} channel 渠道
     * @return {*}
     */
    #[tauri::command]
    pub async fn get_mv_category(channel: network::song_struct::ChannelType) -> String {
        match network::get_mv_category(channel).await {
            Ok(res) => res,
            Err(info) => {
                println!("搜索mv分类请求错误: {:?}", info);
                "".to_string()
            }
        }
    }

    /**
     * @description: 获取mv列表
     * @param {u16} short mv分类short
     * @param {u16} sort mv分类sort
     * @param {u16} id mv分类id
     * @param {u16} page 页数
     * @param {u16} size 每页条数
     * @param {ChannelType} channel 渠道
     * @return {*}
     */
    #[tauri::command]
    pub async fn get_mv_list(
        short: u16,
        sort: u16,
        id: u16,
        page: Option<u16>,
        size: Option<u16>,
        channel: network::song_struct::ChannelType,
    ) -> String {
        match network::get_mv_list(short, sort, id, page, size, channel).await {
            Ok(res) => res,
            Err(info) => {
                println!("搜索mv列表请求错误: {:?}", info);
                "".to_string()
            }
        }
    }
}
