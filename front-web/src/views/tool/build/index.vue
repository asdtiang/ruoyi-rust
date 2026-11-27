<template>
  <element-plus-designer v-model="drawingList"></element-plus-designer>
</template>
<script setup  name="Build">

import {ref} from "vue";
const drawingList = ref([
])
</script>

<style>
body {
  margin: 0px;
}
</style>