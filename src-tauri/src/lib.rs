use tauri::{AppHandle, Emitter, Manager, Position, Size, PhysicalSize, PhysicalPosition, WebviewUrl, WebviewWindowBuilder, async_runtime};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::fs;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub mod backend;
mod openclaw;
use backend::{BackendManager, HermesClient, HermesDiscovery, OpenClawClientAdapter};
use backend::client_trait::Stream;
use openclaw::{Attachment, LifecycleEvent, OpenClawClient};

// 动画状态枚举（需要与前端一致）
#[derive(Clone, serde::Serialize)]
pub enum AnimState {
    Idle,
    Shock,
    EnterInput,
    StartWorking,
    Working,
    WorkingPreview,
    EnterReceiving,
    Receiving,
    Received,
    Response,
}

impl AnimState {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "idle" => Some(AnimState::Idle),
            "shock" => Some(AnimState::Shock),
            "EnterInput" => Some(AnimState::EnterInput),
            "startworking" => Some(AnimState::StartWorking),
            "working" => Some(AnimState::Working),
            "workingPreview" => Some(AnimState::WorkingPreview),
            "EnterReceiving" => Some(AnimState::EnterReceiving),
            "Receiving" => Some(AnimState::Receiving),
            "received" => Some(AnimState::Received),
            "Response" => Some(AnimState::Response),
            _ => None,
        }
    }
}

// AppState for BackendManager
#[derive(Clone)]
pub struct AppState {
    pub backend_manager: Arc<Mutex<BackendManager>>,
}

// 从路径中提取文件名
fn extract_file_name(path: &str) -> String {
    // 从 "/Users/hue/project/src/main.rs" → "main.rs"
    let file_name = std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path);

    // 保留最多 20 字符，超出截断
    if file_name.len() > 20 {
        format!("{}...", &file_name[..17])
    } else {
        file_name.to_string()
    }
}

// 简化 URL
fn simplify_url(url: &str) -> String {
    // "https://github.com/user/repo" → "github.com/user/repo"
    let simplified = url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");

    // 保留最多 25 字符
    if simplified.len() > 25 {
        format!("{}...", &simplified[..22])
    } else {
        simplified.to_string()
    }
}

// 从 JSON 中提取指定 key 的值作为字符串
fn extract_string_arg(args: &serde_json::Value, key: &str) -> Option<String> {
    args.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
}

