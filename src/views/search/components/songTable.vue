<template>
  <el-table
    highlight-current-row
    style="width: 100%"
    header-cell-class-name="search-header-cell"
    cell-class-name="search-body-cell"
    :data="props.list"
    :border="false"
  >
    <el-table-column prop="name" label="歌曲名" width="390">
      <template #default="{ row }">
        <div class="song-container search-result-match flex">
          <div class="left-container flex">
            <div class="title ellipsis" v-html="row.SongName"></div>
            <div
              class="mv no-shrink flex-center click-active"
              v-if="row.mvdata?.length"
            >
              MV
            </div>
            <div
              class="playing no-shrink flex-center"
              v-if="curSong.id === row.ID"
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
            <div
              v-if="row.online"
              class="icon-container click-active"
              title="下载"
            >
              <i class="ri-download-2-line"></i>
            </div>
          </div>
        </div>
      </template>
    </el-table-column>
    <el-table-column
      prop="SingerName"
      label="歌手"
      show-overflow-tooltip
      width="100"
    >
      <template #default="{ row }">
        <span class="search-result-match">
          <span v-html="row.SingerName"></span>
        </span>
      </template>
    </el-table-column>
    <el-table-column
      prop="AlbumName"
      label="专辑"
      show-overflow-tooltip
      width="112"
    >
      <template #default="{ row }">
        <span class="search-result-match" v-if="row.AlbumID">
          <span v-html="row.AlbumName"></span>
        </span>
      </template>
    </el-table-column>
    <el-table-column
      prop="Duration"
      label="时间"
      :align="'right'"
      show-overflow-tooltip
      width="80"
    >
      <template #default="{ row }">
        <span>{{ getFormatPlayTime(row.Duration) }}</span>
      </template>
    </el-table-column>
  </el-table>
</template>

<script setup lang="ts">
import { SongInfo } from '@/interface';
import { PropType, computed } from 'vue';
import useMainStore from '@/store';
import { getFormatPlayTime } from '@/utils';
import { invoke } from '@tauri-apps/api';
import { EventName } from '@/const/event';
import emitter from '@/utils/eventHub';

const props = defineProps({
  list: {
    type: Array as PropType<any[]>,
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
const handlePlay = async (currentInfo: any) => {
  const params = {
    hash: currentInfo.FileHash,
    albumId: currentInfo.AlbumID,
    channel: 'KuGou'
  };
  const res: string = await invoke(EventName.GET_SONG_INFO, params);
  if (!res) {
    return;
  }
  const json = JSON.parse(res)?.data;
  const songInfo: SongInfo = {
    id: currentInfo.ID,
    singer: json.author_name,
    name: json.song_name,
    path: json.play_url,
    avatar: json.img,
    time: getFormatPlayTime(currentInfo.Duration),
    duration: currentInfo.Duration,
    mv: json.have_mv,
    album: currentInfo.AlbumID,
    online: true,
    lyrics: json.lyrics
  };
  mainStore.setCurrentSong(songInfo);
  mainStore.addPlayList([songInfo]);
  emitter.emit('music.play', true);
};

defineExpose({ handlePlay });
</script>

<style lang="scss" scoped></style>
<style lang="scss">
.el-table {
  .el-table__cell {
    &.is-leaf {
      &.search-header-cell {
        border-bottom: none;
      }
    }
  }
}
.search-result-match {
  em {
    color: $primaryColor;
  }
}
</style>
