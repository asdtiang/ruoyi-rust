<template>
  <div class="d-flex flex-wrap">
    <div v-for="file in thisFileList" class="file">
      <div  :title="file">
      <el-image v-if="isImageFile(file)"
                :src="`${file}`" fit="contain" :preview-src-list="imageFileList"
                :initial-index="imageFileList.indexOf(file)"
                :style="mediaStyle" show-progress preview-teleported z-index="3000"/>
      <audio controls :src="file" v-else-if="isAudioFile(file)" :style="mediaStyle2"></audio>
      <video controls :src="file" v-else-if="isVideoFile(file)" :style="mediaStyle2"></video>
      <el-icon :style="mediaStyle" v-else>
        <Document></Document>
      </el-icon>
      </div>
      <el-link icon="Download" :href="file" title="下载" download></el-link>
    </div>
  </div>


</template>
<script setup>
import {computed} from "vue";
import {Document} from "@element-plus/icons-vue";

const baseUrl = import.meta.env.VITE_APP_BASE_API;
const props = defineProps({
  fileList: {
    type: Array,
    required: true,
    default: () => []
  },
  width: {
    type: Number,
    default: 100
  },
  height: {
    type: Number,
    default: 100
  },
})
const thisFileList = computed(() => props.fileList.map(fileName => baseUrl + fileName))
const imageFileList = computed(() => props.fileList.filter(fileName => isImageFile(fileName)).map(fileName => baseUrl + fileName))
// 判断是否为图片文件
const isImageFile = (file) => {
  return /\.(jpg|jpeg|png|gif)$/i.test(file)
}

// 判断是否为音频文件
const isAudioFile = (file) => {
  return /\.(mp3|wav|ogg|m4a|amr)$/i.test(file)
}
// 判断是否为音频文件
const isVideoFile = (file) => {
  return /\.(mp4|mov|avi|mkv|flv)$/i.test(file)
}
const mediaStyle = computed(() => {
  return {width: props.width + 'px', height: props.height + 'px'}
})
const mediaStyle2 = computed(() => {
  return {width: props.width*2 + 'px', height: props.height + 'px'}
})

</script>


<style scoped lang="scss">
.file {
  padding: 5px;

  &:hover {
    box-shadow: inset 0px 0px 0px 2px gray;
    border-radius: 5px;
  }
}

</style>