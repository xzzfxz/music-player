use thiserror::Error;

/**
 * @description: 文件相关错误信息
 * @return {*}
 */
#[derive(Error, Debug)]
pub enum FileError {
    #[error("文件路径获取失败")]
    PathError,
    #[error("文件未找到")]
    FileNotFound,
    #[error("文件读取失败")]
    FileReadFail,
    #[error("获取文件名失败")]
    FileNameFail,
}
