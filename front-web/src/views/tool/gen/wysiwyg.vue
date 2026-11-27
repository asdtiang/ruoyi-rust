<template>
  <div class="app-container">
    <element-plus-render ref="renderRef" :draw-item-list="drawingList"
                         v-model="formModelAndRules"></element-plus-render>
  </div>

</template>

<script setup name="GenEdit">
import {getGenTable} from "@/api/tool/gen.js";
import {useRoute} from "vue-router";

const route = useRoute();
const tables = ref([]);
const columns = ref([]);
const info = ref({});
import {getDicts} from "@/api/system/dict/data.js";


const renderRef = ref();

let buildDrawItem
let getRefs
const dictMap = {};
onMounted(() => {
  buildDrawItem = renderRef.value.buildDrawItem;
  getRefs = renderRef.value.getRefs;
  const tableId = route.params && route.params.tableId;
  if (tableId) {

    // 获取表详细信息
    getGenTable(tableId).then(res => {
      columns.value = res.data.rows;
      info.value = res.data.info;
      tables.value = res.data.tables;
      const promises = [];
      for (const col of columns.value) {
        if (col.dictType && !dictMap[col.dictType]) {
          dictMap[col.dictType] = [];
          promises.push(getDicts(col.dictType));
        }
      }
      Promise.all(promises).then(all => {
        all.forEach(r => {
          r.data.forEach(item => {
            dictMap[item.dictType].push({
              label: item.dictLabel,
              value: item.dictValue,
              elTagType: item.listClass,
              // elTagClass:item.dictLabel
            })
          })
        })
        buildDrawList();
      });
    });
  }
})
const drawingList = ref([]);
const total = ref(100)

const formModelAndRules = ref({
  queryParams: {}
});

function buildDrawList() {


  drawingList.value.push(
      buildQueryFrom(),
      buildActionBar(),
      buildTable(),
      buildInputDlg(),
      buildPage()
  )

}

const showSearch = ref(true)

function buildQueryFrom() {

  const queryFormInputs = []
  const queryForm = buildDrawItem("form", {
    __props__: {
      "model": "queryParams",
      ref: "queryRef"
    },
    __native__: {
      "class": "queryForm",
    },
    "__slots__": {
      "default": [{
        ...buildDrawItem("row", {
          __props__: {
            gutter: 15
          }
        }),
        "__slots__": {
          default: queryFormInputs
        }
      }]
    }
  })
  columns.value.filter(col => col.query).forEach(col => {
    const input = buildQueryDrawItem(col);
    const elCol = buildDrawItem("col", {
      "__props__": {
        md: 12, lg: 8, xl: 6
      },
      __slots__: {
        default: [input]
      }
    })

    queryFormInputs.push(elCol)
  })
  const searchBtn = buildDrawItem("button", {
    "__props__": {
      "type": "primary",
      "icon": "Search",
      onclick: getList

    },
    __slots__: {
      default: ["查询"]
    }
  })
  const resetBtn = buildDrawItem("button", {
    "__props__": {
      "icon": "Refresh",
      onclick: resetQueryForm
    },
    __slots__: {
      default: ["重置"]
    }
  })
  const elCol = buildDrawItem("col", {
    "__props__": {
      md: 12, lg: 8, xl: 6
    },
    __slots__: {
      default: [searchBtn, resetBtn]
    }
  })
  queryFormInputs.push(elCol)
  return queryForm;
}


function buildActionBar() {
  const actionButtons = []
  const actionRow = buildDrawItem("row", {
    __props__: {
      gutter: 10
    },
    __native__: {
      class: "mb8"
    },
    __slots__: {
      default: actionButtons
    }
  });


  const newBtn = buildDrawItem("col", {
    "__props__": {
      span: 1.5
    },
    __slots__: {
      default: [buildDrawItem("button", {
        "__props__": {
          "type": "primary",
          "icon": "Plus",
          "plain": true,
          onClick: handleAdd
        },
        __slots__: {
          default: ["新增"]
        }
      })]
    }
  })
  const deleteBtn = buildDrawItem("col", {
    "__props__": {
      span: 1.5
    },
    __slots__: {
      default: [buildDrawItem("button", {
        "__props__": {
          "type": "danger",
          "icon": "Delete",
          "plain": true,
          disabled: rowUnChecked,
          onclick: () => handleDelete()
        },
        __slots__: {
          default: ["删除"]
        }
      })]
    }
  })
  const exportBtn = buildDrawItem("col", {
    "__props__": {
      span: 1.5
    },
    __slots__: {
      default: [buildDrawItem("button", {
        "__props__": {
          "type": "warning",
          "icon": "Download",
          "plain": true,
          onclick: handleExport
        },
        __slots__: {
          default: ["导出"]
        }
      })]
    }
  })
  const rightToolbar = {
    __config__: {
      tag: "right-toolbar",
      layout: "containerItem"
    },
    "__props__": {
      span: 1.5,
      onQueryTable: getList,
      showSearch: showSearch,
      'onUpdate:showSearch': (v) => {
        showSearch.value = v;
      },
    },
    __slots__: {
      default: [buildDrawItem("button", {
        "__props__": {
          "type": "warning",
          "icon": "Download",
          "plain": true,

        }

      })]
    }
  }
  actionButtons.push(newBtn, deleteBtn, exportBtn, rightToolbar)
  return actionRow;
}

