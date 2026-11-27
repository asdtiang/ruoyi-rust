<template>
  <el-form ref="genInfoForm" :model="info" :rules="rules" label-width="150px">
    <el-row>
      <el-col :span="12">
        <el-form-item prop="tplCategory">
          <template #label>生成模板</template>
          <el-select v-model="info.tplCategory" @change="tplSelectChange">
            <el-option label="单表（增删改查）" value="crud" />
            <el-option label="树表（增删改查）" value="tree" />
            <el-option label="一对一（增删改查）" value="o2o" />
            <el-option label="一对多（增删改查）" value="o2m" />
            <el-option label="子表（增删改查）" value="subTable" />
            <el-option label="多对多（增删改查）" value="m2m" />
            <el-option label="连接表（增删）" value="join" />
          </el-select>
        </el-form-item>
      </el-col>

      <el-col :span="12">
        <el-form-item prop="tplWebType">
          <template #label>前端类型</template>
          <el-select v-model="info.tplWebType">
            <el-option label="Vue2 Element UI 模版(未实现)" value="element-ui" />
            <el-option label="Vue3 Element Plus 模版" value="element-plus" />
          </el-select>
        </el-form-item>
      </el-col>
      <el-col :span="12">
        <el-form-item prop="tplWebType">
          <template #label>后台端类型</template>
          <el-select v-model="info.tplBackType">
            <el-option label="rust" value="rust" />
            <el-option label="java未实现" value="java" />
          </el-select>
        </el-form-item>
      </el-col>

      <el-col :span="12" v-if="info.tplBackType==='java'">
        <el-form-item prop="packageName" >
          <template #label>
            生成包路径
            <el-tooltip content="生成在哪个java包下，例如 com.ruoyi.system" placement="top">
              <el-icon><question-filled /></el-icon>
            </el-tooltip>
          </template>
          <el-input v-model="info.packageName" />
        </el-form-item>
      </el-col>

      <el-col :span="12">
        <el-form-item prop="moduleName">
          <template #label>
            生成模块名
            <el-tooltip content="可理解为子系统名，例如 system" placement="top">
              <el-icon><question-filled /></el-icon>
            </el-tooltip>
          </template>
          <el-input v-model="info.moduleName" />
        </el-form-item>
      </el-col>

      <el-col :span="12">
        <el-form-item prop="businessName">
          <template #label>
            生成业务名
            <el-tooltip content="可理解为功能英文名，例如 user" placement="top">
              <el-icon><question-filled /></el-icon>
            </el-tooltip>
          </template>
          <el-input v-model="info.businessName" />
        </el-form-item>
      </el-col>

      <el-col :span="12">
        <el-form-item prop="functionName">
          <template #label>
            生成功能名
            <el-tooltip content="用作类描述，例如 用户" placement="top">
              <el-icon><question-filled /></el-icon>
            </el-tooltip>
          </template>
          <el-input v-model="info.functionName" />
        </el-form-item>
      </el-col>

      <el-col :span="12">
        <el-form-item prop="genType">
          <template #label>
            生成代码方式
            <el-tooltip content="默认为zip压缩包下载，也可以自定义生成路径" placement="top">
              <el-icon><question-filled /></el-icon>
            </el-tooltip>
          </template>
          <el-radio v-model="info.genType" label="0">zip压缩包</el-radio>
          <el-radio v-model="info.genType" label="1">自定义路径</el-radio>
        </el-form-item>
      </el-col>

      <el-col :span="12" v-if="info&&info.options">
        <el-form-item>
          <template #label>
            上级菜单
            <el-tooltip content="分配到指定菜单下，例如 系统管理" placement="top">
              <el-icon><question-filled /></el-icon>
            </el-tooltip>
          </template>
          <el-tree-select :check-strictly=true
                          v-model="info.options.parentMenuId"
                          :data="menuOptions"
                          :render-after-expand="false"
                          style="width: 240px"
                          placeholder="请选择系统菜单"
                          :props="{ value: 'menuId', label: 'menuName', children: 'children' }"
          />
        </el-form-item>
      </el-col>

      <el-col :span="24" v-if="info.genType === '1'">
        <el-form-item prop="genPathBack">
          <template #label>
            后台自定义路径
            <el-tooltip content="填写磁盘绝对路径，若不填写，则生成到当前Web项目下" placement="top">
              <el-icon><question-filled /></el-icon>
            </el-tooltip>
          </template>
          <el-input v-model="info.genPathBack">
            <template #append>
              <el-dropdown>
                <el-button type="primary">
                  最近路径快速选择
                  <i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="info.genPathBack = '/'">恢复默认的生成基础路径</el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item prop="genPathWeb">
          <template #label>
            前台自定义路径
            <el-tooltip content="填写磁盘绝对路径，若不填写，则生成到当前Web项目下" placement="top">
              <el-icon><question-filled /></el-icon>
            </el-tooltip>
          </template>
          <el-input v-model="info.genPathWeb">
            <template #append>
              <el-dropdown>
                <el-button type="primary">
                  最近路径快速选择
                  <i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="info.genPathWeb = '/'">恢复默认的生成基础路径</el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </template>
          </el-input>
        </el-form-item>
      </el-col>
    </el-row>

    <template v-if="info.tplCategory === 'tree'">
      <h4 class="form-header">其他信息</h4>
      <el-row v-show="info.tplCategory === 'tree'">
        <el-col :span="12">
          <el-form-item>
            <template #label>
              树编码字段
              <el-tooltip content="树显示的编码字段名， 如：dept_id" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.treeCode" placeholder="请选择">
              <el-option
                  v-for="(column, index) in info.columns"
                  :key="index"
                  :label="column.columnName + '：' + column.columnComment"
                  :value="column.columnName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item>
            <template #label>
              树父编码字段
              <el-tooltip content="树显示的父编码字段名， 如：parent_Id" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.treeParentCode" placeholder="请选择">
              <el-option
                  v-for="(column, index) in info.columns"
                  :key="index"
                  :label="column.columnName + '：' + column.columnComment"
                  :value="column.columnName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item>
            <template #label>
              树名称字段
              <el-tooltip content="树节点的显示名称字段名， 如：dept_name" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.treeName" placeholder="请选择">
              <el-option
                  v-for="(column, index) in info.columns"
                  :key="index"
                  :label="column.columnName + '：' + column.columnComment"
                  :value="column.columnName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>
      </el-row>
    </template>



    <template v-if="['o2m','m2m','o2o','subTable'].indexOf(info.tplCategory)>=0">
      <h4 class="form-header">关联信息</h4>
      <el-row>
        <el-col :span="8" v-if="info.tplCategory==='m2m'">
          <el-form-item>
            <template #label>
              连接表
              <el-tooltip content="连接表， 假设主表为sys_user，子表为sys_post（下同），如：sys_user_post" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.joinTableName" placeholder="请选择" @change="subSelectChange">
              <el-option
                  v-for="(table, index) in tables"
                  :key="index"
                  :label="table.tableName + '：' + table.tableComment"
                  :value="table.tableName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="8" v-if="info.tplCategory==='m2m'">
          <el-form-item>
            <template #label>
              关联主表键名
              <el-tooltip content="连接表关联主表的外键名， 如：sys_user_post中的user_id" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.joinLeftColName" placeholder="请选择">
              <el-option
                  v-for="(column, index) in joinColumns"
                  :key="index"
                  :label="column.columnName + '：' + column.columnComment"
                  :value="column.columnName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="8" v-if="info.tplCategory==='m2m'">
          <el-form-item>
            <template #label>
              关联子表键名
              <el-tooltip content="连接表关联子表的外键名， 如：sys_user_post中的post_id" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.joinSubTableColName" placeholder="请选择">
              <el-option
                  v-for="(column, index) in joinColumns"
                  :key="index"
                  :label="column.columnName + '：' + column.columnComment"
                  :value="column.columnName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>

        <el-col :span="8" v-if="info.tplCategory!=='subTable'">
          <el-form-item>
            <template #label>
              {{info.tplCategory==="o2o"?"扩展表":"子表"}}
              <el-tooltip :content="hintsMap[info.tplCategory][0]" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.subTableName" placeholder="请选择" @change="subSelectChange">
              <el-option
                  v-for="(table, index) in tables"
                  :key="index"
                  :label="table.tableName + '：' + table.tableComment"
                  :value="table.tableName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="8"  v-if="info.tplCategory==='o2m'||info.tplCategory==='o2o'">
          <el-form-item>
            <template #label>
              关联键名
              <el-tooltip :content="hintsMap[info.tplCategory][1]" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.subTableColName" placeholder="请选择">
              <el-option
                  v-for="(column, index) in subColumns"
                  :key="index"
                  :label="column.columnName + '：' + column.columnComment"
                  :value="column.columnName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="8" v-if="info.tplCategory==='m2m'">
          <el-form-item>
            <template #label>
              子表名称
              <el-tooltip content="子表显示键名，在el-select作为label显示 如：post_name" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.subTableDisColName" placeholder="请选择">
              <el-option
                  v-for="(column, index) in subColumns"
                  :key="index"
                  :label="column.columnName + '：' + column.columnComment"
                  :value="column.columnName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>

        <el-col :span="8" v-if="info.tplCategory==='subTable'">
          <el-form-item>
            <template #label>
              主表
              <el-tooltip content="在一对多或者一对一的主表" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <el-select v-model="info.options.subTableName" placeholder="请选择" @change="subSelectChange">
              <el-option
                  v-for="(table, index) in tables"
                  :key="index"
                  :label="table.tableName + '：' + table.tableComment"
                  :value="table.tableName"
              ></el-option>
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="16"  v-if="info.tplCategory==='subTable'">
          <el-form-item>
            <template #label>
              关联键名
              <el-tooltip content="主表显示键名，在el-select作为label显示 如：user_name" placement="top">
                <el-icon><question-filled /></el-icon>
              </el-tooltip>
            </template>
            <div class="d-flex w-100">
              <el-select v-model="info.options.mainTableDisColName" placeholder="请选择" class="w-50">
                <el-option
                    v-for="(column, index) in subColumns"
                    :key="index"
                    :label="column.columnName + '：' + column.columnComment"
                    :value="column.columnName"
                ></el-option>
              </el-select>
              <el-select v-model="info.options.mainTableDisLabel" placeholder="请选择" class="w-50">
                <el-option
                    v-for="(column, index) in subColumns"
                    :key="index"
                    :label= "column.columnComment"
                    :value="column.columnComment"
                ></el-option>
              </el-select>
            </div>
          </el-form-item>
        </el-col>
      </el-row>
    </template>
  </el-form>
