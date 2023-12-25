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
          <div class="title" data-tauri-drag-region>周杰伦 - 青花瓷</div>
        </div>
      </div>
    </template>
    <template #default>
      <div class="body-container" data-tauri-drag-region>
        <video
          ref="videoRef"
          class="video"
          oncontextmenu="return false"
          data-tauri-drag-region
          autoplay
          :src="state.src"
          @error="handlePlayError"
        ></video>
      </div>
    </template>
    <template #footer>
      <div class="footer-container flex">
        <div class="back-icon"></div>
        <div class="title"></div>
      </div>
    </template>
  </el-drawer>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const state = reactive({
  showDrawer: false,
  src: 'http://fsmvpc.ali.kugou.com/202312251756/944f28bc30a86ea4b71aeaba831abac3/KGTX/CLTX002/b7f7fa5a2dd3861c8f3bcb16fbab0da0.mp4'
});

// 播放错误
const handlePlayError = (e: any) => {
  console.log(e.target.error);
};

// 关闭mv播放
const handleCloseDrawer = () => {
  state.showDrawer = false;
};
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
