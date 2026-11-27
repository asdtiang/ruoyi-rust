<script setup>
const props = defineProps({
  text:{
    type: String,
    default: '',
  }
})
watch(()=>props.text, (val)=>{
  type.value=""
})
const type=ref("");
function copyTextSuccess(){
  type.value="success"
}
</script>

<template>
  <el-button  text :icon="type?'Check':'DocumentCopy'" :type="type" v-copyText="text"
              v-copyText:callback="copyTextSuccess" class="ml-1 p-0"></el-button>
</template>

<style scoped lang="scss">

</style>