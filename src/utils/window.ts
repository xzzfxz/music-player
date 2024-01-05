import { WINDOW_NAME } from '@/enum';
import { WindowOptions } from '@tauri-apps/api/window';
import { WebviewWindow } from '@tauri-apps/api/window';

/**
 * @description: 创建窗口
 * @param name 窗口名称
 * @param options 窗口配置
 * @return {*}
 */
export const createWindow = (
  name: WINDOW_NAME,
  options: WindowOptions = {}
) => {
  const webview = new WebviewWindow(name, {
    fullscreen: false,
    alwaysOnTop: true,
    height: 460,
    width: 340,
    resizable: false,
    skipTaskbar: true,
    titleBarStyle: 'overlay',
    title: '',
    transparent: false,
    ...options
  });
  console.log(webview);
};
