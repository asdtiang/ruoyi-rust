# RuoYi-Vue Rust 权限系统说明

加以下宏表示检查权限，如果参数为空，表示登录即可访问。
#[pre_authorize("system:user:list", user_cache)]


> 快速了解和使用权限系统

---

## 一、权限系统概述

### 权限模型
```
用户 → 角色 → 菜单权限 → 操作权限
```

### 核心组成
- **UserCache**：用户缓存对象（存 Redis）
- **权限字符串**：格式 `模块:实体:操作`（如 `system:user:list`）
- **数据权限**：5种数据范围控制
- **权限注解**：`#[pre_authorize]` 宏

---

## 二、数据库表结构

### 核心表
```sql
-- 1. sys_user（用户表）
user_id, user_name, dept_id, status

-- 2. sys_role（角色表）
role_id, role_name, role_key, data_scope, status
-- data_scope: 1全部 2自定义 3本部门 4本部门及以下 5仅本人

-- 3. sys_menu（菜单表）
menu_id, menu_name, parent_id, perms, menu_type
-- perms: 权限标识（如 system:user:list）

-- 4. sys_user_role（用户角色关联表）
user_id, role_id

-- 5. sys_role_menu（角色菜单关联表）
role_id, menu_id
```

---

## 三、核心代码

### 1. UserCache 结构体
```rust
// src/web/token/auth.rs
pub struct UserCache {
    pub user_id: String,
    pub user_name: String,
    pub dept_id: String,
    pub permissions: Vec<String>,  // 权限列表
    pub roles: Vec<CommonRoleVO>, // 角色列表
    // ...
}

impl UserCache {
    pub fn is_admin(&self) -> bool {
        self.user_name.eq("admin")  // 管理员拥有所有权限
    }
}
```

### 2. 权限检查
```rust
// src/web/token/permit.rs
pub async fn check_permit(user_cache: &UserCache, permit_str: &str) -> Option<RespVO<u64>> {
    if permit_str.is_empty() { return None; }
    if user_cache.is_admin() { return None; }  // 管理员直接通过
    if user_cache.permissions.contains(permit_str) { return None; }
    Some(RespVO::from_error_info("无权限访问"))
}
```

### 3. 数据权限过滤
```rust
// src/web/data_scope.rs
pub async fn build_data_scope<T>(
    dto: &mut T,
    dept_alias: &str,  // 部门字段别名
    user_alias: &str,  // 用户字段别名
    user_cache: &UserCache
) -> Result<bool> {
    // 根据角色的 data_scope 构建 SQL 条件
    // DATA_SCOPE_ALL: 不过滤
    // DATA_SCOPE_DEPT: d.dept_id = '当前部门'
    // DATA_SCOPE_SELF: u.user_id = '当前用户'
}
```

---

## 四、使用方法

### 1. Controller 权限注解
```rust
use macros::pre_authorize;

// 检查权限
#[pre_authorize("system:user:list", user_cache)]
pub async fn list(dto: Json<UserPageDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_user_service.page(&dto.0, &user_cache).await;
    PageVO::from_result(&vo).into_response()
}

// 不检查权限（公开接口）
#[pre_authorize]
pub async fn get_info(user_cache: UserCache) -> impl IntoResponse {
    // ...
}
```

### 2. DTO 启用数据权限
```rust
use macros::page_request;

// 启用数据权限
#[page_request(params, dataScope)]
pub struct UserPageDTO {
    pub user_name: Option<String>,
    // 自动添加: page_no, page_size, params
}
```

### 3. Service 应用数据权限
```rust
use crate::web::data_scope::build_data_scope;

pub async fn page(&self, dto: &mut UserPageDTO, user_cache: &UserCache) -> Result<Page<SysUserVO>> {
    // 构建数据权限 SQL
    build_data_scope(dto, "d", "u", user_cache).await?;

    // 使用数据权限查询
    let result = SysUser::select_page(pool!(), &dto.into(), &dto.params).await?;
    Ok(result)
}
```

### 4. SQL 使用数据权限
```rust
let data_scope = params.get("dataScope").unwrap_or(&String::new());

let sql = format!(
    "SELECT * FROM sys_user u
     LEFT JOIN sys_dept d ON u.dept_id = d.dept_id
     WHERE u.del_flag = '0' {}",
    data_scope  // 自动插入数据权限条件
);
```

---

## 五、权限字符串规范

### 格式
```
模块:实体:操作
```

### 常用操作
| 操作 | 说明 |
|------|------|
| list | 列表查询 |
| query | 详情查询 |
| add | 新增 |
| edit | 编辑 |
| remove | 删除 |
| export | 导出 |

