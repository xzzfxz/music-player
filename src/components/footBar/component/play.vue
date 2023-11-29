<template>
  <div class="audio-container">
    <audio ref="audioRef" :src="state.src"></audio>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';

const audioRef = ref();
const state = reactive({
  src: ''
});
</script>

<style scoped lang="scss">
.audio-container {
  position: absolute;
  z-index: -1;
}
</style>
