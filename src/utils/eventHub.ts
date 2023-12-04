import mitt from 'mitt';

type Events = {
  // 播放
  'music.play': void;
};

export const emitter = mitt<Events>();

export default emitter;
