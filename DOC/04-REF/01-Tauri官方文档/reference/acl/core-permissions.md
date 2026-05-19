# 核心权限

_Source: https://v2.tauri.org.cn/reference/acl/core-permissions/_

Tauri 框架核心可使用的所有权限列表。

如果您正在寻找特定 Tauri 插件的权限，请参考文档的 [插件部分](/plugin/)。

## 默认权限

名为“默认权限”的章节

Tauri 中的 `core:default` 权限会自动添加

  * `core:app:default`
  * `core:event:default`
  * `core:image:default`
  * `core:menu:default`
  * `core:path:default`
  * `core:resources:default`
  * `core:tray:default`
  * `core:webview:default`
  * `core:window:default`

## App

名为“App”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:app:default` 包含以下内容

  * `allow-version`
  * `allow-name`
  * `allow-tauri-version`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:app:allow-app-hide`| 启用 `app_hide` 命令，无需任何预配置的作用域。
`core:app:deny-app-hide`| 拒绝 `app_hide` 命令，无需任何预配置的作用域。
`core:app:allow-app-show`| 启用 `app_show` 命令，无需任何预配置的作用域。
`core:app:deny-app-show`| 拒绝 `app_show` 命令，无需任何预配置的作用域。
`core:app:allow-default-window-icon`| 启用 `default_window_icon` 命令，无需任何预配置的作用域。
`core:app:deny-default-window-icon`| 拒绝 `default_window_icon` 命令，无需任何预配置的作用域。
`core:app:allow-name`| 启用 `name` 命令，无需任何预配置的作用域。
`core:app:deny-name`| 拒绝 `name` 命令，无需任何预配置的作用域。
`core:app:allow-set-app-theme`| 启用 `set_app_theme` 命令，无需任何预配置的作用域。
`core:app:deny-set-app-theme`| 拒绝 `set_app_theme` 命令，无需任何预配置的作用域。
`core:app:allow-tauri-version`| 启用 `tauri_version` 命令，无需任何预配置的作用域。
`core:app:deny-tauri-version`| 拒绝 `tauri_version` 命令，无需任何预配置的作用域。
`core:app:allow-version`| 启用 `version` 命令，无需任何预配置的作用域。
`core:app:deny-version`| 拒绝 `version` 命令，无需任何预配置的作用域。

## Event

名为“Event”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:event:default` 包含以下内容

  * `allow-listen`
  * `allow-unlisten`
  * `allow-emit`
  * `allow-emit-to`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:event:allow-emit`| 启用 `emit` 命令，无需任何预配置的作用域。
`core:event:deny-emit`| 拒绝 `emit` 命令，无需任何预配置的作用域。
`core:event:allow-emit-to`| 启用 `emit_to` 命令，无需任何预配置的作用域。
`core:event:deny-emit-to`| 拒绝 `emit_to` 命令，无需任何预配置的作用域。
`core:event:allow-listen`| 启用 `listen` 命令，无需任何预配置的作用域。
`core:event:deny-listen`| 拒绝 `listen` 命令，无需任何预配置的作用域。
`core:event:allow-unlisten`| 启用 `unlisten` 命令，无需任何预配置的作用域。
`core:event:deny-unlisten`| 拒绝 `unlisten` 命令，无需任何预配置的作用域。

## 图像

名为“Image”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:image:default` 包含以下内容

  * `allow-new`
  * `allow-from-bytes`
  * `allow-from-path`
  * `allow-rgba`
  * `allow-size`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:image:allow-from-bytes`| 启用 `from_bytes` 命令，无需任何预配置的作用域。
`core:image:deny-from-bytes`| 拒绝 `from_bytes` 命令，无需任何预配置的作用域。
`core:image:allow-from-path`| 启用 `from_path` 命令，无需任何预配置的作用域。
`core:image:deny-from-path`| 拒绝 `from_path` 命令，无需任何预配置的作用域。
`core:image:allow-new`| 启用 `new` 命令，无需任何预配置的作用域。
`core:image:deny-new`| 拒绝 `new` 命令，无需任何预配置的作用域。
`core:image:allow-rgba`| 启用 `rgba` 命令，无需任何预配置的作用域。
`core:image:deny-rgba`| 拒绝 `rgba` 命令，无需任何预配置的作用域。
`core:image:allow-size`| 启用 `size` 命令，无需任何预配置的作用域。
`core:image:deny-size`| 拒绝 `size` 命令，无需任何预配置的作用域。