// 提取工具详情（用于气泡显示）
fn extract_tool_detail(tool_name: &str, args: &serde_json::Value) -> Option<String> {
    // 打印原始 args 用于调试
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
                    // 支持字符串和数字
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

// 切换动画命令
#[tauri::command]
fn switch_animation(state: &str, app: AppHandle) -> Result<(), String> {
    let _ = AnimState::from_str(state).ok_or_else(|| {
        format!("Unknown animation state: {}", state)
    })?;

    // 全局发射事件
    app.emit("switch-animation", state)
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    log::info!("[Rust] switch_animation: {}", state);
    Ok(())
}

// 发送消息到 OpenClaw session（支持附件）
#[tauri::command]
async fn send_to_openclaw(message: String, attachments: Vec<Attachment>, app: AppHandle) -> Result<(), String> {
    let client = app.state::<Arc<OpenClawClient>>();
    let attach_count = attachments.len();
    client.sessions_send(&message, attachments).await?;
    log::info!("[Rust] send_to_openclaw: {} (attachments: {})", message, attach_count);
    Ok(())
}

// 发送消息到 Hermes
#[tauri::command]
async fn send_to_hermes(message: String, app: AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();
    let manager = state.backend_manager.lock().map_err(|e| e.to_string())?;

    let stream = manager.send_message_to_client("hermes", &message)?;

    // 启动任务处理 stream
    let app_for_task = app.clone();
    tokio::spawn(async move {
        handle_hermes_stream(app_for_task, stream).await;
    });

    log::info!("[Hermes] send_message: {}", message);
    Ok(())
}

// 统一消息路由命令 - 根据当前后端自动路由
#[tauri::command]
async fn send_message(message: String, attachments: Vec<Attachment>, app: AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();

    // 获取当前后端（需要先获取，避免锁跨 await）
    let current_backend = {
        let manager = state.backend_manager.lock().map_err(|e| e.to_string())?;
        manager.current().unwrap_or_else(|| "openclaw".to_string())
    };

    let attach_count = attachments.len();
    log::info!("[Router] send_message called - backend: {}, message_len: {}", current_backend, message.len());

    match current_backend.as_str() {
        "hermes" => {
            let message_text = message.clone();
            let app_for_task = app.clone();

            // 使用 spawn_blocking 在阻塞线程中执行 discover()，避免阻塞 I/O 问题
            let (endpoint, api_key) = tokio::task::spawn_blocking(|| {
                let config = backend::HermesDiscovery::discover();
                let endpoint = config.as_ref()
                    .map(|c| c.endpoint.clone())
                    .unwrap_or_else(|| "http://127.0.0.1:8642".to_string());
                let api_key = config.and_then(|c| c.api_key);
                (endpoint, api_key)
            }).await.map_err(|e| format!("spawn_blocking error: {}", e))?;
            log::info!("[Hermes] send_message - endpoint: {}, api_key present: {}", endpoint, api_key.is_some());

            tokio::spawn(async move {
                // 直接在 async task 中处理 Hermes HTTP 请求
                let client = reqwest::Client::new();
                let mut request = client
                    .post(format!("{}/v1/chat/completions", endpoint))
                    .json(&serde_json::json!({
                        "model": "assistant",
                        "messages": [{"role": "user", "content": &message_text}],
                        "stream": true
                    }));

                if let Some(key) = &api_key {
                    request = request.header("Authorization", format!("Bearer {}", key));
                }

                // 发送 working bubble - 显示 "思考中..."
                let working_payload = BubblePayload {
                    tools: vec![ToolCall {
                        name: "hermes".to_string(),
                        status: "running".to_string(),
                        detail: "思考中...".to_string(),
                    }],
                };
                let _ = app_for_task.emit("update-bubble", working_payload);

                let result = request.send().await;
                match result {
                    Ok(resp) => {
                        let status = resp.status();
                        log::info!("[Hermes] HTTP response status: {}", status);

                        // 检查 HTTP 状态码，非 2xx 返回错误
                        if !status.is_success() {
                            let error_body = resp.text().await.unwrap_or_default();
                            log::error!("[Hermes] HTTP error {}: {}", status, error_body);
                            return;
                        }

                        let mut full_text = String::new();
                        let mut stream = resp.bytes_stream();

                        while let Some(chunk_result) = futures_util::StreamExt::next(&mut stream).await {
                            match chunk_result {
                                Ok(bytes) => {
                                    if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                                        log::info!("[Hermes] SSE chunk: {:?}", text);
                                        // 解析 SSE 格式
                                        let mut current_event = String::new();
                                        for line in text.lines() {
                                            let line = line.trim();
                                            if line.is_empty() { continue; }

                                            // 处理 event: xxx 行
                                            if let Some(event_name) = line.strip_prefix("event: ") {
                                                current_event = event_name.to_string();
                                                continue;
                                            }

                                            // 处理 data: xxx 行
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

                                                        log::info!("[Hermes] Tool progress: {} - {}", tool, detail);
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

                                                if data == "[DONE]" {
                                                    // 最终响应
                                                    let empty_payload = BubblePayload { tools: vec![] };
                                                    let _ = app_for_task.emit("update-bubble", empty_payload);

                                                    if let Err(e) = fs::write(state_file(), "Response\n") {
                                                        log::error!("[Hermes] Failed to write state: {}", e);
                                                    }
                                                    let _ = app_for_task.emit("update-response-bubble", serde_json::json!({ "text": full_text }));
                                                    let _ = app_for_task.emit("switch-animation", "Response");
                                                    log::info!("[Hermes] Response complete, text length: {}", full_text.len());
                                                    return;
                                                }

                                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                                                    // 检查是否是最终响应（finish_reason === "stop"）
                                                    let is_final = json["choices"][0]["finish_reason"]
                                                        .as_str()
                                                        .map(|s| s == "stop")
                                                        .unwrap_or(false);

                                                    // 收集 content delta
                                                    if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                                        full_text.push_str(content);
                                                    } else if is_final {
                                                        // 最终响应（无 content delta 但 finish_reason 为 stop）
                                                        let empty_payload = BubblePayload { tools: vec![] };
                                                        let _ = app_for_task.emit("update-bubble", empty_payload);
                                                        if let Err(e) = fs::write(state_file(), "Response\n") {
                                                            log::error!("[Hermes] Failed to write state: {}", e);
                                                        }
                                                        let _ = app_for_task.emit("update-response-bubble", serde_json::json!({ "text": full_text }));
                                                        let _ = app_for_task.emit("switch-animation", "Response");
                                                        log::info!("[Hermes] Response complete (finish_reason=stop), text length: {}", full_text.len());
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("[Hermes] SSE error: {}", e);
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("[Hermes] HTTP error: {}", e);
                        return;
                    }
                }
            });
            log::info!("[Router] send_message via Hermes: {} (attachments: {})", message, attach_count);
        }
        _ => {
            // OpenClaw 使用原有的 WebSocket 方式
            let client = app.state::<Arc<OpenClawClient>>();
            client.sessions_send(&message, attachments).await?;
            log::info!("[Router] send_message via OpenClaw: {} (attachments: {})", message, attach_count);
        }
    }

    Ok(())
}

// 处理 Hermes 流式响应
async fn handle_hermes_stream(
    app_handle: AppHandle,
    mut stream: Box<dyn Stream>,
) {
    let mut full_text = String::new();

    loop {
        match stream.poll_next() {
            Some(Ok(response)) => {
                // 累积文本
                if !response.text_delta.is_empty() {
                    full_text.push_str(&response.text_delta);
                }

                // 如果是最终响应
                if response.is_final {
                    // 写入状态文件，触发 Frontend 切换到 Response 状态
                    if let Err(e) = fs::write(state_file(), "Response\n") {
                        log::error!("[Hermes] Failed to write state file: {}", e);
                    }
                    // 发送 Response 气泡
                    let payload = ResponseBubblePayload {
                        text: full_text.clone(),
                    };
                    let _ = app_handle.emit("update-response-bubble", payload);
                    // 发送状态切换事件
                    let _ = app_handle.emit("switch-animation", "Response");
                    break;
                }
            }
            Some(Err(e)) => {
                log::error!("[Hermes] stream error: {}", e);
                break;
            }
            None => {
                log::info!("[Hermes] stream ended");
                break;
            }
        }
    }
}

// 测试命令 - 用于验证 send_to_openclaw 功能
#[tauri::command]
async fn test_openclaw(app: AppHandle) -> Result<String, String> {
    log::info!("[Rust] test_openclaw called!");
    let client = app.state::<Arc<OpenClawClient>>();
    client.sessions_send("TEST MESSAGE FROM CLI!", vec![]).await?;
    log::info!("[Rust] test_openclaw completed!");
    Ok("Test passed!".to_string())
}

// 测试气泡命令 - 验证 Rust → Vite 事件链路
#[tauri::command]
async fn test_bubble(app: AppHandle) -> Result<String, String> {
    log::info!("[Rust] test_bubble called!");
    let payload = BubblePayload {
        tools: vec![
            ToolCall {
                name: "read_file".to_string(),
                status: "running".to_string(),
                detail: "config.json".to_string(),
            },
            ToolCall {
                name: "write_file".to_string(),
                status: "running".to_string(),
                detail: "settings.json".to_string(),
            },
        ],
    };
    match app.emit("update-bubble", payload) {
        Ok(_) => {
            log::info!("[Rust] test_bubble emit SUCCESS");
            Ok("Bubble sent!".to_string())
        }
        Err(e) => {
            log::error!("[Rust] test_bubble emit failed: {}", e);
            Err(format!("Emit failed: {}", e))
        }
    }
}

// 测试 Response 气泡命令 - 验证 envelope 格式显示
#[tauri::command]
async fn test_response_bubble(app: AppHandle) -> Result<String, String> {
    log::info!("[Rust] test_response_bubble called!");
    let text = "找到了！**工具调用确实记录了时间和状态**！让我总结一下：\n\n---\n\n## 🔍 工具调用记录的信息\n\n[WebChat 2024-01-15 10:30] 每个 `toolResult` 都有 `details` 字段\n\n| 字段 | 内容 | 示例 |\n|------|------|------|\n| `status` | 执行状态 | `completed`, `running` |\n| `durationMs` | 执行时间 | `4624` |\n\n[Discord] 测试一下长文本自动换行和滚动功能是否正常工作";
    let payload = ResponseBubblePayload { text: text.to_string() };
    match app.emit("update-response-bubble", payload) {
        Ok(_) => {
            log::info!("[Rust] test_response_bubble emit SUCCESS");
            Ok("Response bubble sent!".to_string())
        }
        Err(e) => {
            log::error!("[Rust] test_response_bubble emit failed: {}", e);
            Err(format!("Emit failed: {}", e))
        }
    }
}

// 测试 Hermes API - 直接调用并返回结果
#[tauri::command]
async fn test_hermes() -> Result<String, String> {
    log::info!("[Rust] test_hermes called!");

    let config = HermesDiscovery::discover();
    let endpoint = config.as_ref().map(|c| c.endpoint.clone()).unwrap_or_else(|| "http://127.0.0.1:8642".to_string());
    let api_key = config.and_then(|c| c.api_key);

    log::info!("[Hermes] Connecting to {} with api_key={}", endpoint, api_key.is_some());

    let client = reqwest::Client::new();
    let mut request = client
        .post(format!("{}/v1/chat/completions", endpoint))
        .json(&serde_json::json!({
            "model": "assistant",
            "messages": [{"role": "user", "content": "hello"}],
            "stream": false
        }));

    if let Some(key) = &api_key {
        request = request.header("Authorization", format!("Bearer {}", key));
    }

    let resp = request.send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    let body = resp.text().await.map_err(|e| e.to_string())?;
    log::info!("[Hermes] Response status: {}", status);
    log::info!("[Hermes] Response body: {}", body);

    Ok(format!("Hermes status: {}, body len: {}", status, body.len()))
}

// 气泡工具结构
#[derive(Clone, serde::Deserialize, serde::Serialize)]
struct ToolCall {
    name: String,
    status: String,
    detail: String,  // 用空字符串表示无详情，不用 Option
}

// 气泡文案结构
#[derive(Clone, serde::Serialize)]
struct BubblePayload {
    tools: Vec<ToolCall>,
}

// 更新气泡命令
#[tauri::command]
fn update_bubble(tools: Vec<ToolCall>, app: AppHandle) -> Result<(), String> {
    let tool_count = tools.len();
    let payload = BubblePayload { tools };
    app.emit("update-bubble", payload)
        .map_err(|e| format!("Failed to emit bubble event: {}", e))?;
    log::info!("[Rust] update_bubble: sent {} tools to frontend", tool_count);
    Ok(())
}

// 显示气泡窗口（带初始位置，气泡左下角对齐猫咪右上角）
#[tauri::command]
async fn show_bubble_window(main_x: i32, main_y: i32, app: AppHandle) -> Result<(), String> {
    // 如果窗口已存在，直接显示并移动到正确位置
    if let Some(window) = app.get_webview_window("bubble") {
        window.show().map_err(|e: tauri::Error| e.to_string())?;
        // 移动到主窗口右上角（固定尺寸 380x560）
        let new_x = main_x + 180;
        let new_y = main_y - 560;
        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: new_x, y: new_y }))
            .map_err(|e: tauri::Error| e.to_string())?;
        return Ok(());
    }

    // 创建新气泡窗口，直接在目标位置
    let bubble_window = WebviewWindowBuilder::new(
        &app,
        "bubble",
        WebviewUrl::App("bubble.html".into()),
    )
    .title("Bubble")
    .inner_size(380.0, 560.0)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .shadow(false)
    .resizable(false)
    .visible(true)
    .build()
    .map_err(|e| format!("Failed to create bubble window: {}", e))?;

    // 移动到主窗口右上角（气泡左下角对齐猫咪右上角，固定尺寸 380x560）
    let new_x = main_x + 180;
    let new_y = main_y - 560;
    bubble_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: new_x, y: new_y }))
        .map_err(|e: tauri::Error| e.to_string())?;

    log::info!("[Rust] bubble window created at ({}, {})", new_x, new_y);
    Ok(())
}

