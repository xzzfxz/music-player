<template>
  <div class="menu-item-container">
    <div class="menu-header-container flex">
      <div class="menu-title">在线音乐</div>
      <div class="menu-right flex">
        <div class="feedback flex"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts"></script>

<style lang="scss" scoped>
.menu-item-container {
  padding-left: 20px;
  border: 1px solid red;
}
.menu-header-container {
  align-items: center;
  justify-content: space-between;
  font-size: 12px;
  color: $menuColor;
}
</style>
