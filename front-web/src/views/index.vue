<template>
  <div class="app-container home">
    <section class="project-overview">
      <h1>若依管理t系统-RUST版本</h1>
      <p>基于若依管理框架，后端使用 Rust 开发，提供用户、角色、菜单和操作日志等管理功能。</p>
      <el-link
        href="https://gitee.com/asdtiang_admin/ruoyi-rust"
        target="_blank"
        rel="noopener noreferrer"
        type="primary"
      >
        项目源码
      </el-link>
    </section>

    <el-row :gutter="20" class="home-cards">
      <el-col :xs="24" :sm="24" :md="14">
        <el-card>
          <template #header>
            <h2 class="section-title">技术栈</h2>
          </template>
          <div class="technology">
            <section>
              <h3>后端</h3>
              <ul>
                <li>Rust(1.97.1)、Axum、Tokio</li>
                <li>RBatis、MySQL、Redis</li>
                <li>JWT 认证</li>
              </ul>
            </section>
            <section>
              <h3>前端</h3>
              <ul>
                <li>Vue 3、Element Plus</li>
                <li>Vite、Pinia</li>
              </ul>
            </section>
          </div>
        </el-card>
      </el-col>

      <el-col :xs="24" :sm="24" :md="10">
        <el-card>
          <template #header>
            <h2 class="section-title">联系信息</h2>
          </template>
          <dl class="contact-details">
            <dt>微信</dt>
            <dd>asdtiang</dd>
            <dt>支付宝转账</dt>
            <dd>asdtiangxia@163.com</dd>
          </dl>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup name="Index">
</script>

<style scoped lang="scss">
.home {
  color: var(--el-text-color-regular);
  line-height: 1.6;
}

.project-overview {
  margin-bottom: 24px;

  h1 {
    margin: 0 0 8px;
    color: var(--el-text-color-primary);
    font-size: 26px;
    font-weight: 600;
  }

  p {
    margin: 0 0 12px;
  }
}

.home-cards {
  row-gap: 16px;
}

.section-title,
.technology h3 {
  margin: 0;
  color: var(--el-text-color-primary);
  font-weight: 600;
}

.section-title {
  font-size: 16px;
}

.technology {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(145px, 1fr));
  gap: 16px;

  h3 {
    font-size: 14px;
  }

  ul {
    margin: 4px 0 0;
    padding: 0;
    list-style: none;
  }
}

.contact-details {
  display: grid;
  grid-template-columns: max-content minmax(0, 1fr);
  gap: 8px 16px;
  margin: 0;

  dt {
    color: var(--el-text-color-secondary);
  }

  dd {
    margin: 0;
    overflow-wrap: anywhere;
  }
}
</style>
