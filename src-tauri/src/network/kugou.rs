use anyhow::Result;

pub async fn search_songs(keyword: String) -> Result<String> {
    let url = format!("https://complexsearch.kugou.com/v2/search/song?appid=1155&area_code=1&clienttime=1702452399854&clientver=313&dfid=-&iscorrection=7&keyword={}&mid=cbdc97dc723c7ea037cd44522b5c066e&page=1&pagesize=20&platform=WebFilter&requestid=14&signature=d8bdf3b77823a6635821e47fb7f519c9&srcappid=2919&tag=em&token=296259bd43f2743da3f2e8464e23c24c0ef266d1b2b10baf37d8f285e5b14c95&userid=577275546&uuid=5cf7a4b151a470ba03a65dae796fdb4f", keyword);
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    if res.status().is_success() {
        let body = res.text().await?;
        Ok(body)
    } else {
        Ok("".to_string())
    }
}
