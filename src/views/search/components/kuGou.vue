<template>
  <SongTable :list="state.list" />
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import SongTable from './songTable.vue';
import { invoke } from '@tauri-apps/api';
import { CHANNEL_TYPE } from '@/enum';
import { EventName } from '@/const/event';

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
  invoke(EventName.SEARCH_SONGS, {
    keyword: props.keyword,
    channel: CHANNEL_TYPE.KU_GOU
  }).then((res: any) => {
    if (!res) {
      return;
    }
    const json = JSON.parse(res);
    console.log(json);
    state.list = json.data.lists || [];
  });
};

getTableData();
</script>

<style lang="scss" scoped></style>
