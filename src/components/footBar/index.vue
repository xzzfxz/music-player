<template>
  <div class="foot-bar-container flex">
    <div class="foot-bar-left flex">
      <div class="control-container flex">
        <div class="to-up-down click-active" @click="handleUpDownSong(true)">
          <img class="image" :src="toUpImg" alt="" />
        </div>
        <div class="play-pause click-active" @click="handlePlayPause">
          <img
            v-if="mainStore.isPlaying"
            class="image"
            :src="toPauseImg"
            alt=""
          />
          <img v-else class="image" :src="toPlayImg" alt="" />
        </div>
        <div class="to-up-down click-active" @click="handleUpDownSong(false)">
          <img class="image" :src="toDownImg" alt="" />
        </div>
      </div>
      <div class="avatar-container">
        <img :src="curSong.avatar" alt="" class="image" />
      </div>
      <div class="play-info-container">
        <div class="music-info-container flex">
          <div class="left-container flex">
            <div class="song-quality no-shrink">
              <SoundQuality />
            </div>
            <div class="song-info">
              <TextScroll :text="curSong.singer + ' - ' + curSong.name" />
            </div>
          </div>
          <div class="right-container flex no-shrink">
            {{ playedTime }} / {{ curSong.time }}
          </div>
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
              :min="0"
              :max="curSong.duration"
              :show-tooltip="false"
              @change="handlePlayProgressChange"
            />
          </div>
        </div>
      </div>
    </div>
    <div class="foot-bar-right flex">
      <div
        class="icon-container like-container"
        :class="{ liked: state.hasLiked }"
      >
        <i v-if="state.hasLiked" class="ri-heart-3-fill"></i>
        <i v-else class="ri-heart-3-line"></i>
      </div>
      <div class="icon-container click-active download-container">
        <i class="ri-download-2-line"></i>
      </div>
      <div class="icon-container click-active">
        <i class="ri-apps-line"></i>
      </div>
      <div class="icon-container" title="音效">
        <i class="ri-equalizer-line"></i>
      </div>
      <div class="icon-container click-active">
        <span>词</span>
      </div>
      <div class="icon-container" title="顺序播放">
        <i class="ri-order-play-line"></i>
      </div>
      <div class="icon-container">
        <el-popover
          placement="top"
          trigger="hover"
          popper-class="volume-popper"
        >
          <template #reference>
            <div><i class="ri-volume-up-line"></i></div>
          </template>
          <el-slider
            v-model="state.volume"
            size="small"
            vertical
            height="120px"
          />
        </el-popover>
      </div>
      <div class="icon-container">
        <i class="ri-play-list-2-line"></i>
        <span class="play-num">{{ mainStore.getPlayList().length }}</span>
      </div>
    </div>
  </div>
  <Play
    ref="playRef"
    @currentTimeChange="handleCurrentTimeChange"
    @bufferChange="handleBufferChange"
    @playEnd="handlePlayEnd"
  />
</template>

<script setup lang="ts">
import toUpImg from '@/assets/imgs/toUp.png';
import toDownImg from '@/assets/imgs/toDown.png';
import toPlayImg from '@/assets/imgs/toPlay.png';
import toPauseImg from '@/assets/imgs/toPause.png';
import { computed, reactive, ref, watch } from 'vue';
import SoundQuality from './component/soundQuality.vue';
import TextScroll from '@/components/TextScroll/index.vue';
import Play from './component/play.vue';
import useMainStore from '@/store';
import emitter from '@/utils/eventHub';
import { getFormatPlayTime } from '@/utils';
import { getUpDownSong } from '@/utils/play';

const mainStore = useMainStore();

const playRef = ref();

const state = reactive({
  cacheProgress: 0,
  playProgress: 0,
  hasLiked: true,
  volume: mainStore.volume
});

// 当前歌曲
const curSong = computed(() => {
  return mainStore.getCurrentSong();
});

// 播放进度
const playedTime = computed(() => {
  return getFormatPlayTime(state.playProgress);
});

// 播放与暂停
const handlePlayPause = () => {
  emitter.emit('music.play', !mainStore.isPlaying);
};

// 上一曲------下一曲
const handleUpDownSong = (isUp: boolean) => {
  const next = getUpDownSong(isUp);
  if (!next) {
    return;
  }
  mainStore.setIsPlaying(false);
  mainStore.setCurrentSong(next);
  emitter.emit('music.play', true);
};

// 播放进度改变
const handlePlayProgressChange = (val: number) => {
  playRef.value?.handleSetCurrentTime(val);
};

// 缓冲进度改变
const handleBufferChange = (progress: number) => {
  state.cacheProgress = progress;
};

// 播放时间改变
const handleCurrentTimeChange = (current: number) => {
  state.playProgress = current;
};

// 播放结束
const handlePlayEnd = () => {
  // 根据规则播放下一曲
  handleUpDownSong(false);
};

// 音量改变，及时调整音量
watch(
  () => state.volume,
  val => {
    mainStore.setVolume(val);
    playRef.value.handleSetVolume();
  }
);
</script>

<style lang="scss" scoped>
.foot-bar-container {
  height: 100%;
  align-items: center;
  justify-content: space-between;
  background-color: rgba(28, 126, 255, 0.8);
  backdrop-filter: blur(2px);
}
.foot-bar-right {
  padding-right: 20px;
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
    height: 5px;
    .el-slider__runway {
      height: 5px;
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
  cursor: pointer;
}
.like-container {
  margin-left: 25px;
  &.liked {
    color: #e7442e;
  }
}
.play-num {
  font-size: 16px;
}
</style>
<style lang="scss">
.volume-popper {
  &.el-popper {
    min-width: 0;
    width: fit-content !important;
    padding: 8px 0 4px;
    $sliderWidth: 3px;
    .el-slider {
      &.is-vertical {
        .el-slider__runway {
          width: $sliderWidth;
        }
        $markWidth: 11px;
        .el-slider__button-wrapper {
          width: $markWidth;
          height: $markWidth;
          left: -4px;
          .el-slider__button {
            width: $markWidth;
            height: $markWidth;
            background-color: $primaryColor;
          }
        }
        .el-slider__bar {
          width: $sliderWidth;
        }
      }
    }
  }
}
</style>
