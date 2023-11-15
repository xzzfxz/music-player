<template>
  <div class="sound-quality-container">
    <el-dropdown
      placement="top"
      trigger="click"
      popper-class="sound-popper-container"
      :hide-on-click="false"
    >
      <div class="down-container flex-center">
        无损
        <i class="ri-arrow-up-s-line icon"></i>
      </div>
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item>
            <el-radio label="1" size="small" v-model="state.soundQuality">
              标准音质
            </el-radio>
          </el-dropdown-item>
          <el-dropdown-item>
            <el-radio label="2" size="small" v-model="state.soundQuality">
              <template #default>
                <div class="flex quality-container">
                  <span>高品音质</span>
                  <span class="quality-mark high">HQ</span>
                </div>
              </template>
            </el-radio>
          </el-dropdown-item>
          <el-dropdown-item>
            <el-radio label="3" size="small" v-model="state.soundQuality">
              <template #default>
                <div class="flex quality-container">
                  <span>无损音质</span>
                  <span class="quality-mark super">SQ</span>
                </div>
              </template>
            </el-radio>
          </el-dropdown-item>
          <el-dropdown-item divided>
            <el-checkbox
              v-model="state.priority"
              label="优先听选中音质"
              size="small"
            />
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

const state = reactive({
  soundQuality: '1',
  priority: true
});
</script>

<style lang="scss" scoped>
.sound-quality-container {
  line-height: normal;
}
.down-container {
  width: 50px;
  height: 20px;
  border: 1px solid #fff;
  border-radius: 10px;
  color: #fff;
  font-size: 12px;
  line-height: 12px;
  outline: none;
  cursor: pointer;
  .icon {
    font-size: 14px;
  }
}
</style>
<style lang="scss">
.sound-popper-container {
  .quality-container {
    align-items: center;
    .quality-mark {
      width: 14px;
      margin-left: 8px;
      background-color: #6bbe2e;
      color: #fff;
      font-size: 8px;
      line-height: 9px;
      text-align: center;
      &.super {
        background-color: #9b6ef9;
      }
    }
  }
  .el-dropdown-menu {
    padding-bottom: 0;
    .el-dropdown-menu__item--divided {
      margin: 0;
    }
  }
}
</style>
