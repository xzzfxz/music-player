import axios from './api';

export const fetchKuGou = (keyword: string) => {
  const data = {
    input: keyword,
    filter: 'name',
    type: 'netease',
    page: 1
  };
  return axios.post('https://music.liuzhijin.cn/', data, {
    headers: { Origin: 'https://music.liuzhijin.cn' }
  });
};