const loading = ref(false)//fixme loading不起作用

function buildTable() {

  //render table
  const elTable = buildDrawItem("table", {
    __props__: {
      data: list,
      onSelectionChange: handleSelectionChange
    },
    __directives__: {
      // "loading": {
      //   "value": loading
      // },
      "fixedTableHeight": {
        "value": 85
      }
    }
  })
  delete elTable.__data__
  elTable.__slots__.default = columns.value.filter(col => col.list).map(col => {
    const item = buildDrawItem("table-column", {
      __props__: {
        label: col.comment,
        align: "center",
        prop: col.javaField,
        showOverflowTooltip: col.htmlType === "textarea" || col.columnType === "json"
      }
    })
    if (col.dictType) {
      const dict = dictMap[col.dictType];
      const field = col.javaField;
      item.__slots__.default = ({row}) => h(resolveComponent("dict-tag"), {
            options: dict,
            value: row[field]
          }
      )
    }

    return item
  })
  const pkCol = columns.value.find(col => col.pk === true)
  elTable.__slots__.default.splice(0, 0, buildDrawItem("table-column", {
    __props__: {
      type: "selection",
      width: 55,
      align: "center"
    }
  }));
  elTable.__slots__.default.push(buildDrawItem("table-column", {
    __slots__: {
      default: ({row}) => [
        h(resolveComponent("el-button"), {
          type: "primary",
          text: true,
          icon: "Edit",
          onclick: (e) => {
            handleUpdate(row[pkCol.javaField])
          }
        }, () => "修改"),
        h(resolveComponent("el-button"), {
          type: "danger",
          text: true,
          icon: "Delete",
          onclick: (e) => {
            handleDelete(row[pkCol.javaField])
          }
        }, () => "修改")]
    }
  }))


  return elTable;
}

const title = ref("")

function buildInputDlg() {
  return buildDrawItem("dialog", {
    __config__: {
      "defaultValue": false
    },
    __props__: {
      title: title
    },
    __slots__: {
      default: [buildDrawItem("form", {
        __props__: {
          "model": "form",
          rules: "rules",
          ref: info.value.businessName + "Ref"
        },
        __native__: {
          "class": "queryForm",
        },
        __slots__: {
          default: [
            buildDrawItem("row", {
              __props__: {
                gutter: 15
              },
              __slots__: {
                default: buildFormInputs()
              }
            })
          ]
        },
      })],

      footer: [
        buildDrawItem("button", {
          "__props__": {
            "type": "primary",
            onclick: submitForm
          },
          __slots__: {
            default: ["确定"]
          }
        }),
        buildDrawItem("button", {
          "__props__": {
            // "type": "primary"
            onclick: (e) => {
              formModelAndRules.value["dialog1"] = false;
            }
          },

          __slots__: {
            default: ["取消"]
          }
        })
      ]
    },
    __vModel__: "dialog1"
  });
}

function buildPage() {
  return {
    __config__: {
      tag: "pagination",
      layout: 'rawItem'
    },
    __props__: {
      total: total,
      onPagination: getList,
      page: pageNum,
      limit: pageSize,
      'onUpdate:page': (v) => {
        pageNum.value = v;
      },
      'onUpdate:limit': (v) => {
        pageSize.value = v;
      }
    },
    __slots__: {}
  }
}