// 隐藏气泡窗口
#[tauri::command]
async fn hide_bubble_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("bubble") {
        window.hide().map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(())
}

// 更新气泡内容
#[tauri::command]
async fn update_bubble_content(text: String, app: AppHandle) -> Result<(), String> {
    app.emit("bubble-update", serde_json::json!({ "text": text }))
        .map_err(|e| format!("Failed to emit bubble update: {}", e))?;
    Ok(())
}

fn show_options_window(app: &AppHandle) -> Result<(), tauri::Error> {
    if let Some(window) = app.get_webview_window("options") {
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let options_window = WebviewWindowBuilder::new(
        app,
        "options",
        WebviewUrl::App("options.html".into()),
    )
    .title("MeoClaw 选项")
    .inner_size(680.0, 550.0)
    .min_inner_size(620.0, 480.0)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .decorations(false)
    .transparent(true)
    .build()?;

    options_window.set_focus()?;
    Ok(())
}

// 更新气泡窗口位置（跟随主窗口）
// 气泡左下角跟随猫咪窗口右上角
#[tauri::command]
async fn move_bubble_window(main_x: i32, main_y: i32, app: AppHandle) -> Result<(), String> {
    if let Some(bubble) = app.get_webview_window("bubble") {
        // 气泡固定尺寸 380x560，左下角对齐猫咪右上角
        let new_x = main_x + 180;
        let new_y = main_y - 560;
        bubble.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: new_x, y: new_y }))
            .map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(())
}

