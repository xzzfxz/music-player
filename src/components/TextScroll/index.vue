<template>
  <div class="horizontal-scroll" ref="containerRef">
    <div class="content" ref="contentRef">
      {{ props.text }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue';

const props = defineProps({
  text: {
    type: String,
    default: ''
  }
});

const containerRef = ref();
const contentRef = ref();
const timeOut = ref();
const waitOut = ref();
const waitTime = ref(1500);
const gapTime = ref(50);

const startScroll = () => {
  // 获取容器宽度和内容宽度
  const containerWidth = containerRef.value?.offsetWidth;
  const contentWidth = contentRef.value?.offsetWidth;

  // 设置滚动动画
  const scrollAnimation = () => {
    containerRef.value.scrollLeft += 1; // 每次滚动的距离，这里是向左滚动1个像素
    const dis = contentRef.value.offsetWidth - containerRef.value.offsetWidth;
    if (containerRef.value.scrollLeft >= dis) {
      // 滚动到最后一个字，停顿一下
      waitOut.value = setTimeout(() => {
        containerRef.value.scrollLeft = 0;
        // 回到第一个字，停顿一下
        timeOut.value = setTimeout(() => {
          scrollAnimation();
        }, waitTime.value);
      }, waitTime.value);
    } else {
      timeOut.value = setTimeout(() => {
        scrollAnimation();
      }, gapTime.value);
    }
  };
  // 开始滚动
  if (contentWidth > containerWidth) {
    scrollAnimation();
  }
};

onMounted(() => {
  timeOut.value = setTimeout(() => {
    startScroll();
  }, waitTime.value);
});

onBeforeUnmount(() => {
  if (timeOut.value) {
    clearTimeout(timeOut.value);
  }
  if (waitOut.value) {
    clearTimeout(waitOut.value);
  }
});
</script>

<style lang="scss" scoped>
.horizontal-scroll {
  overflow-x: hidden;
  white-space: nowrap;
}
.content {
  display: inline-block;
}
</style>
