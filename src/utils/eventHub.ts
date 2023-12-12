import mitt from 'mitt';

type Events = {
  // 播放
  'music.play': boolean;
  // 刷新当前路由
  'router.reload': void;
};

export const emitter = mitt<Events>();

export default emitter;
