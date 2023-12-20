mod kugou;
pub mod song_struct;

use anyhow::{Ok, Result};

use crate::network::song_struct::ChannelType;

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
        Ok(body)
    } else {
        Ok("".to_string())
    }
}

/**
 * @description: 在线搜索歌曲
 * @param {String} keyword 关键词
 * @param {network} channel 渠道
 * @return {*}
 */
pub async fn search_songs(keyword: String, channel: ChannelType) -> Result<String> {
    let res = match channel {
        ChannelType::KuGou => kugou::search_songs(keyword).await?,
        _ => "".to_string(),
    };
    Ok(res)
}

/**
 * @description: 获取歌曲信息
 * @param {String} hash 文件hash
 * @param {String} album_id 专辑id
 * @param {network} channel 渠道
 * @return {*}
 */
pub async fn get_song_info(hash: String, album_id: String, channel: ChannelType) -> Result<String> {
    let res = match channel {
        ChannelType::KuGou => kugou::get_song_info(hash, album_id).await?,
        _ => "".to_string(),
    };
    Ok(res)
}

/**
 * @description: 获取mv分类
 * @param {ChannelType} channel 渠道
 * @return {*}
 */
pub async fn get_mv_category(channel: ChannelType) -> Result<String> {
    let res = match channel {
        ChannelType::KuGou => kugou::get_mv_category().await?,
        _ => "".to_string(),
    };
    Ok(res)
}

/**
 * @description: 获取mv列表
 * @param {u16} short mv分类short
 * @param {u16} page 页数
 * @param {u16} size 每页条数
 * @param {ChannelType} channel 渠道
 * @return {*}
 */
pub async fn get_mv_list(
    short: u16,
    sort: u16,
    id: u16,
    page: Option<u16>,
    size: Option<u16>,
    channel: ChannelType,
) -> Result<String> {
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(20);
    let res = match channel {
        ChannelType::KuGou => kugou::get_mv_list(short, sort, id, page, size).await?,
        _ => "".to_string(),
    };
    Ok(res)
}