## 菜单

标题为“Menu”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:menu:default` 包含以下内容

  * `allow-new`
  * `allow-append`
  * `allow-prepend`
  * `allow-insert`
  * `allow-remove`
  * `allow-remove-at`
  * `allow-items`
  * `allow-get`
  * `allow-popup`
  * `allow-create-default`
  * `allow-set-as-app-menu`
  * `allow-set-as-window-menu`
  * `allow-text`
  * `allow-set-text`
  * `allow-is-enabled`
  * `allow-set-enabled`
  * `allow-set-accelerator`
  * `allow-set-as-windows-menu-for-nsapp`
  * `allow-set-as-help-menu-for-nsapp`
  * `allow-is-checked`
  * `allow-set-checked`
  * `allow-set-icon`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:menu:allow-append`| 启用 `append` 命令，无需任何预配置的作用域。
`core:menu:deny-append`| 拒绝 `append` 命令，无需任何预配置的作用域。
`core:menu:allow-create-default`| 启用 `create_default` 命令，无需任何预配置的作用域。
`core:menu:deny-create-default`| 拒绝 `create_default` 命令，无需任何预配置的作用域。
`core:menu:allow-get`| 启用 `get` 命令，无需任何预配置的作用域。
`core:menu:deny-get`| 拒绝 `get` 命令，无需任何预配置的作用域。
`core:menu:allow-insert`| 启用 `insert` 命令，无需任何预配置的作用域。
`core:menu:deny-insert`| 拒绝 `insert` 命令，无需任何预配置的作用域。
`core:menu:allow-is-checked`| 启用 `is_checked` 命令，无需任何预配置的作用域。
`core:menu:deny-is-checked`| 拒绝 `is_checked` 命令，无需任何预配置的作用域。
`core:menu:allow-is-enabled`| 启用 `is_enabled` 命令，无需任何预配置的作用域。
`core:menu:deny-is-enabled`| 拒绝 `is_enabled` 命令，无需任何预配置的作用域。
`core:menu:allow-items`| 启用 `items` 命令，无需任何预配置的作用域。
`core:menu:deny-items`| 拒绝 `items` 命令，无需任何预配置的作用域。
`core:menu:allow-new`| 启用 `new` 命令，无需任何预配置的作用域。
`core:menu:deny-new`| 拒绝 `new` 命令，无需任何预配置的作用域。
`core:menu:allow-popup`| 启用 `popup` 命令，无需任何预配置的作用域。
`core:menu:deny-popup`| 拒绝 `popup` 命令，无需任何预配置的作用域。
`core:menu:allow-prepend`| 启用 `prepend` 命令，无需任何预配置的作用域。
`core:menu:deny-prepend`| 拒绝 `prepend` 命令，无需任何预配置的作用域。
`core:menu:allow-remove`| 启用 `remove` 命令，无需任何预配置的作用域。
`core:menu:deny-remove`| 拒绝 `remove` 命令，无需任何预配置的作用域。
`core:menu:allow-remove-at`| 启用 `remove_at` 命令，无需任何预配置的作用域。
`core:menu:deny-remove-at`| 拒绝 `remove_at` 命令，无需任何预配置的作用域。
`core:menu:allow-set-accelerator`| 启用 `set_accelerator` 命令，无需任何预配置的作用域。
`core:menu:deny-set-accelerator`| 拒绝 `set_accelerator` 命令，无需任何预配置的作用域。
`core:menu:allow-set-as-app-menu`| 启用 `set_as_app_menu` 命令，无需任何预配置的作用域。
`core:menu:deny-set-as-app-menu`| 拒绝 `set_as_app_menu` 命令，无需任何预配置的作用域。
`core:menu:allow-set-as-help-menu-for-nsapp`| 启用 `set_as_help_menu_for_nsapp` 命令，无需任何预配置的作用域。
`core:menu:deny-set-as-help-menu-for-nsapp`| 拒绝 `set_as_help_menu_for_nsapp` 命令，无需任何预配置的作用域。
`core:menu:allow-set-as-window-menu`| 启用 `set_as_window_menu` 命令，无需任何预配置的作用域。
`core:menu:deny-set-as-window-menu`| 拒绝 `set_as_window_menu` 命令，无需任何预配置的作用域。
`core:menu:allow-set-as-windows-menu-for-nsapp`| 启用 `set_as_windows_menu_for_nsapp` 命令，无需任何预配置的作用域。
`core:menu:deny-set-as-windows-menu-for-nsapp`| 拒绝 `set_as_windows_menu_for_nsapp` 命令，无需任何预配置的作用域。
`core:menu:allow-set-checked`| 启用 `set_checked` 命令，无需任何预配置的作用域。
`core:menu:deny-set-checked`| 拒绝 `set_checked` 命令，无需任何预配置的作用域。
`core:menu:allow-set-enabled`| 启用 `set_enabled` 命令，无需任何预配置的作用域。
`core:menu:deny-set-enabled`| 拒绝 `set_enabled` 命令，无需任何预配置的作用域。
`core:menu:allow-set-icon`| 启用 `set_icon` 命令，无需任何预配置的作用域。
`core:menu:deny-set-icon`| 拒绝 `set_icon` 命令，无需任何预配置的作用域。
`core:menu:allow-set-text`| 启用 `set_text` 命令，无需任何预配置的作用域。
`core:menu:deny-set-text`| 拒绝 `set_text` 命令，无需任何预配置的作用域。
`core:menu:allow-text`| 启用 `text` 命令，无需任何预配置的作用域。
`core:menu:deny-text`| 拒绝 `text` 命令，无需任何预配置的作用域。