function buildQueryDrawItem(col) {

  const {htmlType, queryType, dictType} = col;

  let id = "input";
  const props = {};
  let dict
  switch (htmlType) {
    case "number":
      id = "input-number";
      if (queryType === "BETWEEN") {
        props.style = {
          width: "unset"
        }
      }
      props.controlsPosition = "right";
      props.placeholder = `请输入${col.columnComment}（${col.queryTypeString}）`;
      break;
    case "radio":
    case "select":
      id = "select"
      props.clearable = true;
      props.placeholder = `请选择${col.columnComment}`;
      if (dictType) {
        dict = dictMap[dictType];
        props.options = dict;
      }
      break;
    case "input":
    case "textarea":
      id = "input"
      props.clearable = true;
      props.placeholder = `请输入${col.columnComment}（${col.queryTypeString}）`;
      break;
    case "datetime":
      id = "date-picker";
      if (queryType === "BETWEEN") {
        props.style = {
          width: "unset"
        }
      }
      props["value-format"] = "YYYY-MM-DD";
      props.type = "date"
      props.placeholder = `请选择${col.columnComment}（${col.queryTypeString}）`;
      break;
  }
  if (queryType === "BETWEEN") {

    const div = buildDrawItem("div", {
      __native__: {
        class: "d-flex"
      },
      __slots__: {
        default: [
          buildDrawItem(id, {
            "__config__": {
              "label": `${col.columnComment}区间`,
              required: false,
              wrapWithFormItem: true
            },
            "__props__": props,
            "__vModel__": `params.begin${col.capJavaField}`
          }),
          buildDrawItem("div", {
            __props__: {
              style: {
                margin: "5px"
              }
            },
            __slots__: {
              default: ["至"]
            }
          }),
          buildDrawItem(id, {
            "__config__": {
              required: false,
              wrapWithFormItem: true
            },
            "__props__": props,
            "__vModel__": `params.end${col.capJavaField}`
          })
        ]
      }

    });

    return div;

  } else {
    const config = {
      "__config__": {
        "label": `${col.columnComment}`,
        required: false,
        wrapWithFormItem: true
      },
      "__props__": props,
      "__vModel__": col.javaField
    };

    const input = buildDrawItem(id, config);
    if (!input) {
      console.info(col.htmlType)
    }

    return input;
  }


}

let defaultFormVal = {}

function buildFormInputs() {
  defaultFormVal = {};
  const inputs = []

  columns.value.filter(col => col.editAndInsert).forEach(col => {
    const {htmlType} = col;
    const input = buildInputDrawItem(col);
    const elCol = buildDrawItem("col", {
      "__props__": {
        span: htmlType === 'textarea' ||htmlType === 'imageUpload'? 24 : 12
      },
      __slots__: {
        default: [input]
      }
    })

    inputs.push(elCol)
  })
  return inputs

}

function buildInputDrawItem(col) {

  const {htmlType, dictType} = col;
  let id = "input";
  let dict
  const custConfig = {}
  const props = {};
  switch (htmlType) {
    case "number":
      id = "input-number";
      props.controlsPosition = "right";
      props.placeholder = `请输入${col.columnComment}`;
      break;
    case "radio":
      id = "radio-group"
      if (dictType) {
        dict = dictMap[dictType];
        props.options = dict;
      }
      custConfig.optionType = "button";
      break;
    case "select":
      id = "select";

      if (dictType) {
        dict = dictMap[dictType];
        props.options = dict;
      }
      break;
    case "input":
      id = "input"
      props.placeholder = `请输入${col.columnComment}`;
      break;
    case "textarea":
      id = "input"
      props.placeholder = `请输入${col.columnComment}`;
      props.type = "textarea";
      break;
    case "datetime":
      id = "date-picker";
      props.placeholder = `请选择${col.columnComment}`;
      break;
    case "imageUpload":
      id = "OssUploader";
      custConfig.layout = "rawItem";
    //  custConfig.defaultValue=[]
      break;
  }

  const config = {
    "__config__": {
      "label": `${col.columnComment}`,
      required: col.required,
      defaultValue: col.defVal,
      wrapWithFormItem: true,
      ...custConfig
    },
    "__props__": props,
    "__vModel__": col.javaField
  };
  defaultFormVal[col.javaField] = col.defVal
  const input = buildDrawItem(id, config);
  id === "OssUploader"&&  console.info(input)
  if (!input) {
    console.info(col.htmlType)
  }

  return input;
}

