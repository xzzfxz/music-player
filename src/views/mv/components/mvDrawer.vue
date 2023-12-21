<template>
  <el-drawer
    v-model="state.showDrawer"
    direction="btt"
    :show-close="true"
    size="100%"
  >
    <template #header>
      <h4>set title by slot</h4>
    </template>
    <template #default>
      <div>sdjflksdjfl</div>
    </template>
  </el-drawer>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const state = reactive({
  showDrawer: false
});
</script>

<style lang="scss" scoped></style>
