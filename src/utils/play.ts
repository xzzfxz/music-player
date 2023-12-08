import useMainStore from '@/store';

/**
 * @description: 获取上一首，下一首
 * @param {boolean} isUp 是否为上一首
 * @return {*} 空或歌曲
 */
export const getUpDownSong = (isUp: boolean) => {
  const mainStore = useMainStore();
  const list = mainStore.getPlayList();
  const curSong = mainStore.getCurrentSong();
  let index = list.findIndex(item => item.id === curSong.id);
  if (isUp) {
    // 上一曲
    if (index < 1) {
      return;
    }
    index--;
  } else {
    // 下一曲
    if (index >= list.length - 1) {
      // 回到第一首
      index = 0;
    } else {
      index++;
    }
  }
  return list[index];
};
