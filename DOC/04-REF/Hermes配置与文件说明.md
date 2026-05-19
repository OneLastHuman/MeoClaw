# Hermes Agent 配置与文件说明

> 来源：~/.hermes/ 目录结构整理

---

## 🏠 用户主目录 (`~/.hermes/`)

| 路径 | 类型 | 说明 |
|------|------|------|
| `config.yaml` | 配置 | 主配置文件（模型、平台、工具、界面偏好） |
| `.env` | 配置 | API 密钥等敏感环境变量 |
| `state.db` | 数据库 | SQLite 会话数据库 |
| `gateway.pid` | 进程ID | Gateway 进程 PID |
| `gateway_state.json` | 状态 | Gateway 运行状态 |
| `channel_directory.json` | 配置 | 消息渠道目录 |
| `processes.json` | 状态 | 进程状态 |
| `auth.json` | 认证 | 认证信息 |
| `models_dev_cache.json` | 缓存 | 模型缓存 |
| `interrupt_debug.log` | 日志 | 中断调试日志 |
| `.hermes_history` | 历史 | 命令历史 |
| `.restart_last_processed.json` | 状态 | 重启状态 |

---

## 📁 子目录

| 路径 | 说明 |
|------|------|
| `skills/` | 用户安装的 Skills（`apple`, `github`, `mlops` 等 30+ 个） |
| `memories/` | 记忆存储 |
| `memories/MEMORY.md` | 长期记忆 |
| `memories/USER.md` | 用户画像 |
| `sessions/` | 所有会话记录（JSON 格式） |
| `logs/` | 运行日志 |
| `audio_cache/` | 语音缓存 |
| `image_cache/` | 图片缓存 |
| `cache/` | 通用缓存 |
| `checkpoints/` | 检查点/快照 |
| `cron/` | 定时任务 |
| `dashboard-themes/` | Dashboard 主题 YAML |
| `hermes-agent/` | **Hermes Agent 源码**（重要！） |
| `web_dist/` | Web UI 编译产物 |
| `hooks/` | 钩子脚本 |
| `sandboxes/` | 沙箱环境 |
| `platforms/` | 平台配置 |
| `plugins/` | 插件 |
| `pastes/` | 临时粘贴数据 |
| `bin/` | 可执行脚本 |

---

## 📖 文档文件

| 路径 | 类型 | 说明 |
|------|------|------|
| `SOUL.md` | 文档 | **机器人的灵魂**——定义人格和行为风格 |
| `memories/MEMORY.md` | 记忆 | 长期记忆 |
| `memories/USER.md` | 记忆 | 用户画像 |

---

## 🧠 hermes-agent 源码 (`~/.hermes/hermes-agent/`)

| 路径 | 类型 | 说明 |
|------|------|------|
| `AGENTS.md` | 📖 文档 | **给 AI 看的开发指南**——项目架构、代码结构 |
| `README.md` | 📖 文档 | 项目说明 |
| `CONTRIBUTING.md` | 📖 文档 | 贡献指南 |
| `RELEASE_v0.10.0.md` | 📖 文档 | 最新版本发布说明 |
| `SECURITY.md` | 📖 文档 | 安全政策 |
| `package.json` | 配置 | Node 依赖 |
| `pyproject.toml` | 配置 | Python 项目配置 |
| `cli.py` | 源码 | CLI 主程序 |
| `web_server.py` | 源码 | Web Dashboard 后端（FastAPI） |
| `hermes_cli/` | 目录 | CLI 子命令实现 |
| `hermes_cli/web_dist/` | 目录 | Web UI 编译后的静态文件 |
| `web/` | 目录 | Web UI 源码（React/Vite） |
| `agent/` | 目录 | Agent 核心逻辑 |
| `gateway/` | 目录 | Gateway 消息网关 |
| `tools/` | 目录 | 工具实现（browser, terminal, file 等） |
| `skills/` | 目录 | 内置 Skills |

---

## 📋 快速索引

| 用途 | 文件 |
|------|------|
| 配置文件 | `~/.hermes/config.yaml` + `~/.hermes/.env` |
| 人格定义 | `~/.hermes/SOUL.md` |
| 记忆存储 | `~/.hermes/memories/` |
| AI 开发文档 | `~/.hermes/hermes-agent/AGENTS.md` |
| Web Dashboard | `web_server.py` + `hermes_cli/web_dist/` |
| CLI 命令入口 | `cli.py`（`hermes` 命令） |

---

## 🔧 常用命令

```bash
# 启动 Web UI
cd ~/.hermes/hermes-agent && source venv/bin/activate && hermes dashboard

# Gateway 管理
hermes gateway start/stop/restart

# 状态查看
hermes status
```
