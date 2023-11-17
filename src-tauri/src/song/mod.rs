use serde::{Deserialize, Serialize};

/**
 * @description: 歌曲信息
 * @return {*}
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct SongInfo {
    id: String,
    singer: String,
    name: String,
    path: String,
    time: String,
    mv: bool,
    album: String,
}
