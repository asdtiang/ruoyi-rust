/**
 * 固定el-table的高度
 * 用法：v-fixedTableHeight，value是要减去高度。
 * Copyright (c) 2019 ruoyi
 */


export default {
    updated(el, {value}, vnode) {
        value = value || 75
        let height = window.innerHeight - el.getBoundingClientRect().top;
        el.style.height = (height - value) + "px"
    }
}

