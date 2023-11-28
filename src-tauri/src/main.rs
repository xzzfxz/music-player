// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod dialog;
mod file;
mod song;
mod window;
use crate::api::outer_apis::{get_local_song_list, open_song_dialog};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_local_song_list,
            open_song_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
