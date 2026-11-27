<template>
  <el-card>
    <el-tabs v-model="activeName">
      <el-tab-pane label="基本信息" name="basic">
        <basic-info-form ref="basicInfo" :info="info" :rows="columns"/>
      </el-tab-pane>
      <el-tab-pane label="字段信息" name="columnInfo">
        <el-table ref="dragTable" :data="columns" row-key="columnId" :max-height="tableHeight">
          <el-table-column label="序号" type="index" min-width="5%"/>
          <el-table-column
              label="字段列名"
              prop="columnName"
              min-width="10%"
              :show-overflow-tooltip="true"
          />
          <el-table-column label="字段描述" min-width="10%">
            <template #default="scope">
              <el-input v-model="scope.row.columnComment"></el-input>
            </template>
          </el-table-column>
          <el-table-column
              label="物理类型"
              prop="columnType"
              min-width="10%"
              :show-overflow-tooltip="true"
          />
          <el-table-column label="rust类型" min-width="11%">
            <template #default="scope">
              <el-select v-model="scope.row.rustType">
                <el-option label="u64" value="u64"/>
                <el-option label="String" value="String"/>
                <el-option label="i32" value="i32"/>
                <el-option label="f64" value="f64"/>
                <el-option label="BigDecimal" value="BigDecimal"/>
                <el-option label="Date" value="Date"/>
                <el-option label="bool" value="bool"/>
                <el-option label="char" value="char"/>
                <el-option label="serde_json::Value" value="serde_json::Value"/>
              </el-select>
            </template>
          </el-table-column>
          <el-table-column label="rust属性" min-width="10%">
            <template #default="scope">
              <el-input v-model="scope.row.rustField"></el-input>
            </template>
          </el-table-column>
          <el-table-column label="开关" width="250px">
            <template #default="{row}">

              <el-dropdown trigger="click" :hide-on-click="false" style="padding-left: 12px">
                <div>
                  <el-tag v-for="v in switchNames(row)" size="small" :title="v.title">{{v.name}}</el-tag>
                  <el-button icon="ArrowDown"  text size="small"></el-button>
                </div>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item v-for="v in allSwitches">
                      <el-checkbox v-model="row[v.key]" true-value="1" false-value="0" :label="v.title"/>
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>


            </template>
          </el-table-column>

<!--          <el-table-column label="插入" min-width="5%">-->
<!--            <template #default="scope">-->
<!--              <el-checkbox true-value="1" false-value="0" v-model="scope.row.isInsert"></el-checkbox>-->
<!--            </template>-->
<!--          </el-table-column>-->
<!--          <el-table-column label="编辑" min-width="5%">-->
<!--            <template #default="scope">-->
<!--              <el-checkbox true-value="1" false-value="0" v-model="scope.row.isEdit"></el-checkbox>-->
<!--            </template>-->
<!--          </el-table-column>-->
<!--          <el-table-column label="列表" min-width="5%">-->
<!--            <template #default="scope">-->
<!--              <el-checkbox true-value="1" false-value="0" v-model="scope.row.isList"></el-checkbox>-->
<!--            </template>-->
<!--          </el-table-column>-->
<!--          <el-table-column label="显示" min-width="5%" title="we3erqwe">-->
<!--            <template #default="scope">-->
<!--              <el-checkbox true-value="1" false-value="0" v-model="scope.row.isTable"></el-checkbox>-->
<!--            </template>-->
<!--          </el-table-column>-->
<!--          <el-table-column label="详情" min-width="5%">-->
<!--            <template #default="scope">-->
<!--              <el-checkbox true-value="1" false-value="0" v-model="scope.row.isDetail"></el-checkbox>-->
<!--            </template>-->
<!--          </el-table-column>-->
<!--          <el-table-column label="导出" min-width="5%">-->
<!--            <template #default="scope">-->
<!--              <el-checkbox true-value="1" false-value="0" v-model="scope.row.isExport"></el-checkbox>-->
<!--            </template>-->
<!--          </el-table-column>-->
          <el-table-column label="查询" min-width="5%">
            <template #default="scope">
              <el-checkbox true-value="1" false-value="0" v-model="scope.row.isQuery"></el-checkbox>
            </template>
          </el-table-column>
          <el-table-column label="查询方式" min-width="10%">
            <template #default="scope">
              <el-select v-model="scope.row.queryType">
                <el-option label="=" value="EQ"/>
                <el-option label="!=" value="NE"/>
                <el-option label=">" value="GT"/>
                <el-option label=">=" value="GTE"/>
                <el-option label="<" value="LT"/>
                <el-option label="<=" value="LTE"/>
                <el-option label="LIKE" value="LIKE"/>
                <el-option label="BETWEEN" value="BETWEEN"/>
              </el-select>
            </template>
          </el-table-column>
