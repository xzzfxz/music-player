import mitt from 'mitt';

type Events = {
  // 播放
  'music.play': boolean;
};

export const emitter = mitt<Events>();

export default emitter;
