<template>
  <div class="title-container flex">
    <div class="title">本地音乐</div>
    <div class="total">({{ state.tableData.length }}首)</div>
  </div>
  <div class="deal-container flex">
    <div class="left-container flex">
      <el-button type="primary" class="deal-btn" @click="handlePlayAll">
        <i class="ri-play-line deal-icon"></i>
        播放全部
      </el-button>
      <el-button class="deal-btn" @click="handleShowAddDialog">
        <i class="ri-add-line deal-icon"></i>
        添加歌曲
      </el-button>
    </div>
    <div class="right-container flex">
      <div class="input-container">
        <el-input
          v-model="state.filterValue"
          placeholder="搜索"
          class="filter-input"
          size="small"
        >
          <template #prefix><i class="ri-search-line"></i></template>
        </el-input>
      </div>
    </div>
  </div>
  <div class="song-container">
    <SongTable ref="songTableRef" :list="songList" @reloadList="initSongList" />
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue';
import SongTable from './components/songTable.vue';
import { MUSIC_EXT } from '@/const';
import { invoke } from '@tauri-apps/api';

import { EventName } from '@/const/event';
import { SongInfo } from '@/interface';
import { UnlistenFn, listen } from '@tauri-apps/api/event';
import useMainStore from '@/store';

const mainStore = useMainStore();

const songTableRef = ref();

const state = reactive({
  filterValue: '',
  tableData: [] as SongInfo[],
  reloadUnListen: undefined as unknown as UnlistenFn
});

// 过滤后的列表
const songList = computed(() => {
  if (!state.filterValue) {
    return state.tableData;
  }
  return state.tableData.filter((song: SongInfo) => {
    return (
      song.name?.includes(state.filterValue) ||
      song.singer?.includes(state.filterValue)
    );
  });
});

// 播放全部
const handlePlayAll = () => {
  mainStore.setPlayList(songList.value);
  songTableRef.value?.handlePlay(songList.value[0]);
};

// 显示添加音乐弹窗
const handleShowAddDialog = async () => {
  await invoke(EventName.OPEN_SONG_DIALOG, {
    fileType: 'music',
    extensions: MUSIC_EXT
  });
};

// 初始化本地列表
const initSongList = async () => {
  let res = await invoke(EventName.GET_LOCAL_SONG_LIST);
  state.tableData = (res || []) as SongInfo[];
};
initSongList();

onMounted(() => {
  listen(EventName.RELOAD_LOCAL_SONG_LIST, initSongList).then((res: any) => {
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
.title-container {
  padding-top: 25px;
  padding-left: 40px;
  align-items: flex-end;
  .title {
    font-size: 20px;
    font-weight: 400;
    line-height: 20px;
  }
  .total {
    margin-left: 8px;
    font-size: 14px;
    line-height: 14px;
    color: $menuColor;
  }
}
.deal-container {
  padding-left: 40px;
  padding-right: 40px;
  margin-top: 20px;
  align-items: center;
  justify-content: space-between;
  .deal-btn {
    width: 100px;
    .deal-icon {
      font-size: 20px;
    }
  }
  .filter-input {
    width: 130px;
    :deep(.el-input__inner) {
      box-shadow: none;
    }
  }
}
.song-container {
  padding: 15px 40px;
}
</style>
