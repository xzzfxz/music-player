<template>
  <el-table
    border
    highlight-current-row
    style="width: 100%"
    :data="state.tableData"
  >
    <el-table-column prop="name" label="歌曲名" width="390" />
    <el-table-column prop="empty"></el-table-column>
    <el-table-column
      prop="singer"
      label="歌手"
      show-overflow-tooltip
      width="100"
    />
    <el-table-column
      prop="time"
      label="时间"
      align="right"
      show-overflow-tooltip
      width="80"
    />
  </el-table>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const state = reactive({
  tableData: [
    { name: '起风了', singer: '这是一个歌', time: '02:55:55' },
    { name: '吹灭小山河', singer: '这是两个歌手', time: '02:44' }
  ]
});
</script>

<style lang="scss" scoped></style>
