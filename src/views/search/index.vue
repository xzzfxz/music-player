<template>
  <div class="title-container">
    搜索“
    <span class="keyword">{{ state.keyword }}</span>
    ”的想着歌曲
  </div>
  <div class="tab-container">
    <el-tabs v-model="state.activeName">
      <el-tab-pane label="酷狗" :name="CHANNEL_TYPE.KU_GOU">
        <KuGou :keyword="state.keyword" v-if="state.keyword" />
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';
import { useRoute } from 'vue-router';
import KuGou from './components/kuGou.vue';
import { CHANNEL_TYPE } from '@/enum';

const route = useRoute();

const state = reactive({
  keyword: '',
  activeName: CHANNEL_TYPE.KU_GOU
});

// 初始化查询数据
const initQuery = () => {
  const query = route.query || {};
  state.keyword = (query.keyword as string) || '';
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