### 示例
```
system:user:list      # 用户列表
system:user:add       # 新增用户
system:user:edit      # 编辑用户
system:user:remove    # 删除用户
system:role:list      # 角色列表
```

---

## 六、数据权限类型

### 5种数据范围
| 值 | 名称 | SQL条件 |
|---|------|---------|
| 1 | 全部数据 | 无过滤 |
| 2 | 自定义数据 | d.dept_id IN (SELECT dept_id FROM sys_role_dept) |
| 3 | 本部门数据 | d.dept_id = '当前部门ID' |
| 4 | 本部门及以下 | d.dept_id IN (SELECT dept_id FROM sys_dept WHERE FIND_IN_SET('当前部门ID', ancestors)) |
| 5 | 仅本人数据 | u.user_id = '当前用户ID' |

### 使用场景
- **管理员**：全部数据
- **部门主管**：本部门及以下
- **普通员工**：本部门或仅本人

---

## 七、实战示例

### 示例1：创建带权限的API

```rust
// 1. Controller
#[pre_authorize("system:product:list", user_cache)]
pub async fn list(dto: Json<ProductPageDTO>) -> impl IntoResponse {
    let data = CONTEXT.product_service.page(&dto.0, &user_cache).await;
    PageVO::from_result(&data).into_response()
}

#[pre_authorize("system:product:add", user_cache)]
pub async fn add(dto: ValidatedForm<ProductAddDTO>) -> impl IntoResponse {
    let rows = CONTEXT.product_service.add(dto.0, &user_cache).await;
    RespVO::judge_result(rows, "添加成功", "添加失败").into_response()
}

// 2. 注册路由
Router::new()
    .route("/list", post(list))
    .route("/", post(add))
```

### 示例2：实现仅本人数据权限

```rust
// DTO
#[page_request(params, dataScope)]
pub struct OrderPageDTO {
    pub order_no: Option<String>,
}

// Service
pub async fn page(&self, dto: &mut OrderPageDTO, user_cache: &UserCache) -> Result<Page<OrderVO>> {
    build_data_scope(dto, "d", "u", user_cache).await?;
    let result = Order::select_page(pool!(), &dto.into(), &dto.params).await?;
    Ok(result)
}

// SQL
let sql = format!(
    "SELECT o.* FROM sys_order o
     LEFT JOIN sys_dept d ON o.dept_id = d.dept_id
     LEFT JOIN sys_user u ON o.user_id = u.user_id
     WHERE o.del_flag = '0' {}",
    params.get("dataScope").unwrap_or(&String::new())
);
```

---

## 八、常见问题

### 1. 权限检查不生效？
```rust
// 检查点：
// 1. 是否使用了 #[pre_authorize]
#[pre_authorize("system:user:list")]  // 必须有

// 2. 权限字符串是否正确（区分大小写）
"system:user:list"  // 正确
"System:User:List"  // 错误

// 3. 数据库中角色是否有对应权限
SELECT * FROM sys_role_menu WHERE role_id = '角色ID';
```

### 2. 数据权限不生效？
```rust
// 1. DTO 必须有 dataScope 标记
#[page_request(params, dataScope)]  // 必须有 dataScope

// 2. Service 必须调用 build_data_scope
build_data_scope(dto, "d", "u", user_cache).await?;

// 3. SQL 必须使用 params.data_scope
let sql = format!("... {}", params.get("dataScope").unwrap_or(&String::new()));
```

### 3. 管理员权限不生效？
```rust
// 检查管理员用户名是否为 admin
// src/config/global_constants.rs
pub const ADMIN_NAME: &str = "admin";
```

### 4. 如何刷新权限缓存？
```
方法1：用户重新登录
方法2：调用刷新接口清除 Redis 缓存
```

---

## 九、最佳实践

### 命名规范
```rust
// 权限字符串：小写 + 冒号分隔
"system:user:list"
"system:user:add"
"system:role:edit"

// 数据权限别名
"u"  // 用户表别名
"d"  // 部门表别名
```

### 安全建议
✅ 最小权限原则
✅ 及时收回离职员工权限
✅ 重要操作需要二次验证
✅ 记录权限变更日志

---

## 十、相关文件

| 文件 | 路径 |
|------|------|
| UserCache | `src/web/token/auth.rs` |
| check_permit | `src/web/token/permit.rs` |
| build_data_scope | `src/web/data_scope.rs` |
| pre_authorize 宏 | `macros/src/lib.rs` |

---

**快速上手**：在 Controller 上加 `#[pre_authorize("权限字符串")]` 即可！
