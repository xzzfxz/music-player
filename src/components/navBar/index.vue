<template>
  <div class="nav-bar-container flex" data-tauri-drag-region>
    <div class="nav-container-all flex" data-tauri-drag-region>
      <div class="left-content flex">
        <div class="icon-container click-active">
          <i class="ri-arrow-left-s-line"></i>
        </div>
        <div class="icon-container click-active">
          <i class="ri-arrow-right-s-line"></i>
        </div>
        <div class="icon-container click-active refresh" @click="handleRefresh">
          <i class="ri-refresh-line"></i>
        </div>
        <div class="search-container flex">
          <Search />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Search from './components/search.vue';
import emitter from '@/utils/eventHub';

// 刷新
const handleRefresh = () => {
  emitter.emit('router.reload');
};
</script>

<style lang="scss" scoped>
.nav-bar-container {
  height: 44px;
  align-items: center;
  background-color: $primaryColor;
}
.nav-container-all {
  width: 100%;
  margin-left: $menuWidth;
  justify-content: space-between;
  align-items: center;
  .left-content {
    align-items: center;
    .icon-container {
      margin-left: 4px;
      font-size: 26px;
      color: #efefef;
      cursor: pointer;
      &:first-child {
        margin-left: 0;
      }
      &.refresh {
        font-size: 22px;
      }
    }
    .search-container {
      width: 210px;
      margin-left: 14px;
      border-radius: 12px;
      align-items: center;
      overflow: hidden;
    }
  }
}
</style>
