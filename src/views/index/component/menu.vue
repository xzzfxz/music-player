<template>
  <div class="menu-category">
    <MenuItem
      :data="state.onlineMenu"
      :selectedItem="state.selectedItem"
      @change="handleMenuChange"
    />
  </div>
  <div class="menu-category">
    <MenuItem
      :data="state.myMusicMenu"
      :selectedItem="state.selectedItem"
      @change="handleMenuChange"
    />
  </div>
  <div class="menu-category">
    <MenuItem
      :data="state.selfCreateMenu"
      :selectedItem="state.selectedItem"
      @change="handleMenuChange"
    />
  </div>
  <div class="menu-category">
    <MenuItem
      :data="state.collectMenu"
      :selectedItem="state.selectedItem"
      @change="handleMenuChange"
    />
  </div>
</template>

<script setup lang="ts">
import MenuItem from '@/components/menuItem/index.vue';
import { MenuCategory, MenuChild } from '@/interface';
import { shallowReactive } from 'vue';

const state = shallowReactive({
  selectedItem: 'local',
  onlineMenu: {
    title: '在线音乐',
    showFeedback: true,
    menuList: [
      { id: 'recommend', name: '推荐', icon: 'ri-disc-line' },
      { id: 'radio', name: '电台', icon: 'ri-broadcast-fill' },
      { id: 'songList', name: '歌单', icon: 'ri-album-line' },
      { id: 'rankingList', name: '排行榜', icon: 'ri-bar-chart-2-line' },
      { id: 'singer', name: '歌手', icon: 'ri-user-voice-line' },
      { id: 'mv', name: 'MV', icon: 'ri-movie-line' }
    ]
  } as MenuCategory,
  myMusicMenu: {
    title: '我的音乐',
    menuList: [
      {
        id: 'privateCloudDisk',
        name: '私人音乐云盘',
        icon: 'ri-cloudy-2-line'
      },
      { id: 'download', name: '下载管理', icon: 'ri-download-2-line' },
      { id: 'iTunes', name: 'iTunes音乐', icon: 'ri-music-2-line' },
      { id: 'local', name: '本地音乐', icon: 'ri-computer-line' }
    ]
  } as MenuCategory,
  selfCreateMenu: {
    title: '我喜欢',
    menuList: [
      { id: 'like', name: '我喜欢', icon: 'ri-heart-3-line', color: 'red' },
      { id: 'defaultCollect', name: '默认收藏', icon: 'ri-file-copy-2-line' },
      { id: 'defaultList', name: '默认列表' }
    ]
  } as MenuCategory,
  collectMenu: {
    title: '收藏歌单',
    menuList: [
      { id: 'collect1', name: '收藏1' },
      { id: 'collect2', name: '收藏2' }
    ]
  } as MenuCategory
});

// 菜单改变
const handleMenuChange = (menuItem: MenuChild) => {
  state.selectedItem = menuItem.id;
};
</script>

<style lang="scss" scoped>
.menu-category {
  margin-bottom: 24px;
}
</style>
