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
import { onBeforeUnmount, onMounted, reactive } from 'vue';
import SongTable from './components/songTable.vue';
import { MUSIC_EXT } from '@/const';
// import { ElMessage } from 'element-plus';
// import { getSingerAndName } from '@/utils';
// import { SongInfo } from '@/interface';
import { invoke } from '@tauri-apps/api';
import { UnlistenFn, listen } from '@tauri-apps/api/event';

const state = reactive({
  filterValue: '',
  reloadUnListen: undefined as unknown as UnlistenFn
});

// 显示添加音乐弹窗
const handleShowAddDialog = async () => {
  await invoke('open_song_dialog', {
    fileType: 'music',
    extensions: MUSIC_EXT
  });
};

onMounted(async () => {
  state.reloadUnListen = await listen('reloadLocalSongList', (info: any) => {
    console.log('这是后端传来的事件', info);
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
