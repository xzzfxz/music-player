# Tauri + Vue 3 + TypeScript

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 使用

### 前置

自己先安装 rust 的环境（网上很多教程）

### 步骤

1. 下载此项目到本地 `git clone https://github.com/xzzfxz/music-player.git`
2. 进入项目执行命令 `pnpm install`
3. 本地运行 `pnpm tauri dev`
4. 打包 `pnpm tauri build`
   打包后的文件在项目根目录`/src-tauri/target/release`下

## 功能

#### 支持的格式

1. mp3
2. wav
3. flac

#### 本地音乐

1. 添加本地音乐
2. 播放，上/下一曲
3. 删除
4. 播放进度条
5. 音量调整

#### 在线音乐

##### 酷狗

1. 搜索
2. mv 列表

#### 状态

开发完善中...
