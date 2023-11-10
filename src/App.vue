<template>
  <div class="app-all-container">
    <el-config-provider :locale="zhCn">
      <RouterView />
    </el-config-provider>
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router';
import zhCn from 'element-plus/es/locale/lang/zh-cn';
</script>

<style scoped lang="scss">
.app-all-container {
  width: 100vw;
  height: 100vh;
  user-select: none;
}
</style>
