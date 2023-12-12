import axios from 'axios';

export const instance = axios.create({
  timeout: 30000
});

export default instance;