## 路径

名为“Path”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:path:default` 包含以下内容

  * `allow-resolve-directory`
  * `allow-resolve`
  * `allow-normalize`
  * `allow-join`
  * `allow-dirname`
  * `allow-extname`
  * `allow-basename`
  * `allow-is-absolute`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:path:allow-basename`| 启用 `basename` 命令，无需任何预配置的作用域。
`core:path:deny-basename`| 拒绝 `basename` 命令，无需任何预配置的作用域。
`core:path:allow-dirname`| 启用 `dirname` 命令，无需任何预配置的作用域。
`core:path:deny-dirname`| 拒绝 `dirname` 命令，无需任何预配置的作用域。
`core:path:allow-extname`| 启用 `extname` 命令，无需任何预配置的作用域。
`core:path:deny-extname`| 拒绝 `extname` 命令，无需任何预配置的作用域。
`core:path:allow-is-absolute`| 启用 `is_absolute` 命令，无需任何预配置的作用域。
`core:path:deny-is-absolute`| 拒绝 `is_absolute` 命令，无需任何预配置的作用域。
`core:path:allow-join`| 启用 `join` 命令，无需任何预配置的作用域。
`core:path:deny-join`| 拒绝 `join` 命令，无需任何预配置的作用域。
`core:path:allow-normalize`| 启用 `normalize` 命令，无需任何预配置的作用域。
`core:path:deny-normalize`| 拒绝 `normalize` 命令，无需任何预配置的作用域。
`core:path:allow-resolve`| 启用 `resolve` 命令，无需任何预配置的作用域。
`core:path:deny-resolve`| 拒绝 `resolve` 命令，无需任何预配置的作用域。
`core:path:allow-resolve-directory`| 启用 `resolve_directory` 命令，无需任何预配置的作用域。
`core:path:deny-resolve-directory`| 拒绝 `resolve_directory` 命令，无需任何预配置的作用域。

## Resources

名为“Resources”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:resources:default` 包含以下内容

  * `allow-close`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:resources:allow-close`| 启用 `close` 命令，无需任何预配置的作用域。
`core:resources:deny-close`| 拒绝 `close` 命令，无需任何预配置的作用域。

## Tray

名为“Tray”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:tray:default` 包含以下内容

  * `allow-new`
  * `allow-get-by-id`
  * `allow-remove-by-id`
  * `allow-set-icon`
  * `allow-set-menu`
  * `allow-set-tooltip`
  * `allow-set-title`
  * `allow-set-visible`
  * `allow-set-temp-dir-path`
  * `allow-set-icon-as-template`
  * `allow-set-show-menu-on-left-click`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:tray:allow-get-by-id`| 启用 `get_by_id` 命令，无需任何预配置的作用域。
