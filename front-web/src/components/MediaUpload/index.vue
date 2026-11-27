<template>
  <div class="component-upload-image">
    <el-upload
        multiple
        :action="uploadImgUrl"
        list-type="picture-card"
        :on-success="handleUploadSuccess"
        :before-upload="handleBeforeUpload"
        :limit="limit"
        :on-error="handleUploadError"
        :on-exceed="handleExceed"
        ref="imageUpload"
        :show-file-list="true"
        :headers="headers"
        :file-list="fileList"
        :class="{ hide: fileList.length >= limit }"
        :accept="fileType.map(t=>'.'+t).join()"
    >
      <!-- 上传按钮 -->
      <el-icon>
        <Plus/>
      </el-icon>

      <!-- 自定义文件项：区分图片和音频 -->
      <template #file="{ file }">
        <div class="custom-file-item">
          <!-- 图片类型：显示缩略图 -->
          <img
              v-if="isImageFile(file)"
              :src="file.url"
              :alt="file.name"
              class="el-upload-list__item-thumbnail"
          />

          <!-- 音频类型：显示播放器 -->
          <div v-else-if="isAudioFile(file)" class="el-upload--picture-card">
            <el-icon>
              <Headset/>
            </el-icon>
          </div>
          <!-- 音频类型：显示播放器 -->
          <div v-else-if="isVideoFile(file)" class="el-upload--picture-card">
            <el-icon>
              <Camera/>
            </el-icon>
          </div>
          <!-- 其他文件类型 -->
          <div v-else class="el-upload--picture-card">
            <el-icon>
              <Document/>
            </el-icon>
          </div>

          <el-tooltip :content="file.name"> <span class="el-upload-list__item-actions" :title="file.name">
            <span class="el-upload-list__item-preview">
              <el-icon @click="handlePictureCardPreview(file)"><ZoomIn/></el-icon>
            </span>
            <span class="el-upload-list__item-delete">
              <el-icon @click="handleDelete(file)"><Delete/></el-icon>
            </span>
          </span></el-tooltip>

        </div>
      </template>


    </el-upload>
    <!-- 上传提示 -->
    <div class="el-upload__tip" v-if="showTip">
      请上传
      <template v-if="fileSize">
        大小不超过 <b style="color: #f56c6c">{{ fileSize }}MB</b>
      </template>
      <template v-if="fileType">
        格式为 <b style="color: #f56c6c">{{ fileType.join("/") }}</b>
      </template>
      的文件
    </div>

    <el-dialog he
               v-model="dialogVisible"
               title="预览"
               fullscreen
               append-to-body
    >
      <img v-if="isImageFile(previewFile)"
           :src="previewFile.url"
           style="display: block; max-width: 100%; margin: 0 auto; height: 90vh"
      />
      <div>
        <audio controls :src="previewFile.url" class="w-100" v-if="isAudioFile(previewFile)"></audio>
        <div>{{ previewFile.name }}</div>
      </div>
      <video controls :src="previewFile.url" class="w-100" v-if="isVideoFile(previewFile)" style="height: 90vh"></video>
    </el-dialog>
  </div>
</template>

<script setup>
import {getToken} from "@/utils/auth";
import {ZoomIn} from "@element-plus/icons-vue";

const props = defineProps({
  modelValue: [String, Object, Array],
  // 图片数量限制
  limit: {
    type: Number,
    default: 5,
  },
  // 大小限制(MB)
  fileSize: {
    type: Number,
    default: 5,
  },
  fileType: {
    type: Array,
    default: () => ["pdf", "png", "jpg", "jpeg"],
  },
  // 是否显示提示
  isShowTip: {
    type: Boolean,
    default: true
  },
});

const {proxy} = getCurrentInstance();
const emit = defineEmits();
const number = ref(0);
const uploadList = ref([]);
const previewFile = ref("");
const dialogVisible = ref(false);
const baseUrl = import.meta.env.VITE_APP_BASE_API;
const uploadImgUrl = ref(import.meta.env.VITE_APP_BASE_API + "/common/upload"); // 上传的图片服务器地址
const headers = ref({Oaauthorization: "Bearer " + getToken()});
const fileList = ref([]);
const showTip = computed(
    () => props.isShowTip && (props.fileType || props.fileSize)
);

