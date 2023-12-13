<template>
  <div class="search-container-com">
    <el-autocomplete
      ref="autoRef"
      v-model="state.filter"
      clearable
      size="small"
      placement="bottom-start"
      fit-input-width
      popper-class="history-popper"
      :trigger-on-focus="true"
      :fetch-suggestions="querySearch"
      @select="handleSelect"
      @keydown.enter.stop="handleEnter"
    >
      <template #prefix><i class="ri-search-line"></i></template>
      <template #default="{ item }">
        <div class="history-item flex">
          <div class="history-name ellipsis" :class="{ all: item.clearAll }">
            {{ item.value }}
          </div>
          <div
            v-if="item.isHistory"
            class="history-clear no-shrink click-active"
            @click.stop="deleteFromHistory(item.value)"
          >
            <i class="ri-close-fill"></i>
          </div>
        </div>
      </template>
    </el-autocomplete>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { useRouter } from 'vue-router';

interface HistoryItem {
  value: string;
  isHistory?: boolean;
  clearAll?: boolean;
}

const router = useRouter();

const HISTORY_NAME = 'searchHistory';

const autoRef = ref();

const state = reactive({
  filter: '',
  lastFilter: '',
  historyList: [] as HistoryItem[],
  suggestionList: [] as HistoryItem[]
});

const localHistoryList = computed(() => {
  if (!state.historyList.length) {
    return [];
  }
  return [...state.historyList, { value: '清空历史', clearAll: true }];
});

// 搜索
const querySearch = async (input: string, cb: any) => {
  if (!input) {
    cb(localHistoryList.value);
    return;
  }
  if (input === state.lastFilter && state.suggestionList.length) {
    cb(state.suggestionList);
    return;
  }
  // 远程搜索
  const res: string = await invoke('search_tips', { keyword: input });
  if (!res) {
    cb([]);
    return;
  }
  const json = JSON.parse(res);
  const listSet: Set<string> = new Set();
  json.data.forEach((item: any) => {
    const list = item.RecordDatas || [];
    list.forEach((it: any) => {
      listSet.add(it.HintInfo);
    });
  });
  state.suggestionList = [...listSet].map((item: string) => {
    return { value: item };
  });
  state.lastFilter = input;
  cb(state.suggestionList);
};

// 清空历史
const handleClearHistory = () => {
  state.historyList = [];
  localStorage.setItem(HISTORY_NAME, JSON.stringify(state.historyList));
};

// 删除历史记录
const deleteFromHistory = (current: string) => {
  const index = state.historyList.findIndex(
    (item: HistoryItem) => item.value === current
  );
  if (index === -1) {
    return;
  }
  state.historyList.splice(index, 1);
  localStorage.setItem(HISTORY_NAME, JSON.stringify(state.historyList));
};

// 添加历史记录
const addToHistory = (value: string) => {
  deleteFromHistory(value);
  state.historyList.unshift({ value, isHistory: true });
  localStorage.setItem(HISTORY_NAME, JSON.stringify(state.historyList));
};

// 按下enter键，保存历史
const handleEnter = () => {
  autoRef.value?.blur();
  addToHistory(state.filter);
  router.push({ path: '/searchResult', query: { keyword: state.filter } });
};

// 选中输入项
const handleSelect = (current: HistoryItem) => {
  if (current.clearAll) {
    // 清空历史
    handleClearHistory();
    return;
  }
  state.lastFilter = current.value;
  handleEnter();
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
<style lang="scss">
.history-popper {
  .history-item {
    width: 176px;
    justify-content: space-between;
    .history-name {
      flex-grow: 1;
      &.all {
        color: $primaryColor;
      }
    }
    .history-clear {
      &:hover {
        color: $primaryColor;
      }
    }
  }
}
</style>
