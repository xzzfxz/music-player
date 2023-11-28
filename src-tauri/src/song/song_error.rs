use thiserror::Error;

#[derive(Debug, Error)]
pub enum SongError {
    #[error("获取歌曲名称错误")]
    NameError,
    #[error("获取歌曲格式错误")]
    ExtError,
}
