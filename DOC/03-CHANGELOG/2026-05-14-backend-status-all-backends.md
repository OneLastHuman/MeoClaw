# 所有后端独立状态检测

**日期**: 2026-05-14
**类型**: Bug 修复

## 问题

打开选项窗口时，未被选中的后端始终显示"检查中..."，因为 `checkBackendStatus()` 只查询当前选中后端的状态。关闭非活跃后端后，其状态指示器无法更新。

## 修复

### Rust 端

- `backend/manager.rs` — 新增 `check_all_health()` 方法，遍历所有已注册客户端逐一检测健康状态
- `lib.rs` — 新增 `check_all_backends_health` Tauri 命令，注册到 `invoke_handler`

### 前端

- `options.ts` — `checkBackendStatus()` 改用 `check_all_backends_health`，同时更新 OpenClaw 和 Hermes 的指示器
- `options.ts` — `verifySwitch()` 改用 `check_all_backends_health` 精确定位目标后端健康状态

## 涉及文件

- `src-tauri/src/backend/manager.rs`
- `src-tauri/src/lib.rs`
- `src/options.ts`

## 相关文档

- `DOC/01-SPEC/03-接口规范/Tauri命令.md`
- `DOC/01-SPEC/03-接口规范/接口总览.md`
- `DOC/01-SPEC/02-架构设计/多后端管理.md`
- `DOC/01-SPEC/02-架构设计/Rust端架构.md`
- `DOC/01-SPEC/02-架构设计/状态管理.md`
- `DOC/02-PLAN/后端切换UI与验证反馈-2026-04-20.md`
- `DOC/01-SPEC/03-接口规范/窗口通信.md`
