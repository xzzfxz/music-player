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