<!--          <el-table-column label="必填" min-width="5%">-->
<!--            <template #default="scope">-->
<!--              <el-checkbox true-value="1" false-value="0" v-model="scope.row.isRequired"></el-checkbox>-->
<!--            </template>-->
<!--          </el-table-column>-->
          <el-table-column label="显示类型" min-width="12%">
            <template #default="scope">
              <el-select v-model="scope.row.htmlType">
                <el-option label="文本框" value="input"/>
                <el-option label="文本域" value="textarea"/>
                <el-option label="下拉框" value="select"/>
                <el-option label="单选框" value="radio"/>
                <el-option label="开关" value="switch"/>
                <el-option label="复选框" value="checkbox"/>
                <el-option label="数字框" value="number"/>
                <el-option label="日期时间控件" value="datetime"/>
                <el-option label="日期控件" value="date"/>
                <el-option label="图片上传" value="imageUpload"/>
                <el-option label="文件上传" value="fileUpload"/>
                <el-option label="富文本控件" value="editor"/>
              </el-select>
            </template>
          </el-table-column>
          <el-table-column label="字典类型" min-width="12%">
            <template #default="{row}">
              <el-select v-model="row.dictType" clearable filterable placeholder="请选择">
                <el-option
                    v-for="dict in dictOptions"
                    :key="dict.dictType"
                    :label="dict.dictName"
                    :value="dict.dictType">
                  <span style="float: left">{{ dict.dictName }}</span>
                  <span style="float: right; color: #8492a6; font-size: 13px">{{ dict.dictType }}</span>
                </el-option>
              </el-select>
            </template>
          </el-table-column>
          <el-table-column label="操作" align="center" min-width="5%">
            <template #default="scope">
              <el-button
                  size="small"
                  text
                  icon="More"
                  @click="openMore(scope.row)"
              >
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
      <el-tab-pane label="生成信息" name="genInfo">
        <gen-info-form ref="genInfo" :info="info" :tables="tables"/>
      </el-tab-pane>
    </el-tabs>
    <el-form label-width="100px">
      <div style="text-align: center;margin-left:-100px;margin-top:10px;">
        <el-button type="primary" @click="submitForm()">提交</el-button>
        <el-button @click="close()">返回</el-button>
      </div>
    </el-form>
    <el-dialog :title="title" v-model="open" width="600px" append-to-body>
      <el-form>
        <el-row>
          <template v-if="currentCol.rustType==='String'">
            <el-col :span="24">
              <el-form-item label="正则表达式">
                <el-input
                    v-model="moreInput.pattern"
                    placeholder="请输入正则表达式"
                ></el-input>
              </el-form-item>
            </el-col>
            <el-col :span="24">
              <el-form-item label="正则消息">
                <el-input
                    v-model="moreInput.patternMessage"
                    placeholder="请输入正则表达式消息提示"
                ></el-input>
              </el-form-item>
            </el-col>
          </template>
          <template v-if="currentCol.htmlType==='number'">
            <el-col :span="12">
              <el-form-item label="最小值">
                <el-input-number controls-position="right"
                                 v-model="moreInput.min"
                                 placeholder="请输入最小值"
                ></el-input-number>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="最大值">
                <el-input-number controls-position="right"
                                 v-model="moreInput.max"
                                 placeholder="请输入最大值"
                ></el-input-number>
              </el-form-item>
            </el-col>
          </template>

          <el-col :span="24">
            <el-form-item label="开关">
              <el-checkbox v-if="currentCol.rustType==='String'"
                           v-model="moreInput.checkLength" true-value="1" false-value="0" title="允许导出"
              >检查字符长度
              </el-checkbox>
              <el-checkbox v-model="moreInput.update" true-value="1" false-value="0"
                           title="只更新此字段，比如state"
              >单独更新
              </el-checkbox>
              <el-checkbox v-if="currentCol.rustType==='String'"
                           v-model="moreInput.isUnique" true-value="1" false-value="0" title="检查唯一值，仅支持String"
              >检查唯一
              </el-checkbox>
            </el-form-item>
          </el-col>
          <el-col :span="24">
            <el-form-item label="排序">
              <el-checkbox v-model="moreInput.defaultOrder" true-value="1" false-value="0" title="选定为默认排序的列"
              >排序
              </el-checkbox>
              <el-radio-group v-if="moreInput.defaultOrder==='1'" v-model="moreInput.isAsc" style="margin-left: 20px">
                <el-radio value="">正序</el-radio>
                <el-radio value="desc">反序</el-radio>
              </el-radio-group>
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
      <template #footer>
        <el-button type="primary" @click="confirmMore">确 定</el-button>
        <el-button @click="cancelMore">取消</el-button>
      </template>
    </el-dialog>

  </el-card>
