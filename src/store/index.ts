import { SongInfo } from '@/interface';
import { defineStore } from 'pinia';

export const useMainStore = defineStore('main', {
  state() {
    return {
      currentSong: undefined as unknown as SongInfo
    };
  },
  persist: {
    storage: localStorage,
    paths: ['currentSong']
  },
  actions: {
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
    }
  }
});

export default useMainStore;