`core:tray:deny-get-by-id`| 拒绝 `get_by_id` 命令，无需任何预配置的作用域。
`core:tray:allow-new`| 启用 `new` 命令，无需任何预配置的作用域。
`core:tray:deny-new`| 拒绝 `new` 命令，无需任何预配置的作用域。
`core:tray:allow-remove-by-id`| 启用 `remove_by_id` 命令，无需任何预配置的作用域。
`core:tray:deny-remove-by-id`| 拒绝 `remove_by_id` 命令，无需任何预配置的作用域。
`core:tray:allow-set-icon`| 启用 `set_icon` 命令，无需任何预配置的作用域。
`core:tray:deny-set-icon`| 拒绝 `set_icon` 命令，无需任何预配置的作用域。
`core:tray:allow-set-icon-as-template`| 启用 `set_icon_as_template` 命令，无需任何预配置的作用域。
`core:tray:deny-set-icon-as-template`| 拒绝 `set_icon_as_template` 命令，无需任何预配置的作用域。
`core:tray:allow-set-menu`| 启用 `set_menu` 命令，无需任何预配置的作用域。
`core:tray:deny-set-menu`| 拒绝 `set_menu` 命令，无需任何预配置的作用域。
`core:tray:allow-set-show-menu-on-left-click`| 启用 `set_show_menu_on_left_click` 命令，无需任何预配置的作用域。
`core:tray:deny-set-show-menu-on-left-click`| 拒绝 `set_show_menu_on_left_click` 命令，无需任何预配置的作用域。
`core:tray:allow-set-temp-dir-path`| 启用 `set_temp_dir_path` 命令，无需任何预配置的作用域。
`core:tray:deny-set-temp-dir-path`| 拒绝 `set_temp_dir_path` 命令，无需任何预配置的作用域。
`core:tray:allow-set-title`| 启用 `set_title` 命令，无需任何预配置的作用域。
`core:tray:deny-set-title`| 拒绝 `set_title` 命令，无需任何预配置的作用域。
`core:tray:allow-set-tooltip`| 启用 `set_tooltip` 命令，无需任何预配置的作用域。
`core:tray:deny-set-tooltip`| 拒绝 `set_tooltip` 命令，无需任何预配置的作用域。
`core:tray:allow-set-visible`| 启用 `set_visible` 命令，无需任何预配置的作用域。
`core:tray:deny-set-visible`| 拒绝 `set_visible` 命令，无需任何预配置的作用域。

## Webview

名为“Webview”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:webview:default` 包含以下内容

  * `allow-get-all-webviews`
  * `allow-webview-position`
  * `allow-webview-size`
  * `allow-internal-toggle-devtools`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:webview:allow-clear-all-browsing-data`| 启用 `clear_all_browsing_data` 命令，无需任何预配置的作用域。
`core:webview:deny-clear-all-browsing-data`| 拒绝 `clear_all_browsing_data` 命令，无需任何预配置的作用域。
`core:webview:allow-create-webview`| 启用 `create_webview` 命令，无需任何预配置的作用域。
`core:webview:deny-create-webview`| 拒绝 `create_webview` 命令，无需任何预配置的作用域。
`core:webview:allow-create-webview-window`| 启用 `create_webview_window` 命令，无需任何预配置的作用域。
`core:webview:deny-create-webview-window`| 拒绝 `create_webview_window` 命令，无需任何预配置的作用域。
`core:webview:allow-get-all-webviews`| 启用 `get_all_webviews` 命令，无需任何预配置的作用域。
`core:webview:deny-get-all-webviews`| 拒绝 `get_all_webviews` 命令，无需任何预配置的作用域。
`core:webview:allow-internal-toggle-devtools`| 启用 `internal_toggle_devtools` 命令，无需任何预配置的作用域。
`core:webview:deny-internal-toggle-devtools`| 拒绝 `internal_toggle_devtools` 命令，无需任何预配置的作用域。
`core:webview:allow-print`| 启用 `print` 命令，无需任何预配置的作用域。
`core:webview:deny-print`| 拒绝 `print` 命令，无需任何预配置的作用域。
`core:webview:allow-reparent`| 启用 `reparent` 命令，无需任何预配置的作用域。
`core:webview:deny-reparent`| 拒绝 `reparent` 命令，无需任何预配置的作用域。
`core:webview:allow-set-webview-focus`| 启用 `set_webview_focus` 命令，无需任何预配置的作用域。
`core:webview:deny-set-webview-focus`| 拒绝 `set_webview_focus` 命令，无需任何预配置的作用域。
`core:webview:allow-set-webview-position`| 启用 `set_webview_position` 命令，无需任何预配置的作用域。
`core:webview:deny-set-webview-position`| 拒绝 `set_webview_position` 命令，无需任何预配置的作用域。
`core:webview:allow-set-webview-size`| 启用 `set_webview_size` 命令，无需任何预配置的作用域。
`core:webview:deny-set-webview-size`| 拒绝 `set_webview_size` 命令，无需任何预配置的作用域。
`core:webview:allow-set-webview-zoom`| 启用 `set_webview_zoom` 命令，无需任何预配置的作用域。
`core:webview:deny-set-webview-zoom`| 拒绝 `set_webview_zoom` 命令，无需任何预配置的作用域。
`core:webview:allow-webview-close`| 启用 `webview_close` 命令，无需任何预配置的作用域。
`core:webview:deny-webview-close`| 拒绝 `webview_close` 命令，无需任何预配置的作用域。
`core:webview:allow-webview-hide`| 启用 `webview_hide` 命令，无需任何预配置的作用域。
`core:webview:deny-webview-hide`| 拒绝 `webview_hide` 命令，无需任何预配置的作用域。
`core:webview:allow-webview-position`| 启用 `webview_position` 命令，无需任何预配置的作用域。
`core:webview:deny-webview-position`| 拒绝 `webview_position` 命令，无需任何预配置的作用域。
`core:webview:allow-webview-show`| 启用 `webview_show` 命令，无需任何预配置的作用域。
`core:webview:deny-webview-show`| 拒绝 `webview_show` 命令，无需任何预配置的作用域。
`core:webview:allow-webview-size`| 启用 `webview_size` 命令，无需任何预配置的作用域。
`core:webview:deny-webview-size`| 拒绝 `webview_size` 命令，无需任何预配置的作用域。