</template>

<script setup>
import { listMenu } from "@/api/system/menu";
import {getGenTable} from "@/api/tool/gen.js";
const hintsMap={
  o2o:["扩展表，比如：主表为sys_user， 如：sys_user_extends，目前仅支持2表有相同的主键","子表关联主表外键名，如：sys_user_extends中主键user_id"],
  o2m:["子表名， 如：sys_user_work_exp","子表关联主表外键名，如：sys_user_work_exp中user_id"],
  m2m:["子表名， 如：sys_post","子表关联主表外键名，如：sys_post中post_id"]
}
const subColumns = ref([]);
//连接表列
const joinColumns = ref([]);

const menuOptions = ref([]);
const { proxy } = getCurrentInstance();

const props = defineProps({
  info: {
    type: Object,
    default: ()=>{
      return {
        options:{}
      };
    }
  },
  tables: {
    type: Array,
    default: ()=>{
      return [];
    }
  }
});

// 表单校验
const rules = ref({
  tplCategory: [{ required: true, message: "请选择生成模板", trigger: "blur" }],
  packageName: [{ required: true, message: "请输入生成包路径", trigger: "blur" }],
  moduleName: [{ required: true, message: "请输入生成模块名", trigger: "blur" }],
  businessName: [{ required: true, message: "请输入生成业务名", trigger: "blur" }],
  functionName: [{ required: true, message: "请输入生成功能名", trigger: "blur" }]
});
function subSelectChange(value) {
  props.info.subTableFkName = "";
}
function tplSelectChange(value) {
  if (value !== "o2m") {
    props.info.subTableName = "";
    props.info.subTableFkName = "";
  }
}
function setSubTableColumns(value) {
  for (var item in props.tables) {
    const name = props.tables[item].tableName;
    if (value === name) {
      let tableId=props.tables[item].tableId;
      getGenTable(tableId).then(res => {
        subColumns.value = res.data.rows;
      });
      break;
    }
  }
}
function setJoinTableColumns(value) {
  for (var item in props.tables) {
    const name = props.tables[item].tableName;
    if (value === name) {
      let tableId=props.tables[item].tableId;
      getGenTable(tableId).then(res => {
        joinColumns.value = res.data.rows;
      });
      break;
    }
  }
}
/** 查询菜单下拉树结构 */
function getMenuTreeselect() {
  listMenu().then(response => {
    menuOptions.value = proxy.handleTree(response.data, "menuId");
  });
}

watch(() => props.info.options.subTableName, val => {
  setSubTableColumns(val);
});
watch(() => props.info.options.joinTableName, val => {
  setJoinTableColumns(val);
});


// watch(() => props.info.tplWebType, val => {
//   if (val === '') {
//     props.info.tplWebType = "element-plus";
//   }
// });

getMenuTreeselect();
</script>
