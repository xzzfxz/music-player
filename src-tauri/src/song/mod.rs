use serde::{Deserialize, Serialize};

/**
 * @description: 歌曲信息
 * @return {*}
 */
#[derive(Deserialize, Serialize, Debug)]
pub struct SongInfo {
    id: Option<String>,
    singer: String,
    name: String,
    path: String,
    time: Option<String>,
    mv: Option<bool>,
    album: Option<String>,
    online: Option<bool>,
}
