<template>
  <div class="foot-bar-container">
    <div class="control-container flex">
      <div class="to-up-down click-active">
        <img class="image" :src="toUpImg" alt="" />
      </div>
      <div class="play-pause click-active" @click="handlePlayPause">
        <img v-if="state.playing" class="image" :src="toPauseImg" alt="" />
        <img v-else class="image" :src="toPlayImg" alt="" />
      </div>
      <div class="to-up-down click-active">
        <img class="image" :src="toDownImg" alt="" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import toUpImg from '@/assets/imgs/toUp.png';
import toDownImg from '@/assets/imgs/toDown.png';
import toPlayImg from '@/assets/imgs/toPlay.png';
import toPauseImg from '@/assets/imgs/toPause.png';
import { reactive } from 'vue';

const state = reactive({
  playing: false
});

// 播放与暂停
const handlePlayPause = () => {
  state.playing = !state.playing;
};
</script>

<style lang="scss" scoped>
.foot-bar-container {
  height: 60px;
  background-color: $primaryColor;
}
.control-container {
  width: $menuWidth;
  height: 100%;
  align-items: center;
  justify-content: space-evenly;
  .to-up-down {
    width: 34px;
    height: 34px;
    cursor: pointer;
  }
  .play-pause {
    width: 42px;
    height: 42px;
    cursor: pointer;
  }
}
</style>
