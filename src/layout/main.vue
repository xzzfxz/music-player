<template>
  <div class="main-layout-container flex">
    <div class="nav-container no-shrink">
      <NavBar />
    </div>
    <div class="main-router-container" v-if="reload">
      <RouterView />
    </div>
    <div class="footer-container no-shrink">
      <FootBar />
    </div>
    <MvDrawer />
  </div>
</template>

<script lang="ts" setup>
import { RouterView } from 'vue-router';
import NavBar from '@/components/navBar/index.vue';
import FootBar from '@/components/footBar/index.vue';
import { onMounted, ref, nextTick, onBeforeUnmount } from 'vue';
import emitter from '@/utils/eventHub';
import MvDrawer from '@/views/mv/components/mvDrawer.vue';

const reload = ref(true);

// 刷新当前路由
const handleReload = () => {
  reload.value = false;
  nextTick(() => {
    reload.value = true;
  });
};

// 初始化事件
const initEvent = () => {
  emitter.on('router.reload', handleReload);
};

// 取消监听事件
const offEvent = () => {
  emitter.off('router.reload', handleReload);
};

onMounted(() => {
  initEvent();
});

onBeforeUnmount(() => {
  offEvent();
});
</script>

<style lang="scss" scoped>
.main-layout-container {
  width: 100%;
  height: 100%;
  flex-direction: column;
}
.main-router-container {
  height: 0;
  flex-grow: 1;
}
.footer-container {
  width: 100%;
  height: $footBarHeight;
  position: fixed;
  left: 0;
  bottom: 0;
  z-index: 999;
}
</style>
