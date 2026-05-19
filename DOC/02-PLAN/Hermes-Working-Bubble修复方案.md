# Hermes Working Bubble 修复方案

> 创建时间：2026/04/22
> 状态：已完成 ✅

---

## 问题描述

**现象**：Hermes 后端发送消息后，前端能正确切换到 `Response` 状态并显示回复气泡，但没有显示 working bubble（处理中的气泡）。

**预期效果**：Hermes 处理消息时，猫咪旁边应显示 "思考中..." 或工具执行状态的 working bubble。

---

## 问题根因分析

### 链路追踪

```
Frontend 发送消息
    ↓
invoke('send_message')
    ↓
Rust send_message (Hermes 分支)
    ↓
写入状态文件 "working" → Frontend 切换到 working 动画
    ↓
tokio::spawn(async move { ... HTTP 请求 ... })
    ↓
Hermes API 处理中...
    ↓
收到最终响应
    ↓
写入状态文件 "Response" → Frontend 切换到 Response 动画
发送 update-response-bubble → 显示回复气泡
```

### 缺失的环节

| 环节 | OpenClaw | Hermes |
|------|----------|--------|
| 开始处理时发送 bubble | ✅ lifecycle: start → emit update-bubble | ❌ 无 |
| 处理过程中更新 bubble | ✅ tool callback → emit update-bubble | ❌ 无（event 行被忽略） |
| 处理完成时清空 bubble | ✅ lifecycle: end → emit update-bubble { tools: [] } | ❌ 无 |

### 关键发现：Hermes 工具事件格式

Hermes 在工具执行时会发送特殊的 SSE 事件：

```
event: hermes.tool.progress
data: {"tool": "terminal", "emoji": "💻", "label": "ls"}
```

**问题**：原代码只处理 `data:` 前缀的行，忽略了 `event:` 前缀的事件行。

---

## 修复方案

### 修改文件

**文件**: `src-tauri/src/lib.rs`

**位置**: `send_message` 函数的 Hermes 分支（第 244-370 行）

### 修改内容

#### 1. 在 HTTP 请求发送前，发送 working bubble

```rust
// 发送 working bubble - 显示 "思考中..."
let working_payload = BubblePayload {
    tools: vec![ToolCall {
        name: "hermes".to_string(),
        status: "running".to_string(),
        detail: "思考中...".to_string(),
    }],
};
let _ = app_for_task.emit("update-bubble", working_payload);
```

#### 2. 支持 SSE event 行解析

```rust
let mut current_event = String::new();
for line in text.lines() {
    let line = line.trim();
    if line.is_empty() { continue; }

    // 检测 event: 前缀
    if let Some(event_name) = line.strip_prefix("event: ") {
        current_event = event_name.to_string();
        continue;
    }

    if let Some(data) = line.strip_prefix("data: ") {
        // 处理 hermes.tool.progress 事件
        if current_event == "hermes.tool.progress" {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                let tool = json["tool"].as_str().unwrap_or("unknown");
                let emoji = json["emoji"].as_str().unwrap_or("");
                let label = json["label"].as_str().unwrap_or("");
                let detail = if emoji.is_empty() {
                    label.to_string()
                } else {
                    format!("{} {}", emoji, label)
                };

                let payload = BubblePayload {
                    tools: vec![ToolCall {
                        name: tool.to_string(),
                        status: "running".to_string(),
                        detail,
                    }],
                };
                let _ = app_for_task.emit("update-bubble", payload);
            }
            current_event.clear();
            continue;
        }
        // ... 其他处理
    }
}
```

#### 3. 在最终响应时，清空 bubble

```rust
// 清空 working bubble
let empty_payload = BubblePayload { tools: vec![] };
let _ = app_for_task.emit("update-bubble", empty_payload);

// 然后发送 Response 气泡
let _ = app_for_task.emit("update-response-bubble", serde_json::json!({ "text": full_text }));
let _ = app_for_task.emit("switch-animation", "Response");
```

---

## 实际代码位置

| 修改点 | 行号 | 说明 |
|--------|------|------|
| Working bubble 初始化 | 269-277 | HTTP 请求发送前，emit "思考中..." |
| SSE event 行解析 | 290-323 | 支持 `event:` 前缀，解析 `hermes.tool.progress` |
| 工具进度 bubble | 313-320 | 收到工具进度时更新 bubble |
| Bubble 清空 | 329-330, 351-352 | 最终响应时清空 bubble |

---

## 预期效果

**修复前**：
- 发送消息 → 切换到 working 动画 → 没有任何 bubble → 切换到 Response 动画 → 显示回复气泡

**修复后**：
- 发送消息 → 切换到 working 动画 → 显示 "思考中..." bubble → Hermes 执行工具时 bubble 更新为 "📖 AGENTS.md" 等 → 收到回复后 bubble 清空 → 切换到 Response 动画 → 显示回复气泡

---

## 测试验证

### 测试步骤

1. 确保 Hermes 后端正在运行：
```bash
curl http://127.0.0.1:8642/health
```

2. 启动应用：
```bash
npm run tauri dev
```

3. 测试流程：
   - 双击桌面宠物打开输入框
   - 输入触发工具调用的消息，如 "read AGENTS.md"
   - 观察 Working Bubble 是否正确显示

### 验证命令

```bash
# 切换到 working 状态
echo "working" > /tmp/meo_anim_state.txt

# 切换到 idle 状态
echo "idle" > /tmp/meo_anim_state.txt

# 切换到 Response 状态
echo "Response" > /tmp/meo_anim_state.txt
```

---

## 相关代码位置

| 文件 | 行号 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 244-370 | Hermes 分支（已修改） |
| `src/components/DesktopPet.vue` | 88-92 | `showBubble` 条件判断 |
| `src/components/DesktopPet.vue` | 1409-1413 | `update-bubble` 事件监听 |
| `src/components/DesktopPet.vue` | 1535-1537 | bubble-overlay 模板 |

---

## 后续优化建议

- [ ] 支持多个工具同时执行时累积显示
- [ ] 添加 OpenClaw 和 Hermes 的集成测试
