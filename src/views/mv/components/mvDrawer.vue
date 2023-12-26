<template>
  <el-drawer
    v-model="state.showDrawer"
    direction="btt"
    class="video-drawer-container"
    size="100%"
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
      <div class="body-container" data-tauri-drag-region>
        <video
          ref="videoRef"
          class="video"
          autoplay
          oncontextmenu="return false"
          data-tauri-drag-region
          :src="state.src"
          @error="handlePlayError"
        ></video>
      </div>
    </template>
    <template #footer>
      <div class="footer-container flex">
        <Controls />
      </div>
    </template>
  </el-drawer>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, reactive, nextTick } from 'vue';
import Controls from './controls.vue';
import emitter from '@/utils/eventHub';
import { MvInfo } from '@/interface/event';

const state = reactive({
  showDrawer: false,
  src: '',
  mvList: [] as MvInfo[],
  current: {} as MvInfo
});

// 播放错误
const handlePlayError = (e: any) => {
  console.log(e.target.error);
};

// 关闭mv播放
const handleCloseDrawer = () => {
  state.showDrawer = false;
};

// 获取mv列表
const handleGetMvList = (list: MvInfo[]) => {
  state.mvList = list;
  state.showDrawer = true;
  nextTick(() => {
    if (list.length) {
      state.current = list[0];
      state.src = state.current.url;
    }
  });
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
