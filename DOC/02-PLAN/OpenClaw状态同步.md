# OpenClaw 状态同步

## 功能目标

桌宠通过 WebSocket 连接 OpenClaw Gateway，感知后端工作状态（思考中/工作中/完成/出错），并在前端右上角气泡显示对应状态。

## 触发链路

```
OpenClaw Gateway (ws://127.0.0.1:18789)
    │
    ├── lifecycle start
    │       └── lifecycle_callback → 写 /tmp/meo_anim_state.txt
    │                                → FileWatcher 检测到变化
    │                                → emit("switch-animation", "working")
    │                                → 前端切换动画状态 + 右上角气泡轮播
    │
    ├── lifecycle end / error
    │       └── lifecycle_callback → 写 /tmp/meo_anim_state.txt "idle"
    │                                → FileWatcher → emit → 前端切换 idle
    │
    └── tool event (start/update/end)
            └── tool_callback → emit("update-bubble", {texts: [...], interval: ...})
                             → 前端更新气泡文案
```

## 状态映射表

### lifecycle phase → AnimState

| Lifecycle phase | AnimState | 说明 |
|-----------------|-----------|------|
| start           | working   | 后端开始处理 |
| end             | idle      | 后端处理完成 |
| error           | idle      | 后端出错，回到 idle |

### tool_name → 气泡文案

| tool_name       | 气泡文案 |
|-----------------|----------|
| read / read_dir | 查看中··· |
| write / edit    | 写入中··· |
| exec / bash     | 执行命令中··· |
| browser         | 浏览中··· |
| message         | 发送消息中··· |
| sessions_spawn   | 启动分身中··· |
| cron            | 定时任务中··· |
| web_search      | 搜索中··· |
| canvas          | 画布操作中··· |
| code_execution  | 代码执行中··· |
| _               | 工作中··· |

## 前提条件

1. OpenClaw Gateway 运行在 `ws://127.0.0.1:18789`
2. 设备已在 OpenClaw 配对（`~/.openclaw/identity/device.json` 和 `~/.openclaw/devices/paired.json` 存在）
3. Gateway 发送 session.message 事件（stream: "lifecycle" 或 "tool"）

## 调试方法

### 日志关键词

启动后搜索以下关键词确认链路是否通：

```
[OpenClaw] Connected, awaiting challenge...
[OpenClaw] Authenticated successfully!
[OpenClaw] Subscribed to sessions
[FileWatcher] Started, watching /tmp/meo_anim_state.txt
[OpenClaw] Lifecycle: start for session
[OpenClaw] Lifecycle -> switch to 'working'
[FileWatcher] Detected state change: working
```

### 文件监控

状态文件位置：`/tmp/meo_anim_state.txt`

```bash
# 实时查看状态变化
tail -f /tmp/meo_anim_state.txt
```

### 手动切换测试

```bash
# 模拟后端开始工作
echo "working" > /tmp/meo_anim_state.txt

# 模拟后端完成
echo "idle" > /tmp/meo_anim_state.txt
```

## 核心代码路径

| 文件 | 作用 |
|------|------|
| `src-tauri/src/openclaw/client.rs` | WebSocket 连接、认证、事件分发 |
| `src-tauri/src/openclaw/auth.rs` | Ed25519 设备身份 + gateway token 加载 |
| `src-tauri/src/openclaw/protocol.rs` | 协议消息类型定义 |
| `src-tauri/src/lib.rs` | lifecycle/tool 回调设置、FileWatcher 线程 |
| `src/components/DesktopPet.vue` | 前端状态机，接收 switch-animation / update-bubble 事件 |

## 回退机制

如果 WebSocket 连接失败（Gateway 未运行），会回退到 HTTP 轮询模式：
- 每 5 秒轮询 `http://127.0.0.1:18789`
- 连接成功 → 写入 "working"
- 连接断开 → 写入 "idle"
