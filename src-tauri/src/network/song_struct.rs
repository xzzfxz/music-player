use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Deserialize, Serialize, Debug)]
pub struct DataSong {
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
pub struct ResultSong {
    code: u32,
    error: String,
    data: Vec<DataSong>,
}

#[derive(Display)]
pub enum SearchType {
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
