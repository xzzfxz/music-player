<template>
  <SongTable :list="state.list" />
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import SongTable from './songTable.vue';
import { invoke } from '@tauri-apps/api';

const props = defineProps({
  keyword: {
    type: String,
    default: ''
  }
});

const state = reactive({
  list: [] as any[]
});

// 查询列表
const getTableData = () => {
  invoke('search_songs', { keyword: props.keyword, channel: 'KuGou' }).then(
    (res: any) => {
      if (!res) {
        return;
      }
      const json = JSON.parse(res);
      state.list = json.data.lists || [];
    }
  );
};

getTableData();
</script>

<style lang="scss" scoped></style>
