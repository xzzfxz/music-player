<template>
  <div class="mv-list-container flex">
    <div
      class="mv-item-container flex"
      v-for="item in props.list"
      :key="item.videoid"
      @click="handleToPlayMv(item)"
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
import { invoke } from '@tauri-apps/api';
import { EventName } from '@/const/event';
import { CHANNEL_TYPE } from '@/enum';
import { MvInfo } from '@/interface/event';
import { MV_QUALITY } from '@/const';
import emitter from '@/utils/eventHub';
import { ElMessage } from 'element-plus';

const props = defineProps({
  list: {
    type: Array as PropType<any>,
    default: () => []
  }
});

// 播放mv
const handleToPlayMv = (current: any) => {
  invoke(EventName.GET_MV_DETAIL, {
    hash: current.mvhash,
    channel: CHANNEL_TYPE.KU_GOU
  }).then((res: any) => {
    if (!res) {
      return;
    }
    const json = JSON.parse(res);
    const mvData = json.mvdata || {};
    const mvList: MvInfo[] = [];
    MV_QUALITY.forEach((item: string) => {
      const mv = mvData[item] || {};
      if (mv.downurl) {
        mvList.push({
          singer: json.singer,
          name: json.songname,
          type: item,
          url: mv.downurl,
          backUrl: mv.backupdownurl,
          duration: mv.timelength,
          bitRate: mv.bitrate
        });
      }
    });
    if (!mvList.length) {
      ElMessage.error('暂未有相关信息');
      return;
    }
    emitter.emit('mv.play', mvList);
  });
};
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
