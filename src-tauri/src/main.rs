// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod song;
use crate::api::outer_apis::{open_song_dialog, save_local_song_info};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_local_song_info,
            open_song_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
