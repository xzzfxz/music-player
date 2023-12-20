/**
 * @description: 和后端交互的事件名称
 * @return {*}
 */
export enum EventName {
  // ***********************************************监听后端事件*********************************************************************

  // 刷新本地音乐列表
  RELOAD_LOCAL_SONG_LIST = 'reloadLocalSongList',

  // ***********************************************触发后端事件*********************************************************************

  // 打开文件选择对话框
  OPEN_SONG_DIALOG = 'open_song_dialog',
  // 获取本地音乐列表
  GET_LOCAL_SONG_LIST = 'get_local_song_list',
  // 删除本地音乐
  DELETE_LOCAL_SONG = 'delete_local_song',
  // 打开文件夹
  OPEN_FOLDER = 'open_folder',
  // 在线查询音乐
  SEARCH_SONGS = 'search_songs',
  // 获取音频播放地址
  GET_SONG_INFO = 'get_song_info',
  // 获取mv分类
  GET_MV_CATEGORY = 'get_mv_category',
  // 获取mv列表
  GET_MV_LIST = 'get_mv_list'
}
