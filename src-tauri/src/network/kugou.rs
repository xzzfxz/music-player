use reqwest::redirect::Policy;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

pub async fn search_songs(keyword: String) -> Result<String> {
    let url = format!(
        "http://songsearch.kugou.com/song_search_v2?callback=&keyword={}&page=1&pagesize=30&userid=-1&platform=WebFilter&tag=em",
        keyword
    );
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    if res.status().is_success() {
        let body = res.text().await?;
        Ok(body)
    } else {
        Ok("".to_string())
    }
}

/**
 * @description: 获取音乐播放地址与歌词
 * @param {String} hash 歌曲hash
 * @param {String} album_id 专辑id
 * @return {*}
 */
pub async fn get_song_info(hash: String, album_id: String) -> Result<String> {
    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let url = format!(
        "https://www.kugou.com/yy/index.php?r=play/getdata&hash={}&album_id={}&_={}",
        hash, album_id, time
    );
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::COOKIE, "kg_mid=2333")
        .send()
        .await?;
    if res.status().is_success() {
        let body = res.text().await?;
        Ok(body)
    } else {
        Ok("".to_string())
    }
}
