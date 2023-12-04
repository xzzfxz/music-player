<template>
  <div class="audio-container">
    <audio ref="audioRef" :src="state.src" preload="auto"></audio>
  </div>
</template>

<script setup lang="ts">
import { SongInfo } from '@/interface';
import useMainStore from '@/store';
import emitter from '@/utils/eventHub';
import { onBeforeUnmount, onMounted, reactive, ref, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api';

const mainStore = useMainStore();

const audioRef = ref();
const state = reactive({
  src: '',
  songInfo: {} as SongInfo
});

// 播放
const handleMusicPlay = async () => {
  state.songInfo = mainStore.getCurrentSong();
  // state.src =
  //   'https://webfs.hw.kugou.com/202312041620/9b5c4918bba5edca35f0b6313259e631/v2/3db322e5cdd76350323e8cee789becab/G286/M09/42/8B/_pQEAGVTV1SACnKYAD9ID-Yz3BY281.mp3';
  // const au = document.querySelector('#test');
  if (state.songInfo.online) {
    // 在线
    state.src = state.songInfo.path;
  } else {
    // 本地文件
    const buffer: any = await invoke('read_local_song', {
      filePath: state.songInfo.path
    });
    if (!buffer.length) {
      return;
    }
    const blob = new Blob([new Uint8Array(buffer)], { type: 'audio/mpeg' });
    const url = URL.createObjectURL(blob);
    console.log(url);
    state.src = url;
  }
  nextTick(() => {
    audioRef.value.play();
  });
};

// 初始化事件
const initEvent = () => {
  emitter.on('music.play', handleMusicPlay);
};

// 取消监听事件
const offEvent = () => {
  emitter.off('music.play', handleMusicPlay);
};

onMounted(() => {
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