// 进入 Response 状态时调整窗口尺寸
// 算法：先 resize（macOS 会从当前位置向下扩展），再移动窗口到目标Y
// 目标：保持猫咪 Y 坐标不变
#[tauri::command]
async fn enter_response_mode(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let pos = window.outer_position().map_err(|e| e.to_string())?;
        let size = window.outer_size().map_err(|e| e.to_string())?;
        let cat_y_before = pos.y + size.height as i32 - 180;
        log::info!("[Rust] enter_response: BEFORE pos=({},{}) size={}x{} cat_y={}",
            pos.x, pos.y, size.width, size.height, cat_y_before);

        // 先 resize（macOS 会从当前位置向下扩展）
        window.set_size(Size::Physical(PhysicalSize { width: 1120, height: 1480 }))
            .map_err(|e: tauri::Error| e.to_string())?;
        log::info!("[Rust] enter_response: resize done");

        // 目标窗口Y：保持猫咪Y不变
        // cat_y = new_y + 1480 - 180 = new_y + 1300
        // cat_y = cat_y_before => new_y = cat_y_before - 1300
        let target_y = cat_y_before - 1300;
        log::info!("[Rust] enter_response: calling set_position(y={})", target_y);

        // 再移动窗口到目标Y
        let pos_result = window.set_position(Position::Physical(PhysicalPosition { x: pos.x, y: target_y }));
        log::info!("[Rust] enter_response: set_position result: {:?}", pos_result);

        let pos_after = window.outer_position().map_err(|e| e.to_string())?;
        let size_after = window.outer_size().map_err(|e| e.to_string())?;
        let cat_y_after = pos_after.y + size_after.height as i32 - 180;
        log::info!("[Rust] enter_response: AFTER pos=({},{}) size={}x{} cat_y={} expected={} diff={}",
            pos_after.x, pos_after.y, size_after.width, size_after.height, cat_y_after,
            cat_y_before, cat_y_after - cat_y_before);
    }
    Ok(())
}

