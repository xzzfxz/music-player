// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod dialog;
mod file;
mod network;
mod song;
mod window;
use crate::api::outer_apis::{
    delete_local_song, get_local_song_list, open_folder, open_song_dialog, search_songs,
    search_tips,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_local_song_list,
            open_song_dialog,
            delete_local_song,
            open_folder,
            search_tips,
            search_songs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
