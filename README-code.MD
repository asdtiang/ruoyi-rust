# RuoYi-Vue Rust 项目完整说明文档

> 🎯 **适合Rust新手的详细项目说明**
>
> 本文档详细介绍了若依Rust版本项目的各个方面，帮助你快速理解和使用这个项目。

---

## 📋 目录

1. [项目简介](#项目简介)
2. [技术栈详解](#技术栈详解)
3. [项目结构详解](#项目结构详解)
4. [配置文件说明](#配置文件说明)
5. [快速开始](#快速开始)
6. [核心概念讲解](#核心概念讲解)
7. [开发指南](#开发指南)
8. [常见问题](#常见问题)

---

## 项目简介

### 什么是RuoYi-Vue Rust？

RuoYi-Vue Rust 是一个基于 Rust 语言重写的若依后台管理系统。若依（RuoYi）是一个很受欢迎的Java后台管理系统，这个项目用 Rust 重新实现了它的核心功能。

**为什么选择 Rust？**
- ⚡ **高性能**：Rust 的运行速度接近 C/C++，比 Java 快得多
- 🛡️ **内存安全**：编译器在编译时就能发现很多内存错误，避免运行时崩溃
- 🔒 **类型安全**：强类型系统让很多错误在编译阶段就被发现
- 🚀 **并发性能**：Rust 的异步编程模型让并发编程更安全高效

### 项目状态

⚠️ **重要提醒**：
- 项目仍在积极开发中，功能可能不完善
- 正式部署请慎重，作者不承担任何后果
- 欢迎开发者参与贡献！

### 项目来源

- 基于 [abs-admin](https://github.com/rbatis/abs_admin) 开发
- 参考了 [若依](https://gitee.com/y_project) 的 [RuoYi-Vue](https://gitee.com/y_project/RuoYi-Vue) 项目
- 使用 Rust 语言完全重写

---

## 技术栈详解

### 1. **rbatis** (ORM框架)

**什么是 ORM？**
ORM（Object-Relational Mapping）是对象关系映射，让我们可以用面向对象的方式操作数据库，而不用写 SQL 语句。

**rbatis 的作用：**
```rust
// 不用 rbatis，需要手写 SQL
SELECT * FROM sys_user WHERE user_id = 1

// 使用 rbatis，直接操作对象
let user = User::select_by_id(&rb, 1).await?;
```

**为什么要用 rbatis？**
- Rust 中最流行的 ORM 之一
- 支持 MySQL、PostgreSQL 等多种数据库
- 性能优秀，支持异步操作

### 2. **Axum** (Web框架)

**什么是 Web 框架？**
Web 框架提供了开发 Web 应用所需的基础功能，比如路由、请求处理、响应生成等。

**Axum 的特点：**
```rust
// 定义一个简单的 API 路由
Router::new()
    .route("/hello", get(hello_handler))
    .route("/user", post(create_user))
```

**Axum 的优势：**
- 基于 Tokio 异步运行时，性能强大
- 路由系统简单直观
- 支持中间件（用于日志、认证等）
- 类型安全的请求和响应处理

### 3. **JWT** (身份验证)

**什么是 JWT？**
JWT（JSON Web Token）是一种用于身份验证的令牌，用户登录后获得令牌，之后每次请求都带上令牌，服务器验证令牌就知道是谁在请求。

**工作流程：**
1. 用户登录 → 服务器验证成功 → 返回 JWT 令牌
2. 用户后续请求 → 在 HTTP Header 中带上 JWT
3. 服务器验证 JWT → 如果有效，允许访问

**本项目的 JWT 使用：**
- 使用 `jsonwebtoken` 库
- 支持令牌自动续期
- 在配置文件中可设置过期时间

### 4. **rust_xlsxwriter** (Excel导出)

**作用：**
- 将数据导出为 Excel 文件
- 支持设置单元格样式、格式等

**使用场景：**
- 导出用户列表
- 导出角色列表
- 导出各种数据报表

### 5. **MiniJinja** (模板引擎)

**什么是模板引擎？**
模板引擎用于生成代码或文档，本项目用于代码生成功能。

**MiniJinja 的作用：**
```jinja
// 模板示例
pub struct {{ table_name }} {
    {% for column in columns %}
    pub {{ column.name }}: {{ column.rust_type }},
    {% endfor %}
}
```

---

## 项目结构详解

### 整体目录结构

```
ruoyi-rust/
├── Cargo.toml              # 项目配置文件（类似 package.json）
├── Cargo.lock              # 依赖版本锁定文件
├── main.rs                 # 程序入口
├── lib.rs                  # 库文件（对外暴露的接口）
├── application.yml         # 主配置文件
├── application-dev.yml     # 开发环境配置
├── application-prod.yml    # 生产环境配置
├── src/                    # 源代码目录
│   ├── main.rs            # 程序入口
│   ├── lib.rs             # 库文件
│   ├── config/            # 配置相关
│   ├── context.rs         # 全局上下文
│   ├── error.rs           # 错误处理
│   ├── modules/           # 业务模块
│   ├── utils/             # 工具类
│   └── web/               # Web相关（JWT、认证等）
├── tests/                  # 测试文件
├── template/               # 代码生成模板
├── macros/                 # 自定义宏定义
└── mysql-sql/              # 数据库脚本
```

### 详细目录说明

#### 📁 **src/** 源代码目录

**src/config/** - 配置管理
```
config/
├── mod.rs           # 模块声明
├── config.rs        # 配置结构体定义
├── global_constants.rs  # 全局常量
└── log.rs           # 日志配置
```

**说明：**
- `config.rs`: 定义了所有配置项的结构体
- `global_constants.rs`: 存放全局常量，如管理员 ID
- `log.rs`: 配置日志输出格式、级别、滚动策略等

**src/context.rs** - 全局上下文

**什么是全局上下文？**
全局上下文是一个全局可访问的变量，存放数据库连接、服务实例等需要在整个应用中共享的对象。

**包含的内容：**
- 数据库连接池（rbatis 实例）
- 各种 Service 实例（用户服务、角色服务等）
- 缓存服务
- 配置信息

**src/error.rs** - 错误处理

**作用：**
- 定义自定义错误类型
- 提供统一的错误处理机制
- 友好的错误信息展示

**src/modules/** - 业务模块

这是项目的核心，包含所有业务逻辑：

```
modules/
├── system/              # 系统管理模块（最重要）
│   ├── controller/     # 控制器层（处理 HTTP 请求）
│   ├── domain/          # 领域模型（数据库实体）
│   └── service/         # 服务层（业务逻辑）
└── code_gen/            # 代码生成模块
    ├── controller/
    ├── domain/
    └── service/
```

**三层架构说明：**

1. **Controller（控制器层）**
   - 处理 HTTP 请求和响应
   - 参数验证
   - 调用 Service 层处理业务逻辑

2. **Service（服务层）**
   - 实现业务逻辑
   - 调用数据库操作
   - 处理事务

3. **Domain（领域层）**
   - 定义数据库表对应的结构体
   - DTO（数据传输对象）
   - VO（视图对象）

**示例流程：**
```
用户请求 → Controller 接收参数 → Service 处理业务逻辑 → Domain 操作数据库 → 返回结果
```

**src/utils/** - 工具类

包含各种辅助工具：
- `password_encoder.rs`: 密码加密
- `excel_utils.rs`: Excel 导出工具
- `validate_utils.rs`: 数据验证工具
- `ip_util.rs`: IP 地址处理
- 等等...

**src/web/** - Web 相关功能

包含 Web 应用的基础功能：
- JWT 令牌生成和验证
- 用户认证和授权
- 中间件（日志、权限检查等）
- 数据权限控制

#### 📁 **tests/** 测试目录

存放单元测试和集成测试文件。

#### 📁 **macros/** 自定义宏

**什么是宏？**
宏是 Rust 的元编程特性，可以在编译时生成代码。

**本项目宏的作用：**
- 简化重复代码
- 减少样板代码
- 提高代码可读性

#### 📁 **template/** 代码生成模板

存放用于生成代码的 Jinja2 模板文件。

#### 📁 **mysql-sql/** 数据库脚本

存放数据库初始化和更新脚本。

---

## 配置文件说明

### application.yml 主配置文件详解

```yaml
# debug模式
# true: 输出控制台日志
# false: 只输出文件日志（生产环境建议关闭）
debug: true

# 服务器名称
server_name: "ruoyi_rust"

# 服务器监听地址和端口
# 0.0.0.0 表示监听所有网络接口
# 9988 是端口号
server_url: "0.0.0.0:9988"

# API 基础路径
# "/" 表示根路径，所有 API 都从根路径开始
base_api: "/"

# 缓存配置
# "mem": 使用内存缓存
# "redis://127.0.0.1:6379": 使用 Redis 缓存
cache: "redis://127.0.0.1:6379"

# 文件存储配置
# "local": 本地文件系统
# "s3://...": AWS S3 或兼容的存储（如 MinIO）
storage: "local"

# 数据库连接
# 格式: mysql://用户名:密码@地址/数据库名
db_url: "mysql://root:123456@127.0.0.1/ruoyi-rust"

# 数据库连接池大小
# 同时最多保持 32 个数据库连接
db_pool_len: 32

# 获取连接超时时间（秒）
db_pool_timeout: 60

# 日志配置
# 日志滚动策略：按时间滚动
# 可选：[1KB, 1MB, 1GB, 'hour', 'minute', 'day']
log_rolling: "day"

# 日志文件存放目录
log_dir: "target/logs/"

# 日志文件最大大小
log_temp_size: "100MB"

# 日志保留策略
# KeepNum(120): 保留最近 120 个日志文件
# KeepTime(86400): 保留最近 1 天的日志
log_keep_type: "KeepNum(120)"

# 日志压缩格式（可选：空、gzip、zip、lz4）
log_pack_compress: ""

# 日志级别
# 从低到高：trace < debug < info < warn < error
log_level: "debug"

# 日志通道长度
# 100000: 使用有界队列，性能更好
# null: 使用无界队列
log_chan_len: 100000

# JWT 密钥（用于签名和验证令牌）
jwt_secret: "ruoyi_rust_vue3"

# JWT 令牌过期时间（秒）
# 1800 秒 = 30 分钟
jwt_exp: 1800

# JWT 令牌刷新时间（秒）
# 600 秒 = 10 分钟
jwt_refresh_token: 600

# 登录失败重试次数限制
# 0 表示无限制
login_fail_retry: 3

# 登录失败后等待时间（秒）
login_fail_retry_wait_sec: 30

# 垃圾数据超时时间（天）
trash_recycle_days: 90

# 验证码过期时间（分钟）
captcha_expired_min: 2

# Token 过期时间（分钟）
token_expired_min: 60

# 日期时间格式
datetime_format: "YYYY-MM-DD hh:mm:ss"

# 是否启用 IP 地址查询
address_enabled: true

# IP 地址查询服务的 ID 和密钥
# 需要在 apihz.cn 注册获取
apihz_id: "*"
apihz_key: "*"

# 错误信息配置
errors:
  '-1': 未知错误
  empty: 缺少参数
  # ... 其他错误信息

# 文件上传配置
upload_path: D:/ruoyi          # Windows
# upload_path: /home/ruoyi      # Linux
upload_max_size: 50            # 最大 50MB
```

### 环境特定配置

**application-dev.yml** - 开发环境配置
- 覆盖主配置文件中的一些设置
- 适用于本地开发

**application-prod.yml** - 生产环境配置
- 生产环境的专用配置
- 通常关闭 debug、调整日志级别等

**配置优先级：**
环境特定配置 > 主配置文件

---

## 快速开始

### 前置要求

1. **安装 Rust**
```bash
# 访问 https://rustup.rs/ 下载安装程序
# 或使用命令行安装（Windows）
winget install Rustlang.Rustup

# 验证安装
rustc --version
cargo --version
```

2. **安装 MySQL**
- 下载并安装 MySQL 8.0+
- 创建数据库：
```sql
CREATE DATABASE `ruoyi-rust` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
```

3. **安装 Redis（可选，如果使用 Redis 缓存）**
- 下载并安装 Redis
- 启动 Redis 服务：
```bash
redis-server
```

### 数据库初始化

```bash
# 导入数据库脚本
mysql -u root -p ruoyi-rust < rouyi-rust.sql
```

### 修改配置文件

编辑 `application.yml` 或 `application-dev.yml`：

```yaml
# 修改数据库连接信息
db_url: "mysql://root:你的密码@127.0.0.1/ruoyi-rust"

# 如果不使用 Redis，改为内存缓存
cache: "mem"

# 修改文件上传路径
upload_path: D:/ruoyi  # Windows
# upload_path: /home/ruoyi  # Linux
```

### 运行项目

```bash
# 开发环境运行
cargo run

# 或使用自定义的命令（启用详细错误信息）
cargo run-trace

# Release 模式运行（更快，但没有调试信息）
cargo run-release-trace
```

### 访问系统

打开浏览器访问：
```
http://localhost:9988
```

默认管理员账号：
- 用户名：admin
- 密码：admin123
- 管理员 ID：1（可在 `src/config/global_constants.rs` 修改）

---

## 核心概念讲解

### 1. Rust 语言基础概念

#### 变量和可变性
```rust
// let 声明变量，默认不可变
let x = 5;
// x = 10;  // 错误！不能修改

// mut 关键字声明可变变量
let mut y = 10;
y = 20;  // 正确
```

#### 所有权（Ownership）
```rust
// 所有权是 Rust 最独特的特性
// 变量离开作用域时，其内存会被自动释放

{
    let s = String::from("hello");  // s 获得所有权
    // 使用 s
}  // s 离开作用域，内存被释放

// 借用（Borrowing）
let s1 = String::from("hello");
let s2 = &s1;  // 借用 s1（不可变引用）
let len = calculate_length(s2);
println!("{} 的长度是 {}", s1, len);  // s1 仍然可用

// 可变借用
let mut s = String::from("hello");
let s1 = &mut s;  // 可变引用
s1.push_str(", world");
```

#### 错误处理
```rust
// Result<T, E> 表示可能失败的操作
// Ok(T) 表示成功
// Err(E) 表示失败

use std::fs::File;

// 使用 match 处理
let file = File::open("test.txt");
match file {
    Ok(f) => println!("文件打开成功"),
    Err(e) => println!("打开文件失败: {}", e),
}

// 使用 ? 运算符（如果错误，提前返回）
fn read_file() -> Result<String, std::io::Error> {
    let file = File::open("test.txt")?;
    // ... 处理文件
    Ok(String::new())
}
```

### 2. 异步编程

Rust 使用 `async/await` 进行异步编程：

```rust
// 异步函数
async fn fetch_data() -> String {
    // 模拟异步操作
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "数据".to_string()
}

// 调用异步函数
#[tokio::main]  // 异步运行时
async fn main() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

**为什么需要异步？**
- 提高并发性能
- 在等待 I/O 时可以处理其他任务
- Web 服务器可以同时处理多个请求

### 3. Web 请求处理流程

```
1. 用户发送 HTTP 请求
   ↓
2. Axum 接收请求
   ↓
3. 中间层处理（日志、认证、权限检查）
   ↓
4. Router 路由到对应的 Controller
   ↓
5. Controller 接收参数并验证
   ↓
6. Service 处理业务逻辑
   ↓
7. 操作数据库或缓存
   ↓
8. 返回结果
   ↓
9. Controller 返回响应
   ↓
10. 用户收到响应
```

### 4. 路由系统

Axum 的路由系统简洁直观：

```rust
use axum::Router;
use axum::routing::{get, post, put, delete};

Router::new()
    // GET 请求
    .route("/user", get(get_user))

    // POST 请求
    .route("/user", post(create_user))

    // PUT 请求（更新）
    .route("/user", put(update_user))

    // DELETE 请求
    .route("/user", delete(delete_user))

    // 嵌套路由
    .nest("/api/v1", api_v1_router())
```

### 5. 中间件

中间件是在请求处理前后执行的函数：

```rust
// 日志中间件
async fn logging_middleware(
    req: Request,
    next: Next,
) -> Response {
    log::info!("收到请求: {:?}", req.uri());
    let response = next.run(req).await;
    log::info!("请求完成");
    response
}

// 使用中间件
Router::new()
    .layer(axum::middleware::from_fn(logging_middleware))
```

### 6. 数据库操作（rbatis）

```rust
// 查询
let users = User::select_all(&rb).await?;

// 条件查询
let user = User::select_by_column(&rb, "user_name", "admin").await?;

// 分页查询
let page = PageRequest::new(1, 10);
let result = User::select_page(&rb, &page, &wrapper).await?;

// 插入
let mut user = User::new();
user.user_name = "new_user".to_string();
User::insert(&rb, &user).await?;

// 更新
User::update_by_column(&rb, &user, "user_id").await?;

// 删除
User::delete_by_column(&rb, "user_id", &user_id).await?;
```

---

## 开发指南

### 添加新的功能模块

假设要添加一个"公告"功能：

#### 1. 创建数据库表

```sql
CREATE TABLE sys_notice (
    notice_id BIGINT AUTO_INCREMENT PRIMARY KEY,
    notice_title VARCHAR(50) NOT NULL,
    notice_type CHAR(1) DEFAULT '1',
    notice_content LONGTEXT,
    status CHAR(1) DEFAULT '0',
    create_by VARCHAR(64),
    create_time DATETIME,
    update_by VARCHAR(64),
    update_time DATETIME,
    remark VARCHAR(255)
);
```

#### 2. 创建 Domain 层

在 `src/modules/system/domain/` 创建 `sys_notice.rs`：

```rust
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysNotice {
    pub notice_id: Option<i64>,
    pub notice_title: String,
    pub notice_type: Option<String>,
    pub notice_content: Option<String>,
    pub status: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<DateTime>,
    pub remark: Option<String>,
}
```

#### 3. 创建 Service 层

在 `src/modules/system/service/` 创建 `sys_notice_service.rs`：

```rust
use crate::modules::system::domain::sys_notice::SysNotice;
use rbatis::Rbatis;

pub struct SysNoticeService {
    pub rb: Rbatis,
}

impl SysNoticeService {
    pub async fn list(&self) -> Result<Vec<SysNotice>, Error> {
        SysNotice::select_all(&self.rb).await
    }

    pub async fn detail(&self, id: &i64) -> Result<SysNotice, Error> {
        SysNotice::select_by_column(&self.rb, "notice_id", id).await
    }
}
```

#### 4. 创建 Controller 层

在 `src/modules/system/controller/` 创建 `sys_notice_controller.rs`：

```rust
use crate::context::CONTEXT;
use axum::{Json, extract::Path};

pub async fn list() -> impl IntoResponse {
    let result = CONTEXT.sys_notice_service.list().await;
    match result {
        Ok(data) => Json(RespVO::from_data(data)).into_response(),
        Err(e) => Json(RespVO::<String>::from_error(e.to_string())).into_response(),
    }
}

pub async fn detail(Path(id): Path<i64>) -> impl IntoResponse {
    let result = CONTEXT.sys_notice_service.detail(&id).await;
    match result {
        Ok(data) => Json(RespVO::from_data(data)).into_response(),
        Err(e) => Json(RespVO::<String>::from_error(e.to_string())).into_response(),
    }
}
```

#### 5. 注册路由

在 `src/modules/system/controller/mod.rs` 中添加：

```rust
pub mod sys_notice_controller;

pub fn build_system_api() -> Router {
    Router::new()
        // ... 其他路由
        .nest("/notice", notice_api())
}

fn notice_api() -> Router {
    Router::new()
        .route("/list", post(sys_notice_controller::list))
        .route("/{notice_id}", get(sys_notice_controller::detail))
}
```

### 调试技巧

#### 1. 使用日志

```rust
use log::{info, debug, error, warn};

info!("这是一条普通信息");
debug!("这是调试信息（只在 debug 模式显示）");
warn!("这是警告信息");
error!("这是错误信息");
```

#### 2. 使用 println!

临时打印调试信息：

```rust
println!("变量值: {:?}", some_variable);
```

#### 3. 使用调试断点

在 VS Code 中，使用 Rust Analyzer 插件可以设置断点进行调试。

#### 4. 使用 cargo 命令

```bash
# 检查代码（不编译）
cargo check

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 查看依赖关系
cargo tree
```

### 代码规范

#### 1. 命名规范

- **函数和变量**：snake_case（如 `get_user`）
- **结构体和枚举**：PascalCase（如 `User`）
- **常量**：SCREAMING_SNAKE_CASE（如 `MAX_SIZE`）
- **模块**：snake_case（如 `user_service`）

#### 2. 错误处理

```rust
// 使用 ? 运算符简化错误处理
async fn some_function() -> Result<(), Error> {
    let result = some_operation()?;  // 如果错误，提前返回
    Ok(())
}
```

#### 3. 注释

```rust
/// 函数文档注释
///
/// # 参数
///
/// * `param1` - 参数1的说明
///
/// # 返回值
///
/// 返回值的说明
pub fn my_function(param1: i32) -> i32 {
    // 单行注释
    param1 + 1
}
```

---

## 常见问题

### 1. 编译错误：找不到模块

**错误信息：**
```
error[E0433]: failed to resolve: use of undeclared crate or module
```

**解决方法：**
检查 `mod.rs` 文件中是否声明了模块：

```rust
// src/modules/system/mod.rs
pub mod sys_user_controller;
pub mod sys_role_controller;
// ... 其他模块
```

### 2. 数据库连接失败

**错误信息：**
```
Error: Connection refused
```

**解决方法：**
1. 检查 MySQL 是否启动
2. 检查 `application.yml` 中的数据库配置是否正确
3. 检查防火墙设置

### 3. Redis 连接失败

**错误信息：**
```
Error: Connection refused (os error 111)
```

**解决方法：**
1. 启动 Redis 服务：`redis-server`
2. 如果不想使用 Redis，在配置文件中改为：`cache: "mem"`

### 4. 端口被占用

**错误信息：**
```
Error: Os { code: 10048, kind: AddrInUse, message: "Address already in use" }
```

**解决方法：**
1. 更换端口号：在 `application.yml` 中修改 `server_url`
2. 或停止占用端口的程序

### 5. 权限不足

**错误信息：**
```
Error: Access denied
```

**解决方法：**
1. 检查登录用户是否有权限
2. 检查角色和权限配置
3. 确认用户状态是否正常（未被禁用）

### 6. 文件上传失败

**错误信息：**
```
Error: Failed to create directory
```

**解决方法：**
1. 检查 `upload_path` 配置是否正确
2. 确保目录存在且有写入权限
3. 检查磁盘空间

---

## 附录

### Rust 常用命令

```bash
# 创建新项目
cargo new project_name

# 运行项目
cargo run

# 编译项目
cargo build

# 编译 Release 版本
cargo build --release

# 运行测试
cargo test

# 检查代码（快速）
cargo check

# 代码格式化
cargo fmt

# 代码检查（更严格）
cargo clippy

# 查看文档
cargo doc --open

# 更新依赖
cargo update

# 清理编译文件
cargo clean
```

### 项目依赖说明

| 依赖 | 版本 | 用途 |
|------|------|------|
| rbatis | 4.6.14 | ORM 框架 |
| axum | 0.8.4 | Web 框架 |
| tokio | 1 | 异步运行时 |
| serde | 1.0.228 | 序列化/反序列化 |
| redis | 1.0.2 | Redis 客户端 |
| jsonwebtoken | 10.2.0 | JWT 处理 |
| bcrypt | 0.18.0 | 密码加密 |
| captcha | 1.0.0 | 验证码生成 |
| reqwest | 0.13.1 | HTTP 客户端 |

### 学习资源

- **Rust 官方文档**：https://doc.rust-lang.org/
- **Rust 程序设计语言**：https://kaisery.github.io/trpl-zh-cn/
- **Axum 文档**：https://docs.rs/axum/
- **rbatis 文档**：https://rbatis.github.io/rbatis.io/
- **若依官方文档**：http://doc.ruoyi.vip/

---

## 贡献指南

我们欢迎任何有兴趣的开发者参与贡献！

### 如何贡献

1. **完善已实现的功能模块**
   - 优化代码结构
   - 修复 bug
   - 添加单元测试

2. **实现未完成的功能**
   - 查看 issue 列表
   - 选择感兴趣的功能
   - 提交 pull request

3. **编写单元测试**
   - 提高代码覆盖率
   - 确保代码质量

4. **完善文档**
   - 补充代码注释
   - 改进文档说明
   - 提供使用示例

5. **优化性能**
   - 优化数据库查询
   - 改进缓存策略
   - 减少内存占用

### 提交代码规范

1. Fork 项目
2. 创建分支：`git checkout -b feature/your-feature`
3. 提交代码：`git commit -m 'Add some feature'`
4. 推送分支：`git push origin feature/your-feature`
5. 提交 Pull Request

---

## 致谢

感谢以下项目提供的基础和参考：
- [若依](https://gitee.com/y_project) 提供的优秀项目模板
- [abs-admin](https://github.com/rbatis/abs_admin) 提供的 Rust 开发基础
- 所有参与贡献的开发者

---

## 联系方式

- **作者**：wizount
- **邮箱**：2537660666@qq.com
- **微信**：glqqyx

---

## 许可证

请查看项目根目录的 LICENSE 文件了解许可证信息。

---

**祝你在 Rust 的学习和开发之旅中愉快！** 🎉