// 退出 Response 状态时恢复窗口尺寸
// 算法：先 resize（macOS 会从当前位置向上收缩），再移动窗口到目标Y
// 目标：保持猫咪 Y 坐标不变
#[tauri::command]
async fn exit_response_mode(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let pos = window.outer_position().map_err(|e| e.to_string())?;
        let size = window.outer_size().map_err(|e| e.to_string())?;
        let cat_y_before = pos.y + size.height as i32 - 180;
        log::info!("[Rust] exit_response: BEFORE pos=({},{}) size={}x{} cat_y={}",
            pos.x, pos.y, size.width, size.height, cat_y_before);

        // 先 resize（macOS 会从当前位置向上收缩）
        window.set_size(Size::Physical(PhysicalSize { width: 360, height: 360 }))
            .map_err(|e: tauri::Error| e.to_string())?;
        log::info!("[Rust] exit_response: resize done");

        // 目标窗口Y：保持猫咪Y不变
        // cat_y = new_y + 360 - 180 = new_y + 180
        // cat_y = cat_y_before => new_y = cat_y_before - 180
        let target_y = cat_y_before - 180;
        log::info!("[Rust] exit_response: calling set_position(y={})", target_y);

        // 再移动窗口到目标Y
        let pos_result = window.set_position(Position::Physical(PhysicalPosition { x: pos.x, y: target_y }));
        log::info!("[Rust] exit_response: set_position result: {:?}", pos_result);

        let pos_after = window.outer_position().map_err(|e| e.to_string())?;
        let size_after = window.outer_size().map_err(|e| e.to_string())?;
        let cat_y_after = pos_after.y + size_after.height as i32 - 180;
        log::info!("[Rust] exit_response: AFTER pos=({},{}) size={}x{} cat_y={} expected={} diff={}",
            pos_after.x, pos_after.y, size_after.width, size_after.height, cat_y_after,
            cat_y_before, cat_y_after - cat_y_before);
    }
    Ok(())
}

// 切换后端命令
#[tauri::command]
fn switch_backend(backend: String, state: tauri::State<AppState>) -> Result<(), String> {
    state.backend_manager.lock().map_err(|e| e.to_string())?.switch(&backend)
}

// 获取当前后端命令
#[tauri::command]
fn get_current_backend(state: tauri::State<AppState>) -> Option<String> {
    state.backend_manager.lock().ok()?.current()
}

// 检测所有后端健康状态命令
#[tauri::command]
fn check_all_backends_health(state: tauri::State<AppState>) -> Vec<backend::BackendHealth> {
    match state.backend_manager.lock() {
        Ok(manager) => manager.check_all_health(),
        Err(e) => vec![backend::BackendHealth {
            available: false,
            backend: "unknown".to_string(),
            endpoint: "".to_string(),
            error: Some(e.to_string()),
        }],
    }
}

// 检测后端健康状态命令
#[tauri::command]
fn check_backend_health(state: tauri::State<AppState>) -> backend::BackendHealth {
    match state.backend_manager.lock() {
        Ok(manager) => manager.check_health(),
        Err(e) => backend::BackendHealth {
            available: false,
            backend: "unknown".to_string(),
            endpoint: "".to_string(),
            error: Some(e.to_string()),
        },
    }
}

// Response 气泡结构
#[derive(Clone, serde::Serialize)]
struct ResponseBubblePayload {
    text: String,
}

fn state_file() -> PathBuf {
    std::env::temp_dir().join("meo_anim_state.txt")
}

