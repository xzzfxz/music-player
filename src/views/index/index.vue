<template>
  <div class="index-container flex">
    <div class="menu-container no-shrink"></div>
    <div class="main-container"></div>
  </div>
</template>

<script setup lang="ts"></script>

<style lang="scss" scoped>
.index-container {
  height: 100%;
}
.menu-container {
  width: 190px;
  background-color: #f8f8f8;
}
.main-container {
  background-color: #ffffff;
  flex-grow: 1;
}
</style>