watch(() => props.modelValue, val => {
  if (val) {
    // 首先将值转为数组
    const list = Array.isArray(val) ? val : props.modelValue.split(",");
    // 然后将数组转为对象数组
    fileList.value = list.map(item => {
      if (typeof item === "string") {
        if (item.indexOf(baseUrl) === -1) {
          item = {name: item, url: baseUrl + item};
        } else {
          item = {name: item, url: item};
        }
      }
      return item;
    });
  } else {
    fileList.value = [];
    return [];
  }
}, {deep: true, immediate: true});

// 上传前loading加载
function handleBeforeUpload(file) {
  let isImg = false;
  if (props.fileType.length) {
    let fileExtension = "";
    if (file.name.lastIndexOf(".") > -1) {
      fileExtension = file.name.slice(file.name.lastIndexOf(".") + 1);
    }
    isImg = props.fileType.some(type => {
      if (file.type.indexOf(type) > -1) return true;
      if (fileExtension && fileExtension.indexOf(type) > -1) return true;
      return false;
    });
  } else {
    isImg = file.type.indexOf("image") > -1;
  }
  if (!isImg) {
    proxy.$modal.msgError(
        `文件格式不正确, 请上传${props.fileType.join("/")}图片格式文件!`
    );
    return false;
  }
  if (props.fileSize) {
    const isLt = file.size / 1024 / 1024 < props.fileSize;
    if (!isLt) {
      proxy.$modal.msgError(`上传头像图片大小不能超过 ${props.fileSize} MB!`);
      return false;
    }
  }
  proxy.$modal.loading("正在上传图片，请稍候...");
  number.value++;
}

// 文件个数超出
function handleExceed() {
  proxy.$modal.msgError(`上传文件数量不能超过 ${props.limit} 个!`);
}

// 上传成功回调
function handleUploadSuccess(res, file) {
  proxy.$modal.closeLoading();
  if (res.code === 200) {
    uploadList.value.push({name: res.fileName, url: baseUrl + res.fileName});
    uploadedSuccessfully();
  } else {
    number.value--;
    proxy.$modal.msgError(res.msg);
    proxy.$refs.imageUpload.handleRemove(file);
    uploadedSuccessfully();
  }
}

// 删除图片
function handleDelete(file) {
  const findex = fileList.value.map(f => f.name).indexOf(file.name);
  if (findex > -1 && uploadList.value.length === number.value) {
    fileList.value.splice(findex, 1);
    let urlList = getUrlList(fileList.value);
    emit("update:modelValue", urlList);
    return false;
  }
}

// 上传结束处理
function uploadedSuccessfully() {
  if (number.value > 0 && uploadList.value.length === number.value) {
    fileList.value = fileList.value.filter(f => f.url !== undefined).concat(uploadList.value);
    uploadList.value = [];
    number.value = 0;
    emit("update:modelValue", getUrlList(fileList.value));
    proxy.$modal.closeLoading();
  }
}

// 上传失败
function handleUploadError() {
  proxy.$modal.msgError("上传图片失败");
  proxy.$modal.closeLoading();
}

// 预览
function handlePictureCardPreview(file) {
  previewFile.value = file;
  dialogVisible.value = true;
}

// 对象转成指定字符串分隔
function getUrlList(list) {
  return list.map(l => l.name);
}

// 接受的文件类型：同时允许图片和音频[citation:3][citation:10]
const acceptTypes = computed(() => {
  const imageTypes = ['image/jpeg', 'image/png', 'image/gif']
  const audioTypes = ['audio/mpeg', 'audio/wav', 'audio/ogg']
  return [...imageTypes, ...audioTypes].join(',')
})

// 判断是否为图片文件[citation:3]
const isImageFile = (file) => {
  return file.raw?.type.startsWith('image/') ||
      /\.(jpg|jpeg|png|gif)$/i.test(file.name)
}

// 判断是否为音频文件[citation:3]
const isAudioFile = (file) => {
  return file.raw?.type.startsWith('audio/') ||
      /\.(mp3|wav|ogg|m4a)$/i.test(file.name)
}
// 判断是否为音频文件[citation:3]
const isVideoFile = (file) => {
  return file.raw?.type.startsWith('video/') ||
      /\.(mp4)$/i.test(file.name)
}

</script>

<style scoped lang="scss">
// .el-upload--picture-card 控制加号部分
:deep(.hide .el-upload--picture-card) {
  display: none;
}

.file-actions {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(0, 0, 0, 0.5);
  opacity: 0;
  transition: opacity 0.3s;
}

.custom-file-item:hover .file-actions {
  opacity: 1;
}

</style>