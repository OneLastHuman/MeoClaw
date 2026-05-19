# Working 状态气泡

> 更新（2026-04-17）：
> 本文中的窗口尺寸和 `adjustWindowForWorking()` 示例属于旧实现。
> 当前版本已将 Working 窗口尺寸并入 `src/stores/petSettings.ts` 的统一 `WindowMode` 模型，且会随 `petScale` 一起缩放。
>
> 当前真实实现请优先参考：
> `DOC/功能/桌宠大小调整功能实现-2026-04-17.md`

## 需求
- 进入 `working` 状态时，右上角显示气泡
- 气泡内容由 Rust 后端驱动，显示工具原始调用信息
- 多个工具同时执行时拼接展示
- 窗口自动调整大小以容纳气泡（与 Response 状态一致）

---

## 窗口尺寸配置

### 气泡定位（CSS 层）

```
窗口坐标系 (360 x 200)
─────────────────────────────→ X
│
│    ┌─────────────────┐
│    │                 │
│    │    气泡         │  ← top: 0, width: 200, height: 20
│    │   (200x20)      │
│    └─────────────────┘
│                      │
│    ┌───────────┐     │
│    │   猫咪     │     │  ← bottom: 0, 180x180
│    └───────────┘     │
└──────────────────────┘
     ↑ left: 160

猫咪在左下角，气泡在顶部右侧，水平方向对齐
```

### 气泡尺寸常量（DesktopPet.vue）

```typescript
const WORKING_BUBBLE_LEFT = 160;     // 气泡左边距
const WORKING_BUBBLE_WIDTH = 200;    // 气泡宽度
const WORKING_BUBBLE_HEIGHT = 20;   // 气泡高度
const CAT_HEIGHT = 180;              // 猫咪高度

// 窗口尺寸计算
const WORKING_WIDTH = WORKING_BUBBLE_LEFT + WORKING_BUBBLE_WIDTH;   // 360
const WORKING_HEIGHT = WORKING_BUBBLE_HEIGHT + CAT_HEIGHT;          // 200
```

### 窗口尺寸

| 状态 | 宽度 | 高度 |
|------|------|------|
| 普通 | 180px | 180px |
| Working | 360px | 200px |

### CSS 气泡样式

```css
.bubble-overlay {
  position: absolute;
  /* left: 160 与猫咪右边缘对齐，top: 0 在窗口顶部 */
  left: 160px;
  top: 0;
  width: 200px;
  height: 20px;
  z-index: 20;
  pointer-events: none;
}
```

### 窗口调整函数

```typescript
async function adjustWindowForWorking(enter: boolean) {
  const win = getCurrentWindow();
  const pos = await win.outerPosition();
  const size = await win.outerSize();

  // 计算当前猫咪顶部的 Y（物理像素）
  const catY = pos.y + size.height - CAT_HEIGHT * 2;

  if (enter) {
    // 进入 Working：放大窗口，保持猫咪位置
    const newHeight = WORKING_HEIGHT * 2;
    const newWidth = WORKING_WIDTH * 2;

    // 目标猫咪Y不变：catY = new_y + new_height - CAT_HEIGHT*2
    const targetY = catY - newHeight + CAT_HEIGHT * 2;

    await win.setSize(new PhysicalSize(newWidth, newHeight));
    await win.setPosition(new PhysicalPosition(pos.x, targetY));
  } else {
    // 退出 Working：缩小窗口，保持猫咪位置
    const newHeight = NORMAL_HEIGHT * 2;
    const newWidth = NORMAL_WIDTH * 2;

    const targetY = catY - newHeight + CAT_HEIGHT * 2;

    await win.setSize(new PhysicalSize(newWidth, newHeight));
    await win.setPosition(new PhysicalPosition(pos.x, targetY));
  }
}
```

### 状态监听

