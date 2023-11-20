pub mod dialog {
    use tauri::api::dialog::FileDialogBuilder;

    pub fn open_local_music(file_type: String, extensions: Vec<&str>) -> String {
        println!("测试");
        FileDialogBuilder::new()
            .add_filter(file_type, &extensions)
            .pick_files(|file_paths| {
                match file_paths {
                    Some(paths) => {
                        println!("文件路径为：{:?}", paths);
                    }
                    _ => {}
                };
            });
        format!("这是测试返回的结果")
    }
}
