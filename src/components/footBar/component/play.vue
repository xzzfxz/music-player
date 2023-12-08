<template>
  <div class="audio-container">
    <audio
      ref="audioRef"
      :src="state.src"
      preload="auto"
      @ended="handlePlayEnd"
      @progress="handleCacheProgress"
    ></audio>
  </div>
</template>

<script setup lang="ts">
import { SongInfo } from '@/interface';
import useMainStore from '@/store';
import emitter from '@/utils/eventHub';
import { onBeforeUnmount, onMounted, reactive, ref, nextTick } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { exists } from '@tauri-apps/api/fs';
import { ElMessage } from 'element-plus';

const emit = defineEmits(['currentTimeChange', 'bufferChange', 'playEnd']);

const mainStore = useMainStore();

const audioRef = ref();
const state = reactive({
  src: '',
  songInfo: {} as SongInfo,
  // 定时获取播放进度
  timeTimeOut: 0
});

// 播放
const handleMusicPlay = async (isPlay: boolean = false) => {
  if (!isPlay) {
    // 暂停
    handlePause();
    return;
  }
  const storeSong = mainStore.getCurrentSong();
  if (state.songInfo.id === storeSong.id) {
    // 由暂停直接播放
    handlePlay();
    return;
  }
  state.songInfo = storeSong;
  // state.src =
  //   'https://webfs.hw.kugou.com/202312061032/064aad4257236651dbb7412e04805d00/KGTX/CLTX001/64ff991a44c4b9f99ce33e59d7e1f8e5.mp3';
  if (state.songInfo.online) {
    // 在线
    state.src = state.songInfo.path;
  } else {
    // 本地文件
    const isExist = await exists(state.songInfo.path);
    if (!isExist) {
      ElMessage.error('文件不存在!');
      return;
    }
    const filePath = convertFileSrc(state.songInfo.path);
    state.src = filePath;
  }
  nextTick(() => {
    handlePlay();
  });
};

// 定时获取播放进度
const handleGetCurrentTime = () => {
  // 播放进度
  const time: number = audioRef.value.currentTime;
  emit('currentTimeChange', Number(time.toFixed(0)));
  state.timeTimeOut = window.setTimeout(() => {
    handleGetCurrentTime();
  }, 1000);
};
// 暂停获取播放进度
const handleStopGetCurrentTime = () => {
  if (state.timeTimeOut) {
    clearTimeout(state.timeTimeOut);
  }
};

// 播放开始
const handlePlay = () => {
  audioRef.value.play();
  mainStore.setIsPlaying(true);
  handleGetCurrentTime();
};

// 播放暂停
const handlePause = () => {
  audioRef.value.pause();
  handleStopGetCurrentTime();
  mainStore.setIsPlaying(false);
};

// 缓冲进度变化
const handleCacheProgress = () => {
  const buffered = audioRef.value.buffered;
  if (buffered.length > 0) {
    const bufferedTime = buffered.end(0); // 获取已缓冲的最后时间点
    const progress = (bufferedTime / audioRef.value.duration) * 100; // 计算已缓冲的百分比
    emit('bufferChange', progress);
  }
};

// 播放结束
const handlePlayEnd = () => {
  handlePause();
  emit('playEnd');
};

// 设置播放进度
const handleSetCurrentTime = (currentTime: number) => {
  audioRef.value.currentTime = currentTime;
};

// 设置音量
const handleSetVolume = () => {
  audioRef.value.volume = mainStore.volume / 100;
};

defineExpose({ handleSetCurrentTime, handleSetVolume });

// 初始化事件
const initEvent = () => {
  emitter.on('music.play', handleMusicPlay);
};

// 取消监听事件
const offEvent = () => {
  emitter.off('music.play', handleMusicPlay);
  handleStopGetCurrentTime();
};

onMounted(() => {
  handleSetVolume();
  initEvent();
});

onBeforeUnmount(() => {
  offEvent();
});
</script>

<style scoped lang="scss">
.audio-container {
  position: absolute;
  z-index: -1;
}
</style>
