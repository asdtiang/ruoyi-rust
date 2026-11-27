# Liquibase 变更规则

- 数据库变更统一写成 SQL，放在变更集的 `<sql>` 标签内。
- XML 只用于 `databaseChangeLog`、`changeSet` 和 `<sql>` 的包装，以及 `master.xml` 的变更集引用。
- 不使用 `<dropColumn>`、`<addColumn>`、`<createTable>` 等 Liquibase 声明式数据库变更标签。
