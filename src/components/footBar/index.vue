<template>
  <div class="foot-bar-container flex">
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
    <div class="avatar-container">
      <img :src="state.avatarUrl" alt="" class="image" />
    </div>
    <div class="play-info-container">
      <div class="music-info-container flex">
        <div class="left-container flex">
          <div class="song-quality no-shrink">
            <SoundQuality />
          </div>
          <div class="song-info">
            <TextScroll :text="state.songInfo" />
          </div>
        </div>
        <div class="right-container flex no-shrink">00:18 / 04:02</div>
      </div>
      <div class="play-progress-container">
        <div class="cache-container">
          <el-progress
            color="#3061A4"
            :percentage="state.cacheProgress"
            :show-text="false"
            :stroke-width="3"
          />
        </div>
        <div class="cache-container">
          <el-slider
            v-model="state.playProgress"
            :min="state.playMin"
            :max="state.playMax"
            :show-tooltip="false"
            @change="handlePlayProgressChange"
          />
        </div>
      </div>
    </div>
    <div
      class="icon-container like-container"
      :class="{ liked: state.hasLiked }"
    >
      <i v-if="state.hasLiked" class="ri-heart-3-fill"></i>
      <i v-else class="ri-heart-3-line"></i>
    </div>
    <div class="icon-container download-container">
      <i class="ri-download-2-line"></i>
    </div>
    <div class="icon-container">
      <SoundDeal />
    </div>
  </div>
</template>

<script setup lang="ts">
import toUpImg from '@/assets/imgs/toUp.png';
import toDownImg from '@/assets/imgs/toDown.png';
import toPlayImg from '@/assets/imgs/toPlay.png';
import toPauseImg from '@/assets/imgs/toPause.png';
import { reactive } from 'vue';
import SoundQuality from './component/soundQuality.vue';
import TextScroll from '@/components/TextScroll/index.vue';
import SoundDeal from './component/soundDeal.vue';

const state = reactive({
  playing: false,
  avatarUrl: '',
  songInfo: '风吹沙 - 蝶恋花，古道旁，我欲语泪先下，山河大好',
  cacheProgress: 50,
  playProgress: 30,
  playMin: 0,
  playMax: 100,
  hasLiked: true
});

// 播放与暂停
const handlePlayPause = () => {
  state.playing = !state.playing;
};

// 播放进度改变
const handlePlayProgressChange = (val: number) => {
  console.log(val);
};
</script>

<style lang="scss" scoped>
.foot-bar-container {
  height: 100%;
  align-items: center;
  background-color: rgba(28, 126, 255, 0.8);
  backdrop-filter: blur(2px);
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
.avatar-container {
  width: 44px;
  height: 44px;
  border-radius: 4px;
  overflow: hidden;
  background-image: url('@/assets/imgs/ktv.png');
  background-size: 80%;
  background-repeat: no-repeat;
  background-position: center;
  background-color: #458ceb;
}
.play-info-container {
  width: 368px;
  margin-left: 14px;
  padding-bottom: 8px;
  .music-info-container {
    align-items: center;
    width: 100%;
    .left-container {
      width: 0;
      flex-grow: 1;
      align-items: center;
      .song-info {
        width: 0;
        margin-left: 8px;
        flex-grow: 1;
        white-space: nowrap;
        color: #fff;
        font-size: 12px;
        overflow: hidden;
      }
    }
    .right-container {
      width: 98px;
      justify-content: flex-end;
      font-size: 12px;
      color: #fff;
    }
  }
}
.play-progress-container {
  margin-top: 6px;
  position: relative;
  :deep(.el-progress) {
    .el-progress-bar__outer {
      background-color: #3b79cb;
    }
  }
  :deep(.el-slider) {
    height: 3px;
    .el-slider__runway {
      height: 3px;
      background-color: transparent;
    }
    $markWidth: 7px;
    .el-slider__button-wrapper {
      width: $markWidth;
      height: $markWidth;
      top: -11px;
      .el-slider__button {
        width: $markWidth;
        height: $markWidth;
        border: none;
      }
    }
    .el-slider__bar {
      height: 3px;
      background-color: #fff;
    }
  }
  .cache-container {
    width: 100%;
    position: absolute;
    left: 0;
    top: 0;
  }
}
.icon-container {
  margin-left: 18px;
  color: #fff;
  font-size: 19px;
}
.like-container {
  margin-left: 25px;
  &.liked {
    color: #e7442e;
  }
}
</style>
