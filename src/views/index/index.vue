<template>
  <div class="index-container flex">
    <div class="menu-container no-shrink">
      <Menu />
    </div>
    <div class="main-container"></div>
  </div>
</template>

<script setup lang="ts">
import Menu from './component/menu.vue';
</script>

<style lang="scss" scoped>
.index-container {
  height: 100%;
}
.menu-container {
  width: 190px;
  height: 100%;
  padding-top: 14px;
  background-color: #f8f8f8;
  overflow-x: hidden;
  overflow-y: auto;
}
.main-container {
  background-color: #ffffff;
  flex-grow: 1;
}
</style>