## 窗口

名为“Window”的章节

### 默认权限

名为“默认权限”的章节

默认权限 `core:window:default` 包含以下内容

  * `allow-get-all-windows`
  * `allow-scale-factor`
  * `allow-inner-position`
  * `allow-outer-position`
  * `allow-inner-size`
  * `allow-outer-size`
  * `allow-is-fullscreen`
  * `allow-is-minimized`
  * `allow-is-maximized`
  * `allow-is-focused`
  * `allow-is-decorated`
  * `allow-is-resizable`
  * `allow-is-maximizable`
  * `allow-is-minimizable`
  * `allow-is-closable`
  * `allow-is-visible`
  * `allow-is-enabled`
  * `allow-title`
  * `allow-current-monitor`
  * `allow-primary-monitor`
  * `allow-monitor-from-point`
  * `allow-available-monitors`
  * `allow-cursor-position`
  * `allow-theme`
  * `allow-internal-toggle-maximize`

### 权限表

名为“权限表”的章节

标识符| 描述
---|---
`core:window:allow-available-monitors`| 启用 `available_monitors` 命令，无需任何预配置的作用域。
`core:window:deny-available-monitors`| 拒绝 `available_monitors` 命令，无需任何预配置的作用域。
`core:window:allow-center`| 启用 `center` 命令，无需任何预配置的作用域。
`core:window:deny-center`| 拒绝 `center` 命令，无需任何预配置的作用域。
`core:window:allow-close`| 启用 `close` 命令，无需任何预配置的作用域。
`core:window:deny-close`| 拒绝 `close` 命令，无需任何预配置的作用域。
`core:window:allow-create`| 启用 `create` 命令，无需任何预配置的作用域。
`core:window:deny-create`| 拒绝 `create` 命令，无需任何预配置的作用域。
`core:window:allow-current-monitor`| 启用 `current_monitor` 命令，无需任何预配置的作用域。
`core:window:deny-current-monitor`| 拒绝 `current_monitor` 命令，无需任何预配置的作用域。
`core:window:allow-cursor-position`| 启用 `cursor_position` 命令，无需任何预配置的作用域。
`core:window:deny-cursor-position`| 拒绝 `cursor_position` 命令，无需任何预配置的作用域。
`core:window:allow-destroy`| 启用 `destroy` 命令，无需任何预配置的作用域。
`core:window:deny-destroy`| 拒绝 `destroy` 命令，无需任何预配置的作用域。
`core:window:allow-get-all-windows`| 启用 `get_all_windows` 命令，无需任何预配置的作用域。
`core:window:deny-get-all-windows`| 拒绝 `get_all_windows` 命令，无需任何预配置的作用域。
`core:window:allow-hide`| 启用 `hide` 命令，无需任何预配置的作用域。
`core:window:deny-hide`| 拒绝 `hide` 命令，无需任何预配置的作用域。
`core:window:allow-inner-position`| 启用 `inner_position` 命令，无需任何预配置的作用域。
`core:window:deny-inner-position`| 拒绝 `inner_position` 命令，无需任何预配置的作用域。
`core:window:allow-inner-size`| 启用 `inner_size` 命令，无需任何预配置的作用域。
`core:window:deny-inner-size`| 拒绝 `inner_size` 命令，无需任何预配置的作用域。
`core:window:allow-internal-toggle-maximize`| 启用 `internal_toggle_maximize` 命令，无需任何预配置的作用域。
`core:window:deny-internal-toggle-maximize`| 拒绝 `internal_toggle_maximize` 命令，无需任何预配置的作用域。
`core:window:allow-is-closable`| 启用 `is_closable` 命令，无需任何预配置的作用域。
`core:window:deny-is-closable`| 拒绝 `is_closable` 命令，无需任何预配置的作用域。
`core:window:allow-is-decorated`| 启用 `is_decorated` 命令，无需任何预配置的作用域。
`core:window:deny-is-decorated`| 拒绝 `is_decorated` 命令，无需任何预配置的作用域。
`core:window:allow-is-enabled`| 启用 `is_enabled` 命令，无需任何预配置的作用域。
`core:window:deny-is-enabled`| 拒绝 `is_enabled` 命令，无需任何预配置的作用域。
`core:window:allow-is-focused`| 启用 `is_focused` 命令，无需任何预配置的作用域。
`core:window:deny-is-focused`| 拒绝 `is_focused` 命令，无需任何预配置的作用域。
`core:window:allow-is-fullscreen`| 启用 `is_fullscreen` 命令，无需任何预配置的作用域。
`core:window:deny-is-fullscreen`| 拒绝 `is_fullscreen` 命令，无需任何预配置的作用域。
`core:window:allow-is-maximizable`| 启用 `is_maximizable` 命令，无需任何预配置的作用域。
`core:window:deny-is-maximizable`| 拒绝 `is_maximizable` 命令，无需任何预配置的作用域。
`core:window:allow-is-maximized`| 启用 `is_maximized` 命令，无需任何预配置的作用域。
`core:window:deny-is-maximized`| 拒绝 `is_maximized` 命令，无需任何预配置的作用域。
`core:window:allow-is-minimizable`| 启用 `is_minimizable` 命令，无需任何预配置的作用域。
`core:window:deny-is-minimizable`| 拒绝 `is_minimizable` 命令，无需任何预配置的作用域。
`core:window:allow-is-minimized`| 启用 `is_minimized` 命令，无需任何预配置的作用域。
`core:window:deny-is-minimized`| 拒绝 `is_minimized` 命令，无需任何预配置的作用域。
`core:window:allow-is-resizable`| 启用 `is_resizable` 命令，无需任何预配置的作用域。
`core:window:deny-is-resizable`| 拒绝 `is_resizable` 命令，无需任何预配置的作用域。
`core:window:allow-is-visible`| 启用 `is_visible` 命令，无需任何预配置的作用域。
`core:window:deny-is-visible`| 拒绝 `is_visible` 命令，无需任何预配置的作用域。
`core:window:allow-maximize`| 启用 `maximize` 命令，无需任何预配置的作用域。
`core:window:deny-maximize`| 拒绝 `maximize` 命令，无需任何预配置的作用域。
`core:window:allow-minimize`| 启用 `minimize` 命令，无需任何预配置的作用域。
`core:window:deny-minimize`| 拒绝 `minimize` 命令，无需任何预配置的作用域。
`core:window:allow-monitor-from-point`| 启用 `monitor_from_point` 命令，无需任何预配置的作用域。
`core:window:deny-monitor-from-point`| 拒绝 `monitor_from_point` 命令，无需任何预配置的作用域。
`core:window:allow-outer-position`| 启用 `outer_position` 命令，无需任何预配置的作用域。
`core:window:deny-outer-position`| 拒绝 `outer_position` 命令，无需任何预配置的作用域。
`core:window:allow-outer-size`| 启用 `outer_size` 命令，无需任何预配置的作用域。
`core:window:deny-outer-size`| 拒绝 `outer_size` 命令，无需任何预配置的作用域。
`core:window:allow-primary-monitor`| 启用 `primary_monitor` 命令，无需任何预配置的作用域。
`core:window:deny-primary-monitor`| 拒绝 `primary_monitor` 命令，无需任何预配置的作用域。
`core:window:allow-request-user-attention`| 启用 `request_user_attention` 命令，无需任何预配置的作用域。
`core:window:deny-request-user-attention`| 拒绝 `request_user_attention` 命令，无需任何预配置的作用域。
`core:window:allow-scale-factor`| 启用 `scale_factor` 命令，无需任何预配置的作用域。
`core:window:deny-scale-factor`| 拒绝 `scale_factor` 命令，无需任何预配置的作用域。
`core:window:allow-set-always-on-bottom`| 启用 `set_always_on_bottom` 命令，无需任何预配置的作用域。
`core:window:deny-set-always-on-bottom`| 拒绝 `set_always_on_bottom` 命令，无需任何预配置的作用域。
`core:window:allow-set-always-on-top`| 启用 `set_always_on_top` 命令，无需任何预配置的作用域。
`core:window:deny-set-always-on-top`| 拒绝 `set_always_on_top` 命令，无需任何预配置的作用域。
`core:window:allow-set-closable`| 启用 `set_closable` 命令，无需任何预配置的作用域。
`core:window:deny-set-closable`| 拒绝 `set_closable` 命令，无需任何预配置的作用域。
`core:window:allow-set-content-protected`| 启用 `set_content_protected` 命令，无需任何预配置的作用域。
`core:window:deny-set-content-protected`| 拒绝 `set_content_protected` 命令，无需任何预配置的作用域。
`core:window:allow-set-cursor-grab`| 启用 `set_cursor_grab` 命令，无需任何预配置的作用域。
`core:window:deny-set-cursor-grab`| 拒绝 `set_cursor_grab` 命令，无需任何预配置的作用域。
`core:window:allow-set-cursor-icon`| 启用 `set_cursor_icon` 命令，无需任何预配置的作用域。
`core:window:deny-set-cursor-icon`| 拒绝 `set_cursor_icon` 命令，无需任何预配置的作用域。
`core:window:allow-set-cursor-position`| 启用 `set_cursor_position` 命令，无需任何预配置的作用域。
`core:window:deny-set-cursor-position`| 拒绝 `set_cursor_position` 命令，无需任何预配置的作用域。
`core:window:allow-set-cursor-visible`| 启用 `set_cursor_visible` 命令，无需任何预配置的作用域。
`core:window:deny-set-cursor-visible`| 拒绝 `set_cursor_visible` 命令，无需任何预配置的作用域。
`core:window:allow-set-decorations`| 启用 `set_decorations` 命令，无需任何预配置的作用域。
`core:window:deny-set-decorations`| 拒绝 `set_decorations` 命令，无需任何预配置的作用域。
`core:window:allow-set-effects`| 启用 `set_effects` 命令，无需任何预配置的作用域。
`core:window:deny-set-effects`| 拒绝 `set_effects` 命令，无需任何预配置的作用域。
`core:window:allow-set-enabled`| 启用 `set_enabled` 命令，无需任何预配置的作用域。
`core:window:deny-set-enabled`| 拒绝 `set_enabled` 命令，无需任何预配置的作用域。
`core:window:allow-set-focus`| 启用 `set_focus` 命令，无需任何预配置的作用域。
`core:window:deny-set-focus`| 拒绝 `set_focus` 命令，无需任何预配置的作用域。
`core:window:allow-set-fullscreen`| 启用 `set_fullscreen` 命令，无需任何预配置的作用域。
`core:window:deny-set-fullscreen`| 拒绝 `set_fullscreen` 命令，无需任何预配置的作用域。
`core:window:allow-set-icon`| 启用 `set_icon` 命令，无需任何预配置的作用域。
`core:window:deny-set-icon`| 拒绝 `set_icon` 命令，无需任何预配置的作用域。
`core:window:allow-set-ignore-cursor-events`| 启用 `set_ignore_cursor_events` 命令，无需任何预配置的作用域。
`core:window:deny-set-ignore-cursor-events`| 拒绝 `set_ignore_cursor_events` 命令，无需任何预配置的作用域。
`core:window:allow-set-max-size`| 启用 `set_max_size` 命令，无需任何预配置的作用域。
`core:window:deny-set-max-size`| 拒绝 `set_max_size` 命令，无需任何预配置的作用域。
`core:window:allow-set-maximizable`| 启用 `set_maximizable` 命令，无需任何预配置的作用域。
`core:window:deny-set-maximizable`| 拒绝 `set_maximizable` 命令，无需任何预配置的作用域。
`core:window:allow-set-min-size`| 启用 `set_min_size` 命令，无需任何预配置的作用域。
`core:window:deny-set-min-size`| 拒绝 `set_min_size` 命令，无需任何预配置的作用域。
`core:window:allow-set-minimizable`| 启用 `set_minimizable` 命令，无需任何预配置的作用域。
`core:window:deny-set-minimizable`| 拒绝 `set_minimizable` 命令，无需任何预配置的作用域。
`core:window:allow-set-position`| 启用 `set_position` 命令，无需任何预配置的作用域。
`core:window:deny-set-position`| 拒绝 `set_position` 命令，无需任何预配置的作用域。
`core:window:allow-set-progress-bar`| 启用 `set_progress_bar` 命令，无需任何预配置的作用域。
`core:window:deny-set-progress-bar`| 拒绝 `set_progress_bar` 命令，无需任何预配置的作用域。
`core:window:allow-set-resizable`| 启用 `set_resizable` 命令，无需任何预配置的作用域。
`core:window:deny-set-resizable`| 拒绝 `set_resizable` 命令，无需任何预配置的作用域。
`core:window:allow-set-shadow`| 启用 `set_shadow` 命令，无需任何预配置的作用域。
`core:window:deny-set-shadow`| 拒绝 `set_shadow` 命令，无需任何预配置的作用域。
`core:window:allow-set-size`| 启用 `set_size` 命令，无需任何预配置的作用域。
`core:window:deny-set-size`| 拒绝 `set_size` 命令，无需任何预配置的作用域。
`core:window:allow-set-size-constraints`| 启用 `set_size_constraints` 命令，无需任何预配置的作用域。
`core:window:deny-set-size-constraints`| 拒绝 `set_size_constraints` 命令，无需任何预配置的作用域。
`core:window:allow-set-skip-taskbar`| 启用 `set_skip_taskbar` 命令，无需任何预配置的作用域。
`core:window:deny-set-skip-taskbar`| 拒绝 `set_skip_taskbar` 命令，无需任何预配置的作用域。
`core:window:allow-set-theme`| 启用 `set_theme` 命令，无需任何预配置的作用域。
`core:window:deny-set-theme`| 拒绝 `set_theme` 命令，无需任何预配置的作用域。
`core:window:allow-set-title`| 启用 `set_title` 命令，无需任何预配置的作用域。
`core:window:deny-set-title`| 拒绝 `set_title` 命令，无需任何预配置的作用域。
`core:window:allow-set-title-bar-style`| 启用 `set_title_bar_style` 命令，无需任何预配置的作用域。
`core:window:deny-set-title-bar-style`| 拒绝 `set_title_bar_style` 命令，无需任何预配置的作用域。
`core:window:allow-set-visible-on-all-workspaces`| 启用 `set_visible_on_all_workspaces` 命令，无需任何预配置的作用域。
`core:window:deny-set-visible-on-all-workspaces`| 拒绝 `set_visible_on_all_workspaces` 命令，无需任何预配置的作用域。
`core:window:allow-show`| 启用 `show` 命令，无需任何预配置的作用域。
`core:window:deny-show`| 拒绝 `show` 命令，无需任何预配置的作用域。
`core:window:allow-start-dragging`| 启用 `start_dragging` 命令，无需任何预配置的作用域。
`core:window:deny-start-dragging`| 拒绝 `start_dragging` 命令，无需任何预配置的作用域。
`core:window:allow-start-resize-dragging`| 启用 `start_resize_dragging` 命令，无需任何预配置的作用域。
`core:window:deny-start-resize-dragging`| 拒绝 `start_resize_dragging` 命令，无需任何预配置的作用域。
`core:window:allow-theme`| 启用 `theme` 命令，无需任何预配置的作用域。
`core:window:deny-theme`| 拒绝 `theme` 命令，无需任何预配置的作用域。
`core:window:allow-title`| 启用 `title` 命令，无需任何预配置的作用域。
`core:window:deny-title`| 拒绝 `title` 命令，无需任何预配置的作用域。
`core:window:allow-toggle-maximize`| 启用 `toggle_maximize` 命令，无需任何预配置的作用域。
`core:window:deny-toggle-maximize`| 拒绝 `toggle_maximize` 命令，无需任何预配置的作用域。
`core:window:allow-unmaximize`| 启用 `unmaximize` 命令，无需任何预配置的作用域。
`core:window:deny-unmaximize`| 拒绝 `unmaximize` 命令，无需任何预配置的作用域。
`core:window:allow-unminimize`| 启用 `unminimize` 命令，无需任何预配置的作用域。
`core:window:deny-unminimize`| 拒绝 `unminimize` 命令，无需任何预配置的作用域。