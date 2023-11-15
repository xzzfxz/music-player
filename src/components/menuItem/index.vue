<template>
  <div class="menu-item-container">
    <div class="menu-header-container flex">
      <div class="menu-title">{{ props.data.title }}</div>
      <div class="menu-right flex">
        <div
          v-if="props.data.showFeedback"
          class="menu-feedback menu-click-item click-active flex"
        >
          <i class="ri-message-line menu-icon"></i>
          <div class="menu-text">反馈</div>
        </div>
      </div>
    </div>
    <div class="menu-child-container">
      <div
        class="menu-child-item flex"
        :class="{ selected: props.selectedItem === item.id }"
        v-for="item in childMenuList"
        :key="item.id"
        @click="handleChangeMenu(item)"
      >
        <div class="menu-child-icon">
          <i
            :class="item.icon || 'ri-file-copy-2-line'"
            :style="{ color: item.color || 'inherit' }"
          ></i>
        </div>
        <div class="menu-child-text">{{ item.name }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MenuCategory, MenuChild } from '@/interface';
import { PropType, computed } from 'vue';

const emit = defineEmits(['change']);

const props = defineProps({
  // 菜单数据
  data: {
    type: Object as PropType<MenuCategory>,
    default: () => ({})
  },
  // 当前选中项
  selectedItem: {
    type: String,
    default: 'recommend'
  }
});

// 子菜单
const childMenuList = computed(() => {
  return props.data.menuList || [];
});

// 切换菜单
const handleChangeMenu = (menuItem: MenuChild) => {
  if (menuItem.id === props.selectedItem) {
    return;
  } else {
    emit('change', menuItem);
  }
};
</script>

<style lang="scss" scoped>
.menu-header-container {
  padding-left: 20px;
  padding-right: 8px;
  align-items: center;
  justify-content: space-between;
  font-size: 12px;
  color: $menuColor;
  .menu-click-item {
    cursor: pointer;
  }
  .menu-feedback {
    align-items: center;
    .menu-icon {
      margin-right: 4px;
      font-size: 14px;
    }
  }
}
.menu-child-container {
  margin-top: 6px;
  font-size: 14px;
  .menu-child-item {
    height: 30px;
    padding-left: 20px;
    padding-right: 8px;
    align-items: center;
    cursor: pointer;
    &:hover {
      background-color: #fff;
    }
    &.selected {
      background-color: #e0e0e0;
    }
    .menu-child-icon {
      align-items: center;
      font-size: 16px;
      color: $menuColor;
    }
    .menu-child-text {
      margin-left: 12px;
    }
  }
}
</style>
