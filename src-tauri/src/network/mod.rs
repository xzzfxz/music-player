mod kugo;
mod song_struct;

use anyhow::{Ok, Result};
use std::collections::HashMap;

// pub async fn search_song(keyword: String) -> Result<()> {
//     println!("搜索的关键词: {}", keyword);
//     let client = reqwest::Client::new();
//     let mut form_params = HashMap::new();
//     form_params.insert("input", keyword);
//     form_params.insert("filter", "name".to_string());
//     form_params.insert("type", SearchType::WangYi.to_string());
//     form_params.insert("page", "1".to_string());
//     let res = client
//         .post("https://music.liuzhijin.cn")
//         .header("X-Requested-With", "XMLHttpRequest")
//         .form(&form_params)
//         .send()
//         .await?;
//     if res.status().is_success() {
//         let body = res.json::<ResultSong>().await?;
//         println!("请求的结果: {:#?}", body);
//     }
//     Ok(())
// }

/**
 * @description: 酷狗搜索提示
 * @param {String} keyword 关键词
 * @return {*}
 */
pub async fn search_tips(keyword: String) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!(
        "http://searchtip.kugou.com/getSearchTip?keyword={}",
        keyword
    );
    let res = client.get(url).send().await?;
    if res.status().is_success() {
        let body = res.text().await?;
        println!("请求的结果: {:#?}", body);
        Ok(body)
    } else {
        Ok("".to_string())
    }
}
