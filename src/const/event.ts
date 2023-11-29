/**
 * @description: 和后端交互的事件名称
 * @return {*}
 */
export enum EventName {
  // 监听后端事件---------------------------------------------------
  // 刷新本地音乐列表
  RELOAD_LOCAL_SONG_LIST = 'reloadLocalSongList',
  // 触发后端事件---------------------------------------------------
  // 打开文件选择对话框
  OPEN_SONG_DIALOG = 'open_song_dialog',
  // 获取本地音乐列表
  GET_LOCAL_SONG_LIST = 'get_local_song_list'
}