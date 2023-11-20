/**
 * @description: 子菜单
 * @return {*}
 */
export interface MenuChild {
  id: string;
  name: string;
  icon?: string;
  color?: string;
}

/**
 * @description: 菜单分类
 * @return {*}
 */
export interface MenuCategory {
  title: string;
  showFeedback?: boolean;
  menuList: MenuChild[];
}

/**
 * @description: 歌曲信息
 * @return {*}
 */
export interface SongInfo {
  id?: string;
  singer: string;
  name: string;
  path: string;
  time?: string;
  mv?: boolean;
  album?: string;
  online?: bool;
}
