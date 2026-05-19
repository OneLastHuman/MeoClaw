# 调试流程

## 热更新开发

正常开发直接 `npm run tauri dev`，Vite 热更新会自动生效，无需手动重启。

**仅在需要彻底重启时**（如 Rust 端改动、端口冲突、多开进程）才手动清理：

```bash
ps aux | grep "[m]eo-claw" | grep -v "\-\-editor" | awk '{print $2}' | xargs kill -9 2>/dev/null || true
```

---

## 日志追踪（状态验证）

### 启动并记录

```bash
npm run tauri dev 2>&1 | tee /tmp/meo_debug.log &
```

### 实时查看

```bash
tail -f /tmp/meo_debug.log
```

### 代码日志点（`src/components/DesktopPet.vue`）

| 时机 | 日志内容 |
|---|---|
| 动画切换 | `[ANIM] switch to {idle\|EnterInput}` |
| 每帧切换 | `[FRAME] {n}` |
| Timer 启停 | `[TIMER] start / stop` |
| 双击检测 | `[CLICK] double-click, interval=...ms` |

---

## 视觉快照

截取桌宠窗口，验证精灵图渲染。

```bash
screencapture -l $(swift -e '
import CoreGraphics
let w = CGWindowListCopyWindowInfo([.optionAll], 0) as! [[String:Any]]
for x in w {
    let name = x["kCGWindowName"] as? String ?? ""
    let owner = x["kCGWindowOwnerName"] as? String ?? ""
    let bounds = x["kCGWindowBounds"] as? [String:Double] ?? [:]
    let width = bounds["Width"] ?? 0
    let height = bounds["Height"] ?? 0
    let isSmallSquare = width > 50 && width < 600 && height > 50 && height < 600 && abs(width-height) < 200
    let onScreen = bounds["X"] ?? 0 > -1 && bounds["Y"] ?? 0 > -1
    if owner == "MeoClaw" && name.isEmpty && isSmallSquare && onScreen {
        print(x["kCGWindowNumber"]!)
        exit(0)
    }
}
print(0)
' 2>/dev/null) -x /tmp/meo_snap.png
```

快照输出：`/tmp/meo_snap.png`

---

## OpenClaw 响应时间测试

### 快速测试脚本

使用 Node.js 内置 WebSocket，直接连接 `ws://127.0.0.1:18789` 进行认证和消息发送。

脚本位置：`src-tauri/examples/openclaw_response_time.cjs`

```bash
node src-tauri/examples/openclaw_response_time.cjs
```

### 测试流程

1. 连接 WebSocket Gateway
2. 处理 `connect.challenge` 获取 nonce
3. 使用 Ed25519 签名认证（device_id = SHA256(public_key)）
4. 订阅 `sessions.subscribe` 和 `sessions.messages.subscribe`
5. 发送测试消息
6. 等待 `session.tool` 事件（phase=start）并记录耗时

---

### 测试数据（2026-04-18）

| 测试 | 消息 | 结果 |
|------|------|------|
| 1 | 查看桌面文件 | 10499 ms |
| 2 | 列出Downloads文件夹内容 | 5002 ms |
| 3 | 当前时间 | Timeout (60s) |
| 4 | 我的IP地址是什么 | 5444 ms |
| 5 | 创建一个测试文件 | Timeout (60s) |

**汇总：成功 3/5，最快 5002ms，最慢 10499ms，平均 6982ms**

### 初步结论

1. OpenClaw 响应时间约 5-10 秒，非即时
2. 超时率 40%，部分请求在 60 秒内未返回工具调用
3. 响应时间波动较大，可能与 AI 处理时间有关
4. 延迟主要发生在 OpenClaw Gateway 端，非桌面宠物应用
