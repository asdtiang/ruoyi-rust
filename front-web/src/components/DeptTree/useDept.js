//针对部门的


import {deptTreeSelect} from "@/api/system/user.js";

export default function () {
    const noChild = ref(false);
    const deptTreeOption = ref([]);
    const deptIdAncestorsMap = {};
    const deptIdNameMap = ref({});
    //单位叶节点
    const deptLeaves = new Set();

    /** 查询单位下拉树结构 */
    deptTreeSelect().then(response => {
        const dt = response.data;
        setDeptIdNameMap(dt, "", 0);
        setDeptIdAncestorsMap(dt, []);
        noChild.value = !(dt[0].children && dt[0].children.length > 0)
        deptTreeOption.value = dt;
    });


    function setDeptIdAncestorsMap(deptList, ancestors) {
        for (const dept of deptList) {
            if (dept.children) {
                ancestors.push(dept.id);
                setDeptIdAncestorsMap(dept.children, ancestors);
                ancestors.pop();

            }
            const array = ancestors.concat(dept.id);
            array.shift();
            deptIdAncestorsMap[dept.id] = array;
        }
    }

    function setDeptIdNameMap(deptList, higherName, level) {
        for (const dept of deptList) {
            deptIdNameMap.value[dept.id] = level > 1 ? higherName + " / " + dept.label : dept.label;

            if (dept.children) {
                setDeptIdNameMap(dept.children, dept.label, level + 1);
            } else {
                deptLeaves.add(dept.id);
            }
        }
    }

    function deptFormat(deptId) {
        return deptIdNameMap.value[deptId];
    }

    return {noChild, deptTreeOption,deptFormat}

}
//用法
/**
 in html
 <dept-tree :dept-options="deptTreeOption" v-model="queryParams.deptId" @tree-click="treeClick"></dept-tree>
 <el-cascader v-model="queryParams.deptId" :options="deptTreeOption[0]&&deptTreeOption[0].children"
 :props="{ expandTrigger: 'hover',value:'id', checkStrictly: true, emitPath:false }"
 :show-all-levels="false"
 filterable clearable></el-cascader>
 <el-table>
 <el-table-column label="创建单位" align="center" prop="deptId" v-if="!noChild">
 <template #default="{ row }">
 {{ deptFormat(row.deptId) }}
 </template>
 </el-table-column>
</el-table>

 in js

 import useDept from "@/components/DeptTree/useDept.js";
 const {noChild, deptTreeOption, deptFormat} = useDept();

 */