setTimeout(() => {
  getList();
}, 2000)
import request from '@/utils/request'
import {ElMessage, ElMessageBox} from "element-plus";

const list = ref([])
const pageNum = ref(1);
const pageSize = ref(10);


function rebuildParams() {
  const queryParams = formModelAndRules.value["queryParams"];

  const params = {};
  Object.assign(params, queryParams);
  params.params = {};
  for (const k in params) {
    if (k.indexOf("params.") === 0) {
      params.params[k.substring(7)] = params[k];
      delete params[k];
    }
  }
  return params;
}

function getList() {
  const params = rebuildParams();

  params.pageNum = pageNum.value;
  params.pageSize = pageSize.value;
  loading.value = true;
  request({
    url: `/${info.value.moduleName}/${info.value.businessName}/list`,
    method: 'get',
    params
  }).then(response => {
    list.value = response.rows;
    total.value = response.total;
    loading.value = false;
  })

}

function resetQueryForm() {
  getRefs().queryRef.resetFields();
  formModelAndRules.value["queryParams"] = {}
  getList();
}

function resetInputForm() {
  const formRef = getRefs()[info.value.businessName + "Ref"];
  if (!formRef) {
    setTimeout(() => {
      getRefs()[info.value.businessName + "Ref"].resetFields();
    }, 500)
  } else {
    formRef.resetFields();
  }
}

/** 新增按钮操作 */
function handleAdd() {
  resetInputForm();
  Object.assign(formModelAndRules.value.form, defaultFormVal);
  formModelAndRules.value['dialog1'] = true

  title.value = "添加" + info.value.tableComment;
}

function handleUpdate(id) {
  resetInputForm();
  request({
    url: `/${info.value.moduleName}/${info.value.businessName}/` + id,
    method: 'get'
  }).then(response => {
    Object.assign(formModelAndRules.value.form, response.data);
    formModelAndRules.value["dialog1"] = true;
    title.value = "修改" + info.value.tableComment;
  });
}

function handleDelete(id) {
  const ids_ = id || ids.value;
  ElMessageBox.confirm(`是否确认删除${info.value.tableComment}编号为${ids_}的数据项？`).then(function () {
    return request({
      url: `/${info.value.moduleName}/${info.value.businessName}/` + ids_,
      method: 'delete'
    })
  }).then(() => {
    getList();
    ElMessage.success("删除成功");
  }).catch(() => {
  });
}

/** 提交按钮 */
function submitForm() {
  const formRef = getRefs()[info.value.businessName + "Ref"];
  if (!formRef) {
    return
  }
  const pkCol = columns.value.find(col => col.pk === true)
  const data = formModelAndRules.value["form"]
  formRef.validate(valid => {
    if (valid) {
      if (data[pkCol.javaField] != null) {
        request({
          url: `/${info.value.moduleName}/${info.value.businessName}`,
          method: 'put',
          data
        }).then(response => {
          ElMessage.success("修改成功");
          formModelAndRules.value["dialog1"] = false;
          getList();
        })
      } else {
        request({
          url: `/${info.value.moduleName}/${info.value.businessName}`,
          method: 'post',
          data
        }).then(response => {
          ElMessage.success("新增成功");
          formModelAndRules.value["dialog1"] = false;
          getList();
        });
      }
    }
  });
}

import {download} from "@/utils/request.js";

/** 导出按钮操作 */
function handleExport() {
  download(`${info.value.moduleName}/${info.value.businessName}/export`, rebuildParams(), `${info.value.businessName}_${new Date().getTime()}.xlsx`)
}

const ids = ref([])
const rowUnChecked = ref(true);

function handleSelectionChange(selection) {
  const pkCol = columns.value.find(col => col.pk === true)
  ids.value = selection.map(item => item[pkCol.javaField]);
  rowUnChecked.value = ids.value.length === 0
}

watch(showSearch, (val) => {
  if (val) {
    drawingList.value.splice(0, 0, buildQueryFrom());
  } else {
    drawingList.value.shift();
  }
})


// "__directives__": {
//   "tooltip": {
//     "name": "tooltip",
//         "modifiers": {
//       "position": "left",
//           "event": "hover"
//     },
//     "options": {
//       "value": "wertwer",
//           "disabled": false,
//           "escape": true,
//           "fitContent": true,
//           "showDelay": 0,
//           "hideDelay": 0,
//           "autoHide": true,
//           "pt": {},
//       "unstyled": false
//     }
//   }
// },
</script>
