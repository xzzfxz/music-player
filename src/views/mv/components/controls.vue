<template>
  <div class="controls-container flex">
    <div class="left-container flex">
      <div class="play-icon click-active">
        <i class="ri-play-fill"></i>
        <!-- <i class="ri-pause-fill"></i> -->
      </div>
      <div class="duration">
        00:02
        <span class="total-duration">/02:30</span>
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
import { reactive } from 'vue';

const state = reactive({
  volume: 50
});
</script>

<style lang="scss" scoped>
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
