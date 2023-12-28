<template>
  <el-drawer
    v-model="state.showDrawer"
    direction="btt"
    class="video-drawer-container"
    size="100%"
    :close-on-press-escape="false"
    :destroy-on-close="true"
    :show-close="false"
  >
    <template #header>
      <div class="header-container flex" data-tauri-drag-region>
        <div class="content flex">
          <div class="back-icon click-active" @click="handleCloseDrawer">
            <i class="ri-arrow-left-s-line"></i>
          </div>
          <div class="title" @click="handleCloseDrawer">
            {{ state.current.singer }} - {{ state.current.name }}
          </div>
        </div>
      </div>
    </template>
    <template #default>
      <div
        class="body-container"
        data-tauri-drag-region
        @mousedown="handleVideoDown"
        @mouseup="handleVideoUp"
        @click.prevent="handleToPlayPause(!state.isPlaying)"
      >
        <video
          v-show="state.src"
          ref="videoRef"
          class="video"
          autoplay
          oncontextmenu="return false"
          data-tauri-drag-region
          :src="state.src"
          @error="handlePlayError"
          @ended="handlePlayEnd"
          @progress="handleCacheProgress"
          @play="handlePlayPause(true)"
          @pause="handlePlayPause(false)"
        ></video>
      </div>
    </template>
    <template #footer>
      <div class="footer-container">
        <Controls
          ref="controlsRef"
          :data="state.current"
          :isPlaying="state.isPlaying"
          :currentTime="state.currentTime"
          @playPause="handleToPlayPause"
          @sliderChange="handleSliderChange"
          @volumeChange="handleSetVolume"
        />
      </div>
    </template>
  </el-drawer>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, reactive, nextTick, ref } from 'vue';
import Controls from './controls.vue';
import emitter from '@/utils/eventHub';
import { MvInfo } from '@/interface/event';
import { ElMessage } from 'element-plus';
import useMainStore from '@/store';
import { appWindow } from '@tauri-apps/api/window';

const mainStore = useMainStore();

const videoRef = ref();
const controlsRef = ref();

const state = reactive({
  showDrawer: false,
  src: '',
  mvList: [] as MvInfo[],
  current: {} as MvInfo,
  currentTime: 0,
  timeTimeOut: 0,
  isPlaying: true,
  isPress: false,
  pressTime: 0
});

// 定时获取播放进度
const handleGetCurrentTime = () => {
  // 播放进度
  const time: number = videoRef.value?.currentTime || 0;
  state.currentTime = Number(Math.floor(time));
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

// 关闭mv播放
const handleCloseDrawer = async () => {
  const isFull = await appWindow.isFullscreen();
  if (isFull) {
    await appWindow.setFullscreen(false);
  }
  state.showDrawer = false;
  handleStopGetCurrentTime();
};

// 获取mv列表
const handleGetMvList = (list: MvInfo[]) => {
  state.mvList = list;
  state.showDrawer = true;
  nextTick(() => {
    if (list.length) {
      state.current = list[0];
      state.src = state.current.url;
      handleGetCurrentTime();
    }
    handleSetVolume();
  });
};

// 播放结束，播放有mv的下一首
const handlePlayEnd = () => {
  state.isPlaying = false;
};

// 缓冲进度变化
const handleCacheProgress = () => {
  const buffered = videoRef.value?.buffered;
  if (!buffered) {
    return;
  }
  if (buffered?.length > 0) {
    const bufferedTime = buffered.end(0); // 获取已缓冲的最后时间点
    const progress = ((bufferedTime / videoRef.value.duration) * 100).toFixed(
      0
    ); // 计算已缓冲的百分比
    controlsRef.value?.handleCacheChange(Number(progress));
  }
};

// 播放或暂停
const handlePlayPause = (isPlay: boolean) => {
  state.isPlaying = isPlay;
};

// 播放错误
const handlePlayError = () => {
  if (!state.src) {
    return;
  }
  ElMessage.error('MV播放错误');
};

// 鼠标按下
const handleVideoDown = () => {
  state.pressTime = new Date().getTime();
};
// 鼠标抬起
const handleVideoUp = () => {
  let curTime = new Date().getTime();
  let dis = curTime - state.pressTime;
  if (dis > 100) {
    // 长按
    state.isPress = true;
  } else {
    state.isPress = false;
  }
};

// 去播放暂停
const handleToPlayPause = (toPlay: boolean) => {
  if (state.isPress) {
    return;
  }
  if (state.currentTime >= Math.floor(state.current.duration)) {
    videoRef.value.currentTime = 0;
  }
  if (toPlay) {
    videoRef.value?.play();
    handleGetCurrentTime();
  } else {
    videoRef.value?.pause();
    handleStopGetCurrentTime();
  }
};

// 设置音量
const handleSetVolume = () => {
  videoRef.value.volume = mainStore.volume / 100;
};

// 滑动到特定位置
const handleSliderChange = (time: number) => {
  videoRef.value.currentTime = time;
};

// 初始化事件
const initEvent = () => {
  emitter.on('mv.play', handleGetMvList);
};

// 取消监听事件
const offEvent = () => {
  emitter.off('mv.play', handleGetMvList);
};

onMounted(() => {
  initEvent();
});

onBeforeUnmount(() => {
  offEvent();
});
</script>

<style lang="scss" scoped>
.header-container {
  height: 64px;
  background-color: #000;
  align-items: center;
  .content {
    align-items: center;
    font-size: 18px;
    color: #cfcfcf;
    cursor: default;
    .back-icon {
      margin-left: 8px;
      font-size: 24px;
      cursor: pointer;
    }
  }
}
.body-container {
  height: 100%;
  background-color: #000;
  overflow: hidden;
  .video {
    width: 100%;
    height: 100%;
    // object-fit: fill;
  }
}
.footer-container {
  height: 64px;
  background-color: #000;
}
</style>
<style lang="scss">
.video-drawer-container {
  .el-drawer__header {
    padding: 0;
    margin-bottom: 0;
  }
  .el-drawer__body {
    height: 0;
    padding: 0;
  }
  .el-drawer__footer {
    padding: 0;
  }
}
</style>
