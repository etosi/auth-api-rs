# auth-api-rs

*`Rust` 后端与 `Axum`、`PostgreSQL` 和电子邮箱验证。*

> - [观看原视频](https://youtu.be/M0wi7V1rP4Y)
> - [原代码仓库](https://github.com/AarambhDevHub/rust-backend-axum)

该存储库包含使用 `Rust` 构建的功能齐全的后端应用程序的源代码，使用 [Axum](https://github.com/tokio-rs/axum) 框架。它包括用户身份验证、邮箱验证和与 `PostgreSQL` 数据库的连接。

## 🛠️ 功能列表

- **用户身份验证**：注册、登录、密码重置功能。
- **邮箱验证**：用户收到电子邮件以验证其帐户。
- **PostgreSQL 集成**：安全存储和管理用户数据。
- **JWT 身份验证**：使用 JSON Web 令牌 (`JWT`) 保护 API 端点。
- **中间件**：实现自定义中间件进行身份验证。
- **使用 Postman 测试**：提供完整的 `Postman` 集合以测试所有 API 端点。

## 🚀 快速入门

### 先决条件

运行此项目，您需要：

- [Rust](https://www.rust-lang.org/) 安装在您的计算机上。
- [PostgreSQL](https://www.postgresql.org/) 本地或远程安装并运行。
- [SQLx-CLI](https://crates.io/crates/sqlx-cli) 用于数据库迁移。
- [Postman](https://www.postman.com/) 用于测试 API 端点。

### 安装

1. **克隆存储库：**

    ```bash
    git clone https://github.com/etosi/auth-api-rs.git
    cd auth-api-rs
    ```

2. **安装依赖项：**

    ```bash
    cargo install --path .
    ```

3. **设置 PostgreSQL：**

    在 `PostgreSQL` 中创建一个新数据库，并使用您的数据库 URL 更新 `.env` 文件。
    示例：

    ```bash
    DATABASE_URL=postgres://user:password@localhost:5432/auth_api
    ```

4. **运行迁移：**

    ```bash
    sqlx database create 
    sqlx migrate run
    ```

    > 安装 `sqlx-cli`：
    >
    > ```rust
    > cargo install sqlx-cli --no-default-features --features native-tls,postgres
    > ```
    >
    > 创建迁移文件：
    >
    > ```bash
    > sqlx migrate add -r users
    > ```

5. **启动服务器：**

    ```bash
    cargo run
    ```

    服务运行地址： `http://127.0.0.1:8000`

## 📬 邮箱验证设置

为了启用电子邮箱验证，您需要配置电子邮件服务提供商。更新 `.env` 文件中的以下环境变量：

```env
SMTP_SERVER=smtp.your-email-provider.com
SMTP_PORT=587
SMTP_USER=your-email@example.com
SMTP_PASSWORD=your-email-password
```

应用程序将在注册后向用户发送验证电子邮件。

### 🧪 使用 Postman 测试 API

您可以使用提供的 `Postman` 集合测试所有 API 端点。[在这里下载 Postman 集合](https://github.com/etosi/auth-api-rs/blob/master/postman_collection.json) 并将其导入 Postman。

API 端点：

- POST `/api/auth/sign-up`: 注册新用户
- POST `/api/auth/sign-in`: 用户登录
- GET `/api/auth/forgot-password`: 请求密码重置
- POST `/api/auth/reset-password`: 重置密码
- GET `/api/auth/verify`: 验证用户电子邮件
- GET `/api/users/me`: 获取当前用户信息（JWT 保护）

## ⚙️ 配置

这个应用需要一个 `.env` 文件来配置。以下是所需的环境变量：

```ini
# -----------------------------------------------------------------------------
# Database (PostgreSQL)
# -----------------------------------------------------------------------------
DATABASE_URL=postgresql://postgres:password@localhost:5432/axum_auth

# -----------------------------------------------------------------------------
# JSON Web Token Credentials
# -----------------------------------------------------------------------------
JWT_SECRET_KEY=my_ultra_secure_jwt_secret_key
JWT_MAX_AGE=60

# -----------------------------------------------------------------------------
# SMTP Server Settings
# -----------------------------------------------------------------------------
SMTP_SERVER=smtp.your-email-provider.com
SMTP_PORT=587                     # Common ports: 587 (TLS), 465 (SSL), 25 (non-secure)
SMTP_USERNAME=your_email@example.com
SMTP_PASSWORD=your_email_password
SMTP_FROM_ADDRESS=no-reply@yourdomain.com
```

## 🎯 功能改进

- 添加基于角色的访问控制 `RBAC` 以支持不同的用户角色：管理员(admin)、普通用户(normal)。
- 增强安全性，增加速率限制和输入验证等附加层。
- 扩展 API 以包括更多功能，例如用户配置文件等。

## ✨ 致谢

- [Axum](https://github.com/tokio-rs/axum) 用于构建强大、快速的 Rust API。
- [PostgreSQL](https://www.postgresql.org/) 用于可靠的数据库管理。
- [SQLx](https://github.com/launchbadge/sqlx) 用于 Rust 中的异步 SQL。
