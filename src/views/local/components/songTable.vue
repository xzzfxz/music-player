<template>
  <el-table highlight-current-row style="width: 100%" :data="state.tableData">
    <el-table-column prop="name" label="歌曲名" width="390">
      <template #default="{ row }">
        <div class="song-container flex">
          <div class="left-container flex">
            <div class="title ellipsis">{{ row.name }}</div>
            <div class="mv no-shrink flex-center click-active" v-if="row.mv">
              MV
            </div>
            <div class="playing no-shrink flex-center">
              <i class="ri-rhythm-line"></i>
            </div>
          </div>
          <div class="right-container flex no-shrink" title="播放">
            <div class="icon-container click-active">
              <i class="ri-play-line"></i>
            </div>
            <div class="icon-container click-active">
              <i
                v-if="row.hasLiked"
                class="ri-heart-3-fill"
                title="取消喜欢"
              ></i>
              <i v-else class="ri-heart-3-line" title="喜欢"></i>
            </div>
            <div class="icon-container click-active" title="下载">
              <i class="ri-download-2-line"></i>
            </div>
            <div class="icon-container click-active" title="更多">
              <i class="ri-apps-line"></i>
            </div>
          </div>
        </div>
      </template>
    </el-table-column>
    <el-table-column prop="empty"></el-table-column>
    <el-table-column
      prop="singer"
      label="歌手"
      show-overflow-tooltip
      width="100"
    />
    <el-table-column
      prop="time"
      label="时间"
      :align="'right'"
      show-overflow-tooltip
      width="80"
    />
  </el-table>
</template>

<script setup lang="ts">
import { EventName } from '@/const/event';
import { SongInfo } from '@/interface';
import { invoke } from '@tauri-apps/api';
import { onBeforeUnmount, onMounted, reactive } from 'vue';
import { UnlistenFn, listen } from '@tauri-apps/api/event';

const state = reactive({
  tableData: [] as SongInfo[],
  reloadUnListen: undefined as unknown as UnlistenFn
});

// 添加本地音乐
const handleAddList = (payload: any) => {
  console.log(payload);
  const list = payload.payload as SongInfo[];
  state.tableData.push(...list);
};

// 初始化本地列表
const initSongList = async () => {
  let res = await invoke(EventName.GET_LOCAL_SONG_LIST);
  console.log(res);
  state.tableData = (res || []) as SongInfo[];
};
initSongList();

onMounted(async () => {
  listen(EventName.RELOAD_LOCAL_SONG_LIST, handleAddList).then((res: any) => {
    state.reloadUnListen = res;
  });
});

onBeforeUnmount(async () => {
  if (state.reloadUnListen) {
    state.reloadUnListen();
  }
});
</script>

<style lang="scss" scoped>
.song-container {
  width: 100%;
  align-items: center;
  justify-content: space-between;
  .left-container {
    width: 0;
    padding-right: 30px;
    flex-grow: 1;
    align-items: center;
    .mv {
      height: 12px;
      padding: 0 2px;
      margin-left: 4px;
      border: 1px solid $primaryColor;
      font-size: 10px;
      color: $primaryColor;
      line-height: 10px;
      cursor: pointer;
    }
    .playing {
      margin-left: 4px;
      font-size: 12px;
      color: $primaryColor;
    }
  }
  .right-container {
    align-items: center;
    .icon-container {
      margin-left: 18px;
      font-size: 16px;
      cursor: pointer;
      &:first-child {
        margin-left: 0;
        font-size: 18px;
      }
    }
  }
}
</style>
