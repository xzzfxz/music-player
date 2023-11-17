pub mod dialog {
    #[tauri::command]
    pub async fn open_local_music() -> String {
        println!("测试");
        // let seleted =
        format!("这是测试返回的结果")
    }
}
