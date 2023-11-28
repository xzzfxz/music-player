<template>
  <div class="index-container flex">
    <div class="menu-container no-shrink">
      <Menu />
    </div>
    <div class="main-container">
      <RouterView />
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
import Menu from './component/menu.vue';
</script>

<style lang="scss" scoped>
.index-container {
  height: 100%;
}
.menu-container {
  width: $menuWidth;
  height: calc(100% - $footBarHeight);
  padding-top: 14px;
  background-color: #f8f8f8;
  overflow-x: hidden;
  overflow-y: auto;
}
.main-container {
  height: 100%;
  width: 0;
  padding-bottom: calc($footBarHeight + 10px);
  background-color: #ffffff;
  flex-grow: 1;
  overflow: auto;
}
</style>
