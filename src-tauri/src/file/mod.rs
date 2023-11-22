pub mod deal_file {
    use std::path::PathBuf;

    use crate::song::get_song_model;

    pub fn write_song_csv(paths: Vec<PathBuf>) {
        for song_path in paths.iter() {
            get_song_model(song_path);
        }
    }
}