</template>

<script setup name="GenEdit">
import {getGenTable, updateGenTable} from "@/api/tool/gen";
import {optionselect as getDictOptionselect} from "@/api/system/dict/type";
import basicInfoForm from "./basicInfoForm.vue";
import genInfoForm from "./genInfoForm.vue";

const route = useRoute();
const {proxy} = getCurrentInstance();

const activeName = ref("columnInfo");
const tableHeight = ref(document.documentElement.scrollHeight - 245 + "px");
const tables = ref([]);
const columns = ref([]);
const dictOptions = ref([]);
const info = ref({options: {}});

/** 提交按钮 */
function submitForm() {
  const basicForm = proxy.$refs.basicInfo.$refs.basicInfoForm;
  const genForm = proxy.$refs.genInfo.$refs.genInfoForm;
  Promise.all([basicForm, genForm].map(getFormPromise)).then(res => {
    const validateResult = res.every(item => !!item);
    if (validateResult) {
      const genTable = Object.assign({}, info.value);
      genTable.columns = columns.value;
      // genTable.params = { todo
      //   treeCode: info.value.treeCode,
      //   treeName: info.value.treeName,
      //   treeParentCode: info.value.treeParentCode,
      //   parentMenuId: info.value.parentMenuId
      // };
      updateGenTable(genTable).then(res => {
        proxy.$modal.msgSuccess(res.msg);
        if (res.code === 200) {
          close();
        }
      });
    } else {
      proxy.$modal.msgError("表单校验未通过，请重新检查提交内容");
    }
  });
}

function getFormPromise(form) {
  return new Promise(resolve => {
    form.validate(res => {
      resolve(res);
    });
  });
}

function close() {
  const obj = {path: "/tool/gen"};
  proxy.$tab.closeOpenPage(obj);
}

(() => {
  const tableId = route.params && route.params.tableId;
  if (tableId) {
    // 获取表详细信息
    getGenTable(tableId).then(res => {
      columns.value = res.data.rows;
      if (!res.data.info.options) {
        res.data.info.options = {}
      } else if (typeof res.data.info.options === 'string') {
        res.data.info.options = JSON.parse(res.data.info.options);
      }

      res.data.info.columns = res.data.rows;
      info.value = res.data.info;

      tables.value = res.data.tables;
    });
    /** 查询字典下拉列表 */
    getDictOptionselect().then(response => {
      dictOptions.value = response.data;
    });
  }
})();


const title = ref("")
const open = ref(false);
const currentCol = ref({});
const moreInput = ref({});

function openMore(row) {
  if(!row.more){
    row.more = {};
  }
  currentCol.value = row;
  title.value =  row.columnComment + " 字段的更多设置";
  open.value = true;
  const _moreInput = {utf: 0, checkLength: 0, defaultOrder: 0, update: 0, isAsc: undefined}
  if (row.more) {
    Object.assign(_moreInput, row.more)
  }
  moreInput.value = _moreInput;
}

function confirmMore() {

  Object.assign(currentCol.value.more, moreInput.value)
  currentCol.value = {};
  open.value = false;
}

function cancelMore() {
  moreInput.value = {};
  open.value = false;
}

const allSwitches = [
  {
    key: "isInsert",
    name: "增",
    title: "可插入数据库"
  },
  {
    key: "isEdit",
    name: "改",
    title: "可修改"
  },
  {
    key: "isList",
    name: "表",
    title: "从数据库查询列表时，查询此字段"
  },
  {
    key: "isTable",
    name: "显",
    title: "在前端表格显示"
  },
  {
    key: "isDetail",
    name: "详",
    title: "从数据库查询详情时，查询此字段"
  },
  {
    key: "isExport",
    name: "导",
    title: "导出时，包含此字段"
  },
  {
    key: "isRequired",
    name: "必",
    title: "必填"
  },
]
function switchNames(row){
  return allSwitches.filter(item =>row[item.key]==='1');

}

</script>