fn bubble_file() -> PathBuf {
    std::env::temp_dir().join("bubble_content.txt")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![switch_animation, update_bubble, send_to_openclaw, send_to_hermes, send_message, test_openclaw, test_bubble, test_response_bubble, test_hermes, show_bubble_window, hide_bubble_window, update_bubble_content, move_bubble_window, enter_response_mode, exit_response_mode, switch_backend, get_current_backend, check_backend_health, check_all_backends_health])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // 初始化 BackendManager 并注册为 app state
            let backend_manager = Arc::new(Mutex::new(BackendManager::new()));

            // 先执行 Hermes 发现（4 层发现）
            let discovered_instances = {
                if let Ok(manager) = backend_manager.lock() {
                    manager.discover_hermes_all()
                } else {
                    Vec::new()
                }
            };

            log::info!("[Backend] Discovered {} Hermes instances", discovered_instances.len());

            // 根据发现结果创建 HermesClient
            let hermes_client = if let Some(best) = discovered_instances.first() {
                log::info!("[Backend] Using discovered Hermes: {}", best.endpoint);
                HermesClient::from_config(best)
            } else {
                log::info!("[Backend] No Hermes discovered, using default");
                HermesClient::new()
            };

            // 注册后端客户端
            if let Ok(manager) = backend_manager.lock() {
                let _ = manager.register_client(Box::new(OpenClawClientAdapter::new()));
                let _ = manager.register_client(Box::new(hermes_client));
                log::info!("[Backend] Registered OpenClawClientAdapter and HermesClient to BackendManager");
            }

            app.manage(AppState {
                backend_manager: backend_manager.clone(),
            });
            log::info!("[Backend] BackendManager initialized");

            // 初始化状态文件为 idle
            let _ = fs::write(state_file(), "idle\n");
            log::info!("[OpenClaw] Initialized state file");

            // 启动 OpenClaw WebSocket 客户端
            match OpenClawClient::new() {
                Ok((client, msg_rx)) => {
                    let client = Arc::new(client);
                    let client_for_task = client.clone();

                    log::info!("[OpenClaw] Client created, starting connection...");

                    // 设置 lifecycle 回调
                    let cb_app = app_handle.clone();
                    client.set_lifecycle_callback(Arc::new(move |event: LifecycleEvent| {
                        let state = if event.is_start() {
                            "working"
                        } else if event.is_end() || event.is_error() {
                            // 工具执行完成后进入 Response 状态（不是 idle）
                            // 同时清空气泡
                            let payload = BubblePayload {
                                tools: vec![],
                            };
                            let _ = cb_app.emit("update-bubble", payload);
                            "Response"
                        } else {
                            return;
                        };

                        log::info!("[OpenClaw] Lifecycle -> switch to '{}'", state);

                        // 写入状态文件
                        if let Err(e) = fs::write(state_file(), format!("{}\n", state)) {
                            log::error!("[OpenClaw] Failed to write state: {}", e);
                        }

                        // 发送状态切换事件
                        if let Err(e) = cb_app.emit("switch-animation", state) {
                            log::error!("[OpenClaw] Failed to emit state: {}", e);
                        }
                    }));

                    // 设置 tool 回调
                    let cb_app2 = app_handle.clone();
                    client.set_tool_callback(Arc::new(move |tool_name: String, phase: String, args: serde_json::Value| {
                        log::info!("[OpenClaw] Tool event: {} (phase: {}) args: {:?}", tool_name, phase, args);

                        if phase == "start" || phase == "update" {
                            // args 为 null 时不发送更新（保留之前的数据）
                            if !args.is_null() {
                                // 提取工具详情
                                let detail = extract_tool_detail(&tool_name, &args).unwrap_or_default();
                                log::info!("[OpenClaw] Tool detail: {:?}", detail);

                                // 发送气泡更新（直接传递工具信息）
                                let payload = BubblePayload {
                                    tools: vec![ToolCall {
                                        name: tool_name.clone(),
                                        status: phase.clone(),
                                        detail,
                                    }],
                                };
                                log::info!("[OpenClaw] Emitting bubble: {}", serde_json::to_string(&payload).unwrap_or_default());
                                match cb_app2.emit("update-bubble", payload) {
                                    Ok(_) => log::info!("[OpenClaw] Emit bubble SUCCESS"),
                                    Err(e) => log::error!("[OpenClaw] Failed to emit bubble: {}", e),
                                }
                            } else {
                                log::info!("[OpenClaw] Tool update with null args, skipping bubble emit");
                            }
                        } else if phase == "end" {
                            // 工具执行结束，清除气泡
                            let payload = BubblePayload {
                                tools: vec![],
                            };
                            let _ = cb_app2.emit("update-bubble", payload);
                        }
                    }));

                    // 设置 response 回调（用于显示最终回答气泡）
                    let cb_app3 = app_handle.clone();
                    client.set_response_callback(Arc::new(move |full_text: String| {
                        log::info!("[OpenClaw] Response text received: {:.100}", full_text);

                        // 发送 response 气泡更新事件
                        let payload = ResponseBubblePayload {
                            text: full_text,
                        };
                        match cb_app3.emit("update-response-bubble", payload) {
                            Ok(_) => log::info!("[OpenClaw] Emit response bubble SUCCESS"),
                            Err(e) => log::error!("[OpenClaw] Failed to emit response bubble: {}", e),
                        }
                    }));

                    // 将 client 存储到 app 状态中供 command 使用
                    app.manage(client.clone());

                    // 创建 HTTP 轮询的停止信号通道 (mpsc)
                    let (http_stop_tx, http_stop_rx) = tokio::sync::mpsc::channel::<()>(1);
                    let http_stop_tx_for_callback = http_stop_tx.clone();
                    let app_handle_for_callback = app_handle.clone();

                    // 设置连接状态回调 - 协调 HTTP 轮询
                    client.set_connection_status_callback(Arc::new(move |connected: bool| {
                        if !connected {
                            // WebSocket 断开，启动 HTTP 轮询
                            // 先停止可能正在运行的 HTTP 轮询
                            let _ = http_stop_tx_for_callback.try_send(());
                            log::info!("[OpenClaw] WebSocket disconnected, starting HTTP polling...");
                            let _stop_tx = http_stop_tx_for_callback.clone();
                            let app = app_handle_for_callback.clone();
                            // 创建新的停止通道
                            let (_new_stop_tx, new_stop_rx) = tokio::sync::mpsc::channel::<()>(1);
                            async_runtime::spawn(async move {
                                start_http_polling_with_stop(app, new_stop_rx).await;
                            });
                            // 通知前端
                            let _ = app_handle_for_callback.emit("gateway-disconnected", ());
                        } else {
                            // WebSocket 已连接，停止 HTTP 轮询
                            log::info!("[OpenClaw] WebSocket reconnected, stopping HTTP polling...");
                            let _ = http_stop_tx_for_callback.try_send(());
                            // 通知前端
                            let _ = app_handle_for_callback.emit("gateway-connected", ());
                        }
                    }));

                    // 存储 http_stop_tx 以便在 client 错误时停止 HTTP 轮询
                    let http_stop_tx_for_task = http_stop_tx.clone();
                    app.manage(http_stop_tx);

                    async_runtime::spawn(async move {
                        if let Err(e) = client_for_task.run(msg_rx).await {
                            log::error!("[OpenClaw] Client error: {}", e);
                            // WebSocket 连接彻底失败（重连耗尽），启动 HTTP 轮询
                            log::info!("[OpenClaw] Falling back to HTTP polling...");
                            let _ = http_stop_tx_for_task.send(()).await;
                            let (new_stop_tx, new_stop_rx) = tokio::sync::mpsc::channel::<()>(1);
                            let app_clone = app_handle.clone();
                            async_runtime::spawn(start_http_polling_with_stop(app_clone, new_stop_rx));
                            // 通知前端
                            let _ = app_handle.emit("gateway-disconnected", ());
                        }
                    });
                }
                Err(e) => {
                    log::error!("[OpenClaw] Failed to create client: {}", e);
                    // 回退到 HTTP 轮询
                    let (_stop_tx, stop_rx) = tokio::sync::mpsc::channel::<()>(1);
                    let app_clone = app_handle.clone();
                    async_runtime::spawn(start_http_polling_with_stop(app_clone, stop_rx));
                    // 通知前端
                    let _ = app_handle.emit("gateway-disconnected", ());
                }
            }

            // 启动文件监控线程
            let file_app_handle = app.handle().clone();
            std::thread::spawn(move || {
                log::info!("[FileWatcher] Started, watching {}", state_file().display());

                let (tx, rx) = mpsc::channel();

                let mut watcher: RecommendedWatcher = match Watcher::new(
                    move |res: Result<notify::Event, notify::Error>| {
                        if let Ok(event) = res {
                            let _ = tx.send(event);
                        }
                    },
                    Config::default().with_poll_interval(Duration::from_millis(500)),
                ) {
                    Ok(w) => w,
                    Err(e) => {
                        log::error!("[FileWatcher] Failed to create watcher: {}", e);
                        return;
                    }
                };

                if let Err(e) = watcher.watch(&state_file(), RecursiveMode::NonRecursive) {
                    log::error!("[FileWatcher] Failed to watch file: {}", e);
                    return;
                }

                loop {
                    match rx.recv_timeout(Duration::from_millis(500)) {
                        Ok(event) => {
                            if event.kind.is_modify() {
                                if let Ok(content) = fs::read_to_string(state_file()) {
                                    let state = content.trim();
                                    log::info!("[FileWatcher] Detected state change: {}", state);
                                    if let Err(e) = file_app_handle.emit("switch-animation", state) {
                                        log::error!("[FileWatcher] Failed to emit: {}", e);
                                    }
                                }
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(_) => break,
                    }
                }
            });

            // 启动气泡内容文件监听器
            let bubble_app_handle = app.handle().clone();
            std::thread::spawn(move || {
                log::info!("[BubbleFileWatcher] Started, watching {}", bubble_file().display());

                let (tx, rx) = mpsc::channel();

                let mut watcher: RecommendedWatcher = match Watcher::new(
                    move |res: Result<notify::Event, notify::Error>| {
                        if let Ok(event) = res {
                            let _ = tx.send(event);
                        }
                    },
                    Config::default().with_poll_interval(Duration::from_millis(200)),
                ) {
                    Ok(w) => w,
                    Err(e) => {
                        log::error!("[BubbleFileWatcher] Failed to create watcher: {}", e);
                        return;
                    }
                };

                // 初始化文件（如果不存在）
                let _ = fs::write(bubble_file(), "");

                if let Err(e) = watcher.watch(&bubble_file(), RecursiveMode::NonRecursive) {
                    log::error!("[BubbleFileWatcher] Failed to watch file: {}", e);
                    return;
                }

                loop {
                    match rx.recv_timeout(Duration::from_millis(200)) {
                        Ok(event) => {
                            if event.kind.is_modify() {
                                if let Ok(content) = fs::read_to_string(bubble_file()) {
                                    let text = content.trim();
                                    if !text.is_empty() {
                                        log::info!("[BubbleFileWatcher] Detected content change, length={}", text.len());
                                        let payload = ResponseBubblePayload { text: text.to_string() };
                                        if let Err(e) = bubble_app_handle.emit("update-response-bubble", payload) {
                                            log::error!("[BubbleFileWatcher] Failed to emit: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(_) => break,
                    }
                }
            });

            // 监听主窗口移动事件，实时更新气泡窗口位置
            if let Some(main_window) = app.get_webview_window("main") {
                if let Some(bubble_window) = app.get_webview_window("bubble") {
                    let bubble = bubble_window.clone();
                    main_window.on_window_event(move |event| {
                        if let tauri::WindowEvent::Moved(position) = event {
                            // 气泡左下角对齐猫咪右上角：气泡x = 猫x + 180，气泡y = 猫y - 560
                            let new_x = position.x + 180;
                            let new_y = position.y as i32 - 560;
                            let _ = bubble.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                                x: new_x,
                                y: new_y,
                            }));
                        }
                    });
                    log::info!("[Rust] Window move listener registered");
                }

                // === Menu Bar Tray Icon ===
                let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
                let restart_item = MenuItemBuilder::with_id("restart", "Restart").build(app)?;
                let options_item = MenuItemBuilder::with_id("options", "Options").build(app)?;

                // 创建主菜单（选项放在最前面）
                let menu = MenuBuilder::new(app)
                    .item(&options_item)
                    .separator()
                    .item(&restart_item)
                    .item(&quit_item)
                    .build()?;

                let main_win = main_window.clone();
                let app_handle = app.handle().clone();

                // 加载图标（使用 app 的默认窗口图标）
                let icon = app.default_window_icon()
                    .cloned()
                    .expect("No default window icon set. Please configure one in tauri.conf.json");

                let _tray = TrayIconBuilder::new()
                    .icon(icon)
                    .tooltip("MeoClaw")
                    .menu(&menu)
                    .show_menu_on_left_click(false) // 禁止左键显示菜单
                    .on_menu_event(move |app, event| {
                        match event.id().as_ref() {
                            "quit" => {
                                log::info!("[Tray] Quit clicked");
                                app.exit(0);
                            }
                            "restart" => {
                                log::info!("[Tray] Restart clicked");
                                let app_handle = app.clone();
                                std::thread::spawn(move || {
                                    std::thread::sleep(std::time::Duration::from_millis(100));
                                    app_handle.restart();
                                });
                            }
                            "options" => {
                                log::info!("[Tray] Options clicked");
                                if let Err(err) = show_options_window(app) {
                                    log::error!("[Tray] Failed to open options window: {}", err);
                                }
                            }
                            _ => {}
                        }
                    })
                    .on_tray_icon_event(move |_tray, event| {
                        if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                            log::info!("[Tray] Left click - toggle window");
                            if let Some(window) = main_win.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                }
                            }
                        }
                    })
                    .build(app)?;

                log::info!("[Rust] Tray icon created");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// HTTP 轮询备用方案（WebSocket 不可用时使用）
// 当 stop_rx 收到 () 时停止轮询
async fn start_http_polling_with_stop(app_handle: AppHandle, mut stop_rx: tokio::sync::mpsc::Receiver<()>) {
    const GATEWAY_URL: &str = "http://127.0.0.1:18789";
    const POLL_INTERVAL_SECS: u64 = 5;

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut was_connected = false;
    loop {
        tokio::select! {
            _ = stop_rx.recv() => {
                log::info!("[OpenClaw] HTTP polling: received stop signal");
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(POLL_INTERVAL_SECS)) => {
                let is_connected = match client.get(GATEWAY_URL).send().await {
                    Ok(resp) => resp.status().is_success(),
                    Err(_) => false,
                };

                if is_connected {
                    if !was_connected {
                        // 首次连接成功，写入 working 状态
                        if let Err(e) = fs::write(state_file(), "working\n") {
                            log::error!("[OpenClaw] Failed to write state: {}", e);
                        } else {
                            log::info!("[OpenClaw] HTTP polling: Gateway connected, switched to working");
                            // 通知前端 Gateway 恢复连接
                            let _ = app_handle.emit("gateway-reconnected", ());
                        }
                        was_connected = true;
                    }
                    // 持续写入 working 保持活跃状态
                    let _ = fs::write(state_file(), "working\n");
                } else {
                    if was_connected {
                        // 连接断开，写入 idle
                        if let Err(e) = fs::write(state_file(), "idle\n") {
                            log::error!("[OpenClaw] Failed to write state: {}", e);
                        } else {
                            log::info!("[OpenClaw] HTTP polling: Gateway disconnected, switched to idle");
                        }
                        was_connected = false;
                        // 通知前端 Gateway 断开
                        let _ = app_handle.emit("gateway-disconnected", ());
                    }
                }
            }
        }
    }
}