```typescript
watch(() => animState.value, async (state) => {
  if (state === 'Response') {
    await adjustWindowForResponse(true);
  } else if (state === 'working') {
    await adjustWindowForWorking(true);
  } else {
    // 退出时根据之前的状态恢复窗口
    const prevState = previousState.value;
    if (prevState === 'Response') {
      await adjustWindowForResponse(false);
    } else if (prevState === 'working') {
      await adjustWindowForWorking(false);
    }
  }
});
```

---

## 实现

### 1. 工具信息结构

```typescript
interface ToolCall {
  name: string;        // 工具名：read_file, write_file, exec...
  status: string;    // 状态：start, update, end...
  detail: string;    // 详情：命令、路径等，空字符串表示无详情
}
```

### 2. 气泡状态管理（DesktopPet.vue）

```typescript
const bubbleTools = ref<ToolCall[]>([]);  // 当前工具列表
const showBubble = computed(() => animState.value === 'working' && bubbleTools.value.length > 0);

// 拼接显示：多个工具用 | 分隔
// detail 为空字符串时只显示工具名，否则显示 "工具名: 详情"
const bubbleDisplayText = computed(() => {
  if (bubbleTools.value.length === 0) return '';
  return bubbleTools.value
    .map(t => t.detail ? `${t.name}: ${t.detail}` : t.name)
    .join(' | ');
});
```

### 3. Rust 事件驱动

```rust
struct ToolCall {
    name: String,
    status: String,
    detail: String,  // 用空字符串表示无详情，不用 Option
}

struct BubblePayload {
    tools: Vec<ToolCall>,
}
```

### 4. 模板结构

```vue
<div v-if="showBubble" class="bubble-overlay">
  <span class="bubble-text">{{ bubbleDisplayText }}</span>
</div>
```

---

## 工具详情提取（Rust）

```rust
fn extract_tool_detail(tool_name: &str, args: &serde_json::Value) -> Option<String> {
    let args_json = serde_json::to_string(args).unwrap_or_else(|_| "<?>".to_string());
    log::info!("[extract_tool_detail] tool={} args={}", tool_name, args_json);

    let name_lower = tool_name.to_lowercase();
    match name_lower.as_str() {
        "read" | "read_dir" => extract_string_arg(args, "path").map(|p| extract_file_name(&p)),
        "write" | "edit" | "apply_patch" => extract_string_arg(args, "path").map(|p| extract_file_name(&p)),
        "exec" | "bash" => {
            // 优先尝试 command/cmd/script
            if let Some(s) = extract_string_arg(args, "command")
                .or_else(|| extract_string_arg(args, "cmd"))
                .or_else(|| extract_string_arg(args, "script")) {
                return Some(if s.len() > 20 { format!("{}...", &s[..20]) } else { s });
            }
            // args 是数组：["cmd", "arg1", "arg2"]
            if let Some(arr) = args.as_array() {
                let parts: Vec<String> = arr.iter().filter_map(|v| {
                    if let Some(s) = v.as_str() {
                        Some(s.to_string())
                    } else if let Some(n) = v.as_i64() {
                        Some(n.to_string())
                    } else if let Some(n) = v.as_f64() {
                        Some(n.to_string())
                    } else {
                        None
                    }
                }).collect();
                if !parts.is_empty() {
                    let joined = parts.join(" ");
                    return Some(if joined.len() > 20 { format!("{}...", &joined[..20]) } else { joined });
                }
            }
            // args 是对象：尝试获取第一个有值的字段
            if let Some(obj) = args.as_object() {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        if !s.is_empty() {
                            return Some(format!("{}: {}", k, if s.len() > 15 { format!("{}...", &s[..15]) } else { s.to_string() }));
                        }
                    }
                }
            }
            None
        }
        "browser" => extract_string_arg(args, "url")
            .or_else(|| extract_string_arg(args, "action"))
            .map(|u| simplify_url(&u)),
        "sessions_spawn" | "subagents" => extract_string_arg(args, "agent_id")
            .or_else(|| extract_string_arg(args, "name")),
        "cron" => extract_string_arg(args, "description")
            .or_else(|| extract_string_arg(args, "id")),
        "web_search" => extract_string_arg(args, "query"),
        "canvas" => Some("canvas".to_string()),
        "code_execution" => Some("code execution".to_string()),
        "message" => Some("sending message".to_string()),
        _ => {
            // 通用：尝试 args 的各种格式
            if let Some(s) = args.as_str() {
                if !s.is_empty() {
                    return Some(if s.len() > 20 { format!("{}...", &s[..20]) } else { s.to_string() });
                }
            }
            if let Some(arr) = args.as_array() {
                let parts: Vec<String> = arr.iter().filter_map(|v| {
                    if let Some(s) = v.as_str() {
                        Some(s.to_string())
                    } else if let Some(n) = v.as_i64() {
                        Some(n.to_string())
                    } else if let Some(n) = v.as_f64() {
                        Some(n.to_string())
                    } else {
                        None
                    }
                }).collect();
                if !parts.is_empty() {
                    let joined = parts.join(" ");
                    return Some(if joined.len() > 20 { format!("{}...", &joined[..20]) } else { joined });
                }
            }
            if let Some(obj) = args.as_object() {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        if !s.is_empty() {
                            return Some(format!("{}: {}", k, if s.len() > 15 { format!("{}...", &s[..15]) } else { s.to_string() }));
                        }
                    }
                }
            }
            None
        }
    }
}
```

