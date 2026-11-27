<template>
  <div>
    <div class="head-container">
      <el-input
        v-model="deptName"
        placeholder="请输入单位名称"
        clearable
        size="small"
        prefix-icon="el-icon-search"
        style="margin-bottom: 20px"
      />
    </div>
    <div class="head-container scrollDiv" style="height: calc(100vh - 210px)">
      <el-tree
        :data="deptOptions"
        :props="defaultProps"
        :expand-on-click-node="false"
        :filter-node-method="filterNode"
        node-key="id"
        ref="tree"
        highlight-current
        @node-click="handleNodeClick"
        :current-node-key="thisDeptId"
        :default-expanded-keys="expandedKeys"
      />
    </div>
  </div>
</template>

<script>

export default {
  name: "DeptTree",
  data() {
    return {
      deptName: undefined,
      // 单位树选项
      defaultProps: {
        children: "children",
        label: "label"
      },
      expandedKeys:[],
      thisDeptId:""
    };
  },
  props: {
    deptOptions: Array,
    showCheckbox: {
      type: Boolean,
      default: false
    },
    modelValue:{
      type: String
    }
  },
  watch: {
    // 根据名称筛选单位树
    deptName(val) {
      this.$refs.tree.filter(val);
    },
    deptOptions(val) {
      this.expandedKeys=[];
      this.expandedKeys.push(this.deptOptions[0].id)
    },
    modelValue(val){
      this.thisDeptId = val;
    }

  },
  created() {
  },
  methods: {
    filterNode(value, data) {
      if (!value) return true;
      return data.label.indexOf(value) !== -1;
    },
    // 节点单击事件
    handleNodeClick(data) {
      this.$emit("update:modelValue", data.id);//自定义事件  传递值“子向父组件传值”
      this.$emit("treeClick", data.id);//自定义事件  传递值“子向父组件传值”
    }
  }
};
</script>
