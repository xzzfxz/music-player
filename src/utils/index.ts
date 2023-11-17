import { MUSIC_EXT } from '@/const';

/**
 * @description: 根据文件名称获取歌手和歌名
 * @param {string} name 文件名
 * @return {*}
 */
export const getSingerAndName = (name: string) => {
  const arr = name.split('-').map((item: string) => item.trim());
  const nameAndExt = arr.splice(arr.length - 1, 1)[0];
  const nameExtArr = nameAndExt.split('.');
  if (
    nameExtArr.length > 1 &&
    MUSIC_EXT.includes(nameExtArr[nameExtArr.length - 1])
  ) {
    // 有音乐后缀名，把后缀名删除
    nameExtArr.splice(nameExtArr.length - 1, 1);
  }
  const songName = nameExtArr.join('.') || '未知歌名';
  const singer = arr.join('-') || '未知歌手';
  return { songName, singer };
};
