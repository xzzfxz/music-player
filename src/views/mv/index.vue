<template>
  <div class="mv-container">
    <el-tabs
      v-model="state.activeName"
      class="demo-tabs"
      @tab-change="handleTabChange"
    >
      <el-tab-pane
        :label="item.name"
        :name="item.channel_id"
        v-for="item in state.categoryList"
        :key="item.name"
      >
        <MvItem :list="state.categoryMap.get(item.channel_id)" />
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { EventName } from '@/const/event';
import { CHANNEL_TYPE } from '@/enum';
import { invoke } from '@tauri-apps/api';
import { reactive } from 'vue';
import MvItem from './components/mvItem.vue';

const state = reactive({
  activeName: '',
  categoryList: [] as any[],
  mvParams: {
    page: 1,
    size: 20,
    id: 9,
    sort: 5,
    short: 0,
    channel: CHANNEL_TYPE.KU_GOU
  },
  categoryMap: new Map() as Map<string, any>
});

// 切换类型
const handleTabChange = () => {
  const list = state.categoryMap.get(state.activeName) || [];
  if (list.length) {
    return;
  }
  const currentCategory = state.categoryList.find(
    (item: any) => item.channel_id === state.activeName
  );
  if (!currentCategory) {
    return;
  }
  state.mvParams.page = 1;
  state.mvParams.id = currentCategory.channel_id;
  state.mvParams.short = currentCategory.is_short;
  state.mvParams.sort = currentCategory.sort_way;
  getMvList(state.activeName);
};

// 根据分类获取mv列表
const getMvList = (activeName: string) => {
  invoke(EventName.GET_MV_LIST, state.mvParams).then((res: any) => {
    if (!res) {
      state.categoryMap.set(activeName, []);
      return;
    }
    const json = JSON.parse(res);
    const list = json.data.info || [];
    state.categoryMap.set(activeName, list);
  });
};

// 获取分类列表
const getCategoryList = () => {
  invoke(EventName.GET_MV_CATEGORY, { channel: CHANNEL_TYPE.KU_GOU }).then(
    (res: any) => {
      if (!res) {
        return;
      }
      const json = JSON.parse(res);
      state.categoryList = json.data?.list || [];
      if (state.categoryList.length) {
        const firstItem = state.categoryList[0];
        state.activeName = firstItem.channel_id;
        state.mvParams.page = 1;
        state.mvParams.id = firstItem.channel_id;
        state.mvParams.short = firstItem.is_short;
        state.mvParams.sort = firstItem.sort_way;
        getMvList(firstItem.channel_id);
      }
    }
  );
};
getCategoryList();
</script>

<style scoped lang="scss">
.mv-container {
  padding: 20px 30px;
}
</style>
