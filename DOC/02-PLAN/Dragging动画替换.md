# Dragging 动画替换

## 变更日期
2026-04-22

## 变更内容

将 dragging 窗口拖拽动画从 5 帧横排替换为 2 帧横排（横二竖一），循环时长 2 秒。

## 变更前配置
| 属性 | 原值 |
|------|------|
| cols | 5 |
| rows | 1 |
| totalFrames | 5 |
| interval | 340ms |
| imagePath | `/anim/edge-placeholders/dragging.png` |

## 变更后配置
| 属性 | 新值 |
|------|------|
| cols | 2 |
| rows | 1 |
| totalFrames | 2 |
| interval | 1000ms |
| imagePath | `/anim/edge-placeholders/dragging.png` |

## 文件操作
- 源文件：`/Users/hue/Documents/dragging.png`
- 目标路径：`public/anim/edge-placeholders/dragging.png`
- 操作：覆盖原文件

## 动画行为
- 2 帧纯循环播放，无 pingpong
- 每帧停留 1 秒，2 秒完成一次循环