---

## 显示示例

| 场景 | 显示 |
|------|------|
| 单个工具 | `exec: ls -la` |
| 无详情工具 | `canvas` |
| 多个工具 | `read_file: config.json \| write_file: settings.json` |

---

## Bug 修复记录

### Bug: 工具 update 事件覆盖 start 事件的数据

**问题现象**：气泡只显示工具名，不显示详情（如只显示 "exec" 而不是 "exec: ls -la"）

**问题分析**：

1. Rust 端 `extract_tool_detail` 正确提取了详情（如 `"ls -la"`）
2. Rust 端 emit 发送的 JSON 正确：`{"tools":[{"name":"exec","status":"start","detail":"ls -la"}]}`
3. 但前端显示只有工具名，没有详情

**根本原因**：

OpenClaw 发送工具事件时，会先发送 `phase="start"`，然后发送 `phase="update"`。

日志显示：
```
start: args={"command":"ls -la"} → detail="ls -la"  ✓ 正确
update: args=null → detail=""  ✗ 空值覆盖了之前的数据
```

当 `phase="update"` 且 `args=null` 时，代码发送了空字符串的 detail，这个空事件覆盖了之前正确的数据。

**修复方案**：

当 `args.is_null()` 时跳过 emit，保留之前的数据：

```rust
if phase == "start" || phase == "update" {
    // args 为 null 时不发送更新（保留之前的数据）
    if !args.is_null() {
        let detail = extract_tool_detail(&tool_name, &args).unwrap_or_default();
        let payload = BubblePayload {
            tools: vec![ToolCall {
                name: tool_name.clone(),
                status: phase.clone(),
                detail,
            }],
        };
        cb_app2.emit("update-bubble", payload)?;
    }
} else if phase == "end" {
    // 工具执行结束，清除气泡
    let payload = BubblePayload { tools: vec![] };
    cb_app2.emit("update-bubble", payload)?;
}
```

---

## 状态流程

```
idle → startworking → working ← 收到 update-bubble 事件 → 显示工具信息
                            ↓
                      工具结束 → 收到 phase=end + 空 tools[] → 隐藏气泡
```

---

## Rust 端调用

```rust
// 工具开始执行
invoke("update_bubble", {
    tools: [
        { name: "read_file", status: "start", detail: "config.json" }
    ]
})

// 多个工具同时执行
invoke("update_bubble", {
    tools: [
        { name: "read_file", detail: "config.json" },
        { name: "write_file", detail: "settings.json" }
    ]
})

// 工具执行完毕
invoke("update_bubble", { tools: [] })
```

---

## 文件结构

```
src/
└── components/
    └── DesktopPet.vue    # 气泡状态、事件监听、模板

src-tauri/src/
└── lib.rs                # ToolCall 结构、extract_tool_detail、update_bubble 命令
```
