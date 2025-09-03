# Vanish - 阅后即焚的秘密消息服务

Vanish 是一个简单的、自托管的、阅后即焚的秘密消息分享应用。所有消息都在内存中处理，应用重启或消息被读取/过期后，将不在服务器上留下任何痕迹。

## ✨ 核心功能

- **阅后即焚**: 消息在被查看一次后立即销毁。
- **定时销毁**: 用户可以设置消息的过期时间（从1分钟到24小时不等）。
- **无数据库**: 所有秘密都临时存储在应用内存中，服务重启后即消失，保证了隐私性。
- **简洁UI**: 提供一个干净、响应式的单页Web界面，易于使用。
- **API支持**: 提供用于创建和获取秘密的简单JSON API。
- **安全限制**: 消息长度限制为500个字符。

## 🛠️ 技术栈

- **后端**: [Rust](https://www.rust-lang.org/) & [Actix-web](https://actix.rs/)
- **前端**: 原生 HTML, CSS, JavaScript (使用 [Pico.css](https://picocss.com/) 美化界面)
- **构建工具**: Cargo

## 🚢 发布新版本

当您修改了代码并希望发布一个新版本时，可以使用项目提供的自动化脚本。

在运行脚本前，请确保您已经通过 `docker login` 命令登录了 Docker Hub。

**执行脚本:**
```bash
# 发布一个带版本号的镜像 (例如: 1.0.0)
./build_and_push.sh 1.0.0

# 或者，只发布 latest 版本的镜像
./build_and_push.sh
```

脚本会自动完成构建、标记和推送到 Docker Hub 的所有步骤。

## ⚙️ 配置

应用的环境变量配置遵循 `.env` 文件标准。项目提供了一个 `.env.example` 文件作为模板。

**初次配置步骤:**
1.  在项目根目录，从模板复制一份配置文件：
    ```bash
    cp .env.example .env
    ```
2.  打开并修改新建的 `.env` 文件，设置您需要的变量值。

- `APP_PORT`: 设置服务监听的端口。默认值为 `5820`。

`docker-compose` 会自动加载 `.env` 文件。

## 🚀 如何运行

在运行之前，请确保您已经按照上面的“配置”说明，创建了您自己的 `.env` 文件。

### 方式一：使用 Docker Compose (推荐)

### 方式一：使用 Docker Compose (推荐)

这种方式清晰地声明了服务配置，命令简单，推荐在各种环境中使用。

1.  **准备 `docker-compose.yml`:**
    确保项目中有 `docker-compose.yml` 文件。

2.  **拉取并启动容器:**
    ```bash
    docker-compose up -d
    ```
    此命令会自动从 Docker Hub 拉取 `yjiang/vanish:latest` 镜像并在后台启动它。

3.  **访问应用:**
    在浏览器中打开 `http://127.0.0.1:5820`。

4.  **更新版本:**
    ```bash
    docker-compose pull && docker-compose up -d
    ```
    这个命令会拉取最新的镜像并重启服务。

### 方式二：使用 Docker Run (单命令运行)

这种方式直接使用 Docker Hub 上的预构建镜像，适合在服务器上快速部署。

1.  **拉取镜像:**
    首先从 Docker Hub 拉取最新的镜像。
    ```bash
    docker pull yjiang/vanish:latest
    ```

2.  **运行容器:**
    执行以下命令来启动应用。
    ```bash
    docker run -d -p 5820:5820 --restart unless-stopped --name vanish_app yjiang/vanish:latest
    ```
    此命令会在后台启动一个名为 `vanish_app` 的容器。

3.  **访问应用:**
    在浏览器中打开 `http://127.0.0.1:5820`。

### 本地运行 (不使用Docker)

如果您想在本地进行开发和调试，可以遵循以下步骤。

#### 环境要求
确保您已经安装了 [Rust 工具链](https://www.rust-lang.org/tools/install)。

#### 步骤
1.  **克隆仓库:** (如果尚未操作)
    ```bash
    git clone https://github.com/582033/vanish
    cd vanish
    ```

2.  **编译并运行 (开发模式):**
    ```bash
    cargo run
    ```
    您也可以使用 `cargo run --release` 以生产模式运行。

3.  **访问应用:**
    服务器启动后，在浏览器中打开 `http://127.0.0.1:5820` 即可开始使用。

## 👨‍💻 开发指南

本项目遵循一些简单的开发约定，以保证代码质量。

- **运行测试:**
  ```bash
  cargo test
  ```

- **格式化代码:**
  ```bash
  cargo fmt
  ```

- **代码检查 (Lint):**
  ```bash
  cargo clippy
  ```

## 📝 API 接口

应用提供了一组简单的RESTful API。

### 1. 创建秘密

- **Endpoint**: `POST /api/secrets`
- **Request Body** (JSON):
  ```json
  {
    "message": "这是一条秘密消息。",
    "expires_in_secs": 600
  }
  ```
  - `message` (string, 必填): 秘密消息内容。
  - `expires_in_secs` (integer, 可选): 过期时间的秒数。如果未提供，默认为600秒（10分钟）。

- **Success Response** (200 OK):
  ```json
  {
    "id": "some_unique_id"
  }
  ```

### 2. 获取并销毁秘密

- **Endpoint**: `GET /api/secrets/{id}`
- **Success Response** (200 OK):
  ```json
  {
    "message": "这是一条秘密消息。"
  }
  ```
  - **注意**: 成功获取后，该消息将立即从服务器销毁。

- **Error Response** (404 Not Found):
  如果消息ID不存在、已被读取或已过期。
  ```json
  {
    "error": "Oops.. 消息不存在或已被销毁。"
  }
  ```
