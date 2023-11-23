use tauri::Window;

/**
 * @description: 用来保存当前窗口实例
 * @return {*}
 */
pub struct CurrentWindow {
    window: Window,
}

impl CurrentWindow {
    pub fn new(window: Window) -> Self {
        return CurrentWindow { window };
    }

    /**
     * @description: 通知前端刷新本地音乐列表
     * @return {*}
     */
    pub fn front_reload_song(self: &Self) {
        println!("开始向前端发送事件");
        self.window.emit("reloadLocalSongList", {}).unwrap();
    }
}
