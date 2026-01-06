#[derive(Clone)]
pub enum BusinessType {
    /**
     * 其它
     */
    OTHER=0,

    /**
     * 新增
     */
    INSERT=1,

    /**
     * 修改
     */
    UPDATE=2,

    /**
     * 删除
     */
    DELETE=3,

    /**
     * 授权
     */
    GRANT=4,

    /**
     * 导出
     */
    EXPORT=5,

    /**
     * 导入
     */
    IMPORT=6,

    /**
     * 强退
     */
    FORCE=7,

    /**
     * 生成代码
     */
    GENCODE=8,

    /**
     * 清空数据
     */
    CLEAN=9,
}

#[derive(Clone)]
pub struct OperState {
    pub title: String,
    pub path:String,
    pub business_type: BusinessType,
}
