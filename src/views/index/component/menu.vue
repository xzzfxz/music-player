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
import {
  Headset,
  Box,
  VideoPlay,
  Monitor,
  Histogram,
  User,
  MostlyCloudy,
  Download,
  Pear,
  Files,
  Star,
  Document
} from '@element-plus/icons-vue';
import { shallowReactive } from 'vue';

const state = shallowReactive({
  selectedItem: 'recommend',
  onlineMenu: {
    title: '在线音乐',
    showFeedback: true,
    menuList: [
      { id: 'recommend', name: '推荐', iconCom: Headset },
      { id: 'radio', name: '电台', iconCom: Box },
      { id: 'songList', name: '歌单', iconCom: Files },
      { id: 'rankingList', name: '排行榜', iconCom: Histogram },
      { id: 'singer', name: '歌手', iconCom: User },
      { id: 'mv', name: 'MV', iconCom: VideoPlay }
    ]
  } as MenuCategory,
  myMusicMenu: {
    title: '我的音乐',
    menuList: [
      { id: 'privateCloudDisk', name: '私人音乐云盘', iconCom: MostlyCloudy },
      { id: 'download', name: '下载管理', iconCom: Download },
      { id: 'iTunes', name: 'iTunes音乐', iconCom: Pear },
      { id: 'local', name: '本地音乐', iconCom: Monitor }
    ]
  } as MenuCategory,
  selfCreateMenu: {
    title: '我喜欢',
    menuList: [
      { id: 'like', name: '我喜欢', iconCom: Star, color: 'red' },
      { id: 'defaultCollect', name: '默认收藏', iconCom: Document },
      { id: 'defaultList', name: '默认列表', iconCom: Document }
    ]
  } as MenuCategory,
  collectMenu: {
    title: '收藏歌单',
    menuList: [
      { id: 'collect1', name: '收藏1', iconCom: Document },
      { id: 'collect2', name: '收藏2', iconCom: Document }
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
