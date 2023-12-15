/**
 * @description: 歌曲菜单事件
 * @return {*}
 */
export enum MENU_EVENT {
  // 播放
  PLAY = 'play',
  // 删除
  DELETE = 'delete',
  // 删除（包含文件）
  DELETE_FILE = 'deleteFile',
  // 打开所在文件夹
  OPEN_FOLDER = 'openFolder'
}

/**
 * @description: 音乐渠道类型
 * @return {*}
 */
export enum CHANNEL_TYPE {
  KU_GOU = 'KuGou'
}
