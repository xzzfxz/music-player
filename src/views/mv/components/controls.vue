<template>
  <div class="play-progress-container">
    <div class="cache-container">
      <el-progress
        color="#474747"
        :percentage="state.cacheProgress"
        :show-text="false"
        :stroke-width="2"
      />
    </div>
    <div class="cache-container slider">
      <el-slider
        v-model="state.playProgress"
        :min="0"
        :max="totalDuration"
        :show-tooltip="false"
        @change="handlePlayProgressChange"
      />
    </div>
  </div>
  <div class="controls-container flex">
    <div class="left-container flex">
      <div class="play-icon click-active">
        <i
          class="ri-pause-fill"
          v-if="props.isPlaying"
          @click="handlePlayPause(false)"
        ></i>
        <i class="ri-play-fill" v-else @click="handlePlayPause(true)"></i>
      </div>
      <div class="duration">
        {{ getFormatPlayTime(props.currentTime) }}
        <span class="total-duration">
          /{{ getFormatPlayTime(totalDuration) }}
        </span>
      </div>
    </div>
    <div class="right-container flex">
      <div class="icon-container">
        <el-popover
          placement="top"
          trigger="hover"
          popper-class="mv-volume-popper"
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
      <div class="fullscreen click-active">
        <i class="ri-fullscreen-fill"></i>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MvInfo } from '@/interface/event';
import useMainStore from '@/store';
import { PropType, computed, reactive, watch } from 'vue';
import { getFormatPlayTime } from '@/utils';

const emit = defineEmits(['playPause', 'sliderChange', 'volumeChange']);

const props = defineProps({
  data: {
    type: Object as PropType<MvInfo>,
    default: () => ({})
  },
  // 当前播放时间
  currentTime: {
    type: Number,
    default: 0
  },
  isPlaying: {
    type: Boolean,
    default: false
  }
});

const mainStore = useMainStore();

const state = reactive({
  volume: mainStore.volume,
  cacheProgress: 0,
  playProgress: 0
});

// 当前mv总时长
const totalDuration = computed(() => {
  if (!props.data.duration) {
    return 0;
  }
  return Number(Math.floor(props.data.duration / 1000));
});

// 播放或暂停
const handlePlayPause = (isToPlay: boolean) => {
  emit('playPause', isToPlay);
};

// 拖动改变播放进度
const handlePlayProgressChange = (val: number) => {
  emit('sliderChange', val);
};

// 缓存进度改变
const handleCacheChange = (progress: number) => {
  state.cacheProgress = progress;
};

watch(
  () => props.currentTime,
  () => {
    state.playProgress = props.currentTime;
  }
);

// 音量改变，及时调整音量
watch(
  () => state.volume,
  val => {
    mainStore.setVolume(val);
    emit('volumeChange');
  }
);

defineExpose({ handleCacheChange });
</script>

<style lang="scss" scoped>
.play-progress-container {
  width: 100%;
  position: relative;
  :deep(.el-progress) {
    .el-progress-bar__outer {
      background-color: #1a1a1b;
    }
  }
  :deep(.el-slider) {
    height: 6px;
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
      height: 2px;
      background-color: $primaryColor;
    }
  }
  .cache-container {
    width: 100%;
    position: absolute;
    left: 0;
    top: 3px;
    &.slider {
      top: 2px;
    }
  }
}
.controls-container {
  width: 100%;
  height: 100%;
  padding: 0 20px;
  align-items: center;
  justify-content: space-between;
  color: #cfcfcf;
  .left-container {
    align-items: center;
    .play-icon {
      font-size: 26px;
      &:hover {
        color: #fff;
      }
    }
    .duration {
      margin-left: 16px;
      color: #fff;
      font-size: 14px;
      .total-duration {
        color: #7f7f7f;
      }
    }
  }
  .right-container {
    font-size: 22px;
    .fullscreen {
      margin-left: 16px;
      &:hover {
        color: #fff;
      }
    }
  }
}
</style>
<style lang="scss">
.mv-volume-popper {
  &.el-popper {
    min-width: 0;
    width: fit-content !important;
    padding: 8px 0 4px;
    background-color: #000;
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
