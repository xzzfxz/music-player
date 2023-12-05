import { SongInfo } from '@/interface';
import { defineStore } from 'pinia';

export const useMainStore = defineStore('main', {
  state() {
    return {
      // 播放中
      isPlaying: false,
      // 当前播放的歌曲
      currentSong: undefined as unknown as SongInfo,
      // 播放列表
      playList: [] as SongInfo[]
    };
  },
  persist: {
    storage: localStorage,
    paths: ['currentSong', 'playList']
  },
  actions: {
    /**
     * @description: 设置是否正在播放
     * @param {boolean} isPlaying
     * @return {*}
     */
    setIsPlaying(isPlaying: boolean) {
      this.isPlaying = isPlaying;
    },
    /**
     * @description: 设置当前播放的歌曲
     * @param {SongInfo} songInfo
     * @return {*}
     */
    setCurrentSong(songInfo: SongInfo) {
      this.currentSong = { ...songInfo };
    },
    getCurrentSong() {
      return { ...(this.currentSong || {}) };
    },
    /**
     * @description: 播放列表
     * @param {SongInfo} list
     * @return {*}
     */
    setPlayList(list: SongInfo[]) {
      this.playList = [...list];
    },
    getPlayList() {
      return [...this.playList];
    }
  }
});

export default useMainStore;
