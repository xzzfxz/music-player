use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Deserialize, Serialize, Debug)]
struct DataSong {
    title: String,
    author: String,
    link: String,
    lrc: String,
    pic: String,
    songid: u32,
    url: String,
    r#type: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ResultSong {
    code: u32,
    error: String,
    data: Vec<DataSong>,
}

#[derive(Display)]
enum SearchType {
    #[strum(serialize = "netease")]
    WangYi,
    #[strum(serialize = "qq")]
    QQ,
    #[strum(serialize = "kugou")]
    KuGou,
    #[strum(serialize = "kuwo")]
    KuWo,
    #[strum(serialize = "baidu")]
    BaiDu,
}

pub async fn search_song(keyword: String) -> Result<()> {
    println!("搜索的关键词: {}", keyword);
    let client = reqwest::Client::new();
    let mut form_params = HashMap::new();
    form_params.insert("input", keyword);
    form_params.insert("filter", "name".to_string());
    form_params.insert("type", SearchType::WangYi.to_string());
    form_params.insert("page", "1".to_string());
    let res = client
        .post("https://music.liuzhijin.cn")
        .header("X-Requested-With", "XMLHttpRequest")
        .form(&form_params)
        .send()
        .await?;
    if res.status().is_success() {
        let body = res.json::<ResultSong>().await?;
        println!("请求的结果: {:#?}", body);
    }
    Ok(())
}
