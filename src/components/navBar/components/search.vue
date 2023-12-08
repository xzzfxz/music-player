<template>
  <div class="search-container-com">
    <el-dropdown trigger="click" placement="bottom-start">
      <div>
        <el-autocomplete
          v-model="state.filter"
          clearable
          size="small"
          :fetch-suggestions="querySearch"
          @select="handleSelect"
          @keydown.enter.stop="handleEnter"
        >
          <template #prefix><i class="ri-search-line"></i></template>
        </el-autocomplete>
      </div>
      <template #dropdown v-if="!state.filter && state.historyList.length">
        <el-dropdown-menu>
          <el-dropdown-item
            v-for="(item, index) in state.historyList"
            :key="item"
          >
            <div class="history-item" @click="deleteFromHistory(index)">
              Action 1
            </div>
          </el-dropdown-item>
          <el-dropdown-item divided>
            <div @click="handleClearHistory">清空历史</div>
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

interface HistoryItem {
  value: string;
}

const HISTORY_NAME = 'searchHistory';

const state = reactive({
  filter: '',
  historyList: [] as HistoryItem[]
});

// 搜索
const querySearch = (input: string, cb: any) => {
  if (!input) {
    cb([]);
  } else {
    // 远程搜索
    cb([]);
  }
};

// 清空历史
const handleClearHistory = () => {
  state.historyList = [];
  localStorage.setItem(HISTORY_NAME, JSON.stringify(state.historyList));
};

// 删除历史记录
const deleteFromHistory = (index: number) => {
  state.historyList.splice(index, 1);
  localStorage.setItem(HISTORY_NAME, JSON.stringify(state.historyList));
};

// 添加历史记录
const addToHistory = (value: string) => {
  const index = state.historyList.findIndex(
    (item: HistoryItem) => item.value === value
  );
  if (index !== -1) {
    // 先删除
    state.historyList.splice(index, 1);
  }
  state.historyList.unshift({ value });
  localStorage.setItem(HISTORY_NAME, JSON.stringify(state.historyList));
};

// 按下enter键，保存历史
const handleEnter = () => {
  addToHistory(state.filter);
};

// 选中输入项
const handleSelect = (current: any) => {
  console.log(current);
  addToHistory(state.filter);
};

// 进入页面时，初始化搜索历史
const initHistoryList = () => {
  const list = localStorage.getItem(HISTORY_NAME) || '[]';
  return JSON.parse(list);
};

// 初始化数据
const initData = () => {
  state.historyList = initHistoryList();
};
initData();
</script>

<style lang="scss" scoped>
.search-container-com {
  width: 100%;
  height: 24px;
  :deep(.el-autocomplete) {
    width: 100%;
    outline: none;
    overflow: hidden;
    .el-input__wrapper {
      padding: 0 4px;
      box-shadow: none;
      &:hover {
        box-shadow: none;
      }
      .el-input__prefix-inner {
        padding-left: 4px;
        font-size: 16px;
      }
      .el-input__inner {
        border-radius: 0;
        box-shadow: none;
        outline: none;
      }
    }
  }
}
</style>
