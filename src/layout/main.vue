<template>
  <div class="main-layout-container flex">
    <div class="nav-container no-shrink">
      <NavBar />
    </div>
    <div class="main-router-container">
      <RouterView />
    </div>
    <div class="footer-container no-shrink">
      <FootBar />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { RouterView } from 'vue-router';
import NavBar from '@/components/navBar/index.vue';
import FootBar from '@/components/footBar/index.vue';
</script>

<style lang="scss" scoped>
.main-layout-container {
  width: 100%;
  height: 100%;
  flex-direction: column;
}
.main-router-container {
  flex-grow: 1;
}
</style>
