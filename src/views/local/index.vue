<template>
  <div class="title-container flex">
    <div class="title">本地音乐</div>
    <div class="total">(2首)</div>
  </div>
  <div class="deal-container flex">
    <div class="left-container flex">
      <el-button type="primary" class="deal-btn">
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
    <SongTable />
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import SongTable from './components/songTable.vue';
import { open } from '@tauri-apps/plugin-dialog';
import { MUSIC_EXT } from '@/const';
import { ElMessage } from 'element-plus';
import { getSingerAndName } from '@/utils';
import { SongInfo } from '@/interface';
import { invoke } from '@tauri-apps/api/primitives';

const state = reactive({
  filterValue: ''
});

// 显示添加音乐弹窗
const handleShowAddDialog = async () => {
  const selected = await open({
    title: '选择音乐',
    multiple: true,
    recursive: false,
    filters: [
      {
        name: 'music',
        extensions: MUSIC_EXT
      }
    ]
  });
  console.log(selected);
  if (!selected?.length) {
    // 没有选中文件
    ElMessage({
      message: '没有选中文件',
      type: 'warning'
    });
    return;
  }
  const songList: SongInfo[] = [];
  selected.forEach((song: any) => {
    const res = getSingerAndName(song.name);
    songList.push({ singer: res.singer, name: res.songName, path: song.path });
  });
  console.log(songList);
  await invoke('save_local_song_info', { test: [1, 2, 3] });
};
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
