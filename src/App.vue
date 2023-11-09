<template>
  <div class="app-all-container">
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
</script>

<style scoped lang="scss">
.app-all-container {
  width: 100vw;
  height: 100vh;
}
</style>
