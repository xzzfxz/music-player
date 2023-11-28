use serde::Serialize;
use strum_macros::Display;
use tauri::Window;

#[derive(Display)]
pub enum EventName {
    // 通知前端刷新本地音乐列表
    #[strum(serialize = "reloadLocalSongList")]
    ReloadLocalSongList,
}

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
     * @description: 向前端发送事件
     * @return {*}
     */
    pub fn event_to_front(self: &Self, event_name: String, payload: impl Serialize + Clone) {
        println!("开始向前端发送事件");
        self.window.emit(&event_name, payload).unwrap();
    }
}
