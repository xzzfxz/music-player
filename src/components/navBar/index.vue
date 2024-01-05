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
      <div class="right-content flex">
        <div class="avatar" @click="handleShowLogin">
          <i class="ri-user-line avatar-icon"></i>
        </div>
        <div class="user-text click-active" @click="handleShowLogin">登录</div>
        <div class="setting-icon"><i class="ri-settings-2-line"></i></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { createWindow } from '@/utils/window';
import Search from './components/search.vue';
import emitter from '@/utils/eventHub';
import { WINDOW_NAME } from '@/enum';

// 刷新
const handleRefresh = () => {
  emitter.emit('router.reload');
};

// 显示登录
const handleShowLogin = () => {
  createWindow(WINDOW_NAME.LOGIN, {
    url: '/login'
  });
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
  .right-content {
    padding-right: 20px;
    align-items: center;
    color: #efefef;
    .avatar {
      width: 26px;
      height: 26px;
      margin-right: 8px;
      border-radius: 50%;
      background-color: #bcdcff;
      text-align: center;
      cursor: pointer;
      .avatar-icon {
        font-size: 18px;
      }
    }
    .user-text {
      margin-right: 14px;
      font-size: 14px;
      cursor: pointer;
    }
    .setting-icon {
      font-size: 18px;
      cursor: pointer;
    }
  }
}
</style>
