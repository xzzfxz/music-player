<template>
  <el-table highlight-current-row style="width: 100%" :data="props.list">
    <el-table-column prop="name" label="歌曲名" width="390">
      <template #default="{ row }">
        <div class="song-container flex">
          <div class="left-container flex">
            <div class="title ellipsis">{{ row.name }}</div>
            <div class="mv no-shrink flex-center click-active" v-if="row.mv">
              MV
            </div>
            <div
              class="playing no-shrink flex-center"
              v-if="curSong.id === row.id"
            >
              <i class="ri-rhythm-line"></i>
            </div>
          </div>
          <div class="right-container flex no-shrink">
            <div
              class="icon-container click-active"
              title="播放"
              @click="handlePlay(row)"
            >
              <i class="ri-play-line"></i>
            </div>
            <div class="icon-container click-active">
              <i
                v-if="row.hasLiked"
                class="ri-heart-3-fill"
                title="取消喜欢"
              ></i>
              <i v-else class="ri-heart-3-line" title="喜欢"></i>
            </div>
            <div class="icon-container click-active" title="下载">
              <i class="ri-download-2-line"></i>
            </div>
            <div class="icon-container click-active" title="更多">
              <el-dropdown @command="handleCommand($event, row)">
                <div><i class="ri-apps-line"></i></div>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item :command="MENU_EVENT.PLAY">
                      播放
                    </el-dropdown-item>
                    <el-dropdown-item :command="MENU_EVENT.DELETE">
                      删除
                    </el-dropdown-item>
                    <el-dropdown-item :command="MENU_EVENT.DELETE_FILE">
                      删除(包含文件)
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </div>
          </div>
        </div>
      </template>
    </el-table-column>
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
      :align="'right'"
      show-overflow-tooltip
      width="80"
    />
  </el-table>
</template>

<script setup lang="ts">
import { SongInfo } from '@/interface';
import { PropType, computed } from 'vue';
import useMainStore from '@/store';
import emitter from '@/utils/eventHub';
import { MENU_EVENT } from '@/enum';
import { invoke } from '@tauri-apps/api';

const emit = defineEmits(['reloadList']);

const props = defineProps({
  list: {
    type: Array as PropType<SongInfo[]>,
    default: () => []
  }
});

const mainStore = useMainStore();

// 当前播放的歌曲
const curSong = computed(() => {
  if (mainStore.isPlaying) {
    return mainStore.getCurrentSong();
  }
  return {} as SongInfo;
});

// 播放
const handlePlay = (songInfo: SongInfo) => {
  mainStore.setCurrentSong(songInfo);
  emitter.emit('music.play', true);
};

// 更新菜单操作
const handleCommand = async (command: MENU_EVENT, songInfo: SongInfo) => {
  if (command === MENU_EVENT.PLAY) {
    // 播放
    handlePlay(songInfo);
  } else if ([MENU_EVENT.DELETE, MENU_EVENT.DELETE_FILE].includes(command)) {
    // 从列表中删除
    const res = await invoke('delete_local_song', {
      songPath: songInfo.path,
      deleteFile: MENU_EVENT.DELETE_FILE === command
    });
    emit('reloadList');
    if (res !== 'ok') {
      console.log('删除歌曲发生错误', res);
    }
  }
};

defineExpose({ handlePlay });
</script>

<style lang="scss" scoped>
.song-container {
  width: 100%;
  align-items: center;
  justify-content: space-between;
  .left-container {
    width: 0;
    padding-right: 30px;
    flex-grow: 1;
    align-items: center;
    .mv {
      height: 12px;
      padding: 0 2px;
      margin-left: 4px;
      border: 1px solid $primaryColor;
      font-size: 10px;
      color: $primaryColor;
      line-height: 10px;
      cursor: pointer;
    }
    .playing {
      margin-left: 4px;
      font-size: 12px;
      color: $primaryColor;
    }
  }
  .right-container {
    align-items: center;
    .icon-container {
      margin-left: 18px;
      font-size: 16px;
      cursor: pointer;
      &:first-child {
        margin-left: 0;
        font-size: 18px;
      }
    }
  }
}
.el-dropdown {
  display: inherit;
  font-size: inherit;
  .el-tooltip__trigger {
    outline: none;
  }
}
</style>
