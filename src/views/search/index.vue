<template>
  <div class="title-container">
    搜索“
    <span class="keyword">{{ state.keyword }}</span>
    ”的想着歌曲
  </div>
  <div class="tab-container">
    <el-tabs v-model="state.activeName">
      <el-tab-pane label="单曲" name="song">
        <SongTable />
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import { useRoute } from 'vue-router';
import SongTable from './components/songTable.vue';
import { invoke } from '@tauri-apps/api';

const route = useRoute();

const state = reactive({
  keyword: '',
  activeName: 'song'
});

// 查询列表
const getTableData = () => {
  invoke('search_song', { keyword: state.keyword });
};

// 初始化查询数据
const initQuery = () => {
  const query = route.query || {};
  state.keyword = (query.keyword as string) || '';
  getTableData();
};
initQuery();
</script>

<style lang="scss" scoped>
.title-container {
  padding: 20px 40px;
  font-size: 21px;
  .keyword {
    color: $primaryColor;
  }
}
.tab-container {
  padding: 0 40px;
}
</style>
