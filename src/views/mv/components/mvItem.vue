<template>
  <div class="mv-list-container flex">
    <div
      class="mv-item-container flex"
      v-for="item in props.list"
      :key="item.videoid"
    >
      <div class="mv-cover">
        <img class="image" :src="replaceImgSize(item.img)" />
      </div>
      <div class="title ellipsis">
        {{ item.title }}
        <span v-if="item.remark">({item.remark})</span>
      </div>
      <div class="desc ellipsis">
        {{ item.singername }}《{{ item.videoname }}》
      </div>
    </div>
    <i class="mv-item-container" />
    <i class="mv-item-container" />
    <i class="mv-item-container" />
  </div>
</template>

<script setup lang="ts">
import { PropType } from 'vue';
import { replaceImgSize } from '@/utils';

const props = defineProps({
  list: {
    type: Array as PropType<any>,
    default: () => []
  }
});
</script>

<style lang="scss" scoped>
.mv-list-container {
  flex-wrap: wrap;
  justify-content: space-between;
}
.mv-item-container {
  width: 224px;
  margin-right: 8px;
  margin-bottom: 20px;
  flex-direction: column;
  .mv-cover {
    width: 100%;
    height: 127px;
    cursor: pointer;
  }
  .title {
    margin-top: 8px;
    font-size: 14px;
    line-height: 16px;
    &:hover {
      color: $primaryColor;
      cursor: pointer;
    }
  }
  .desc {
    margin-top: 6px;
    font-size: 12px;
    line-height: 12px;
    color: #999;
  }
}
</style>
