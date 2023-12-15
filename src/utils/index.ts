import { MUSIC_EXT } from '@/const';
import { v4 as uuidV4 } from 'uuid';

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

/**
 * @description: 数字小于10时补0
 * @param {number} num
 * @return {*}
 */
export const formatZero = (num: number) => {
  if (num < 10) {
    return '0' + num;
  }
  return num + '';
};

/**
 * @description: 格式化时间格式：HH:mm:ss
 * @param {number} time
 * @return {*}
 */
export const getFormatPlayTime = (time: number) => {
  let lastTime = '';
  const hour = Math.floor(time / 60 / 60);
  if (hour > 0) {
    lastTime += formatZero(hour) + ':';
  }
  const min = Math.floor((time - hour * 60 * 60) / 60);
  lastTime += formatZero(min) + ':';
  const sec = (time - hour * 60 * 60 - min * 60) % 60;
  lastTime += formatZero(sec);
  return lastTime;
};

/**
 * @description: 生成uuid
 * @return {*}
 */
export const getUUID = () => {
  return uuidV4();
};
