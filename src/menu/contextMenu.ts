import { Menu, MenuItem, Submenu } from '@tauri-apps/api/menu';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import type { AnimState } from '../config/animations';
export type { AnimState } from '../config/animations';

// 切换动画的回调函数类型
export type SwitchAnimCallback = (state: AnimState) => void;

let switchAnimCallback: SwitchAnimCallback | null = null;
let shockClickCount = 0;

// 注册切换动画的回调
export function onSwitchAnim(callback: SwitchAnimCallback) {
  switchAnimCallback = callback;
}

// 构建并显示右键菜单
// showAll: 是否显示调试用的动画切换菜单项
export async function showContextMenu(showAll = false) {
  const shockItem = await MenuItem.new({
    text: 'Shock',
    async action() {
      console.log('[MENU] Shock clicked');
      switchAnimCallback?.('shock');
      const fasterMessages = ['FASTER!', 'FASTER! FASTER!', 'FASTER! FASTER! FASTER!'];
      const message = fasterMessages[shockClickCount % 3];
      shockClickCount++;
      console.log(`[MENU] Shock send: ${message}`);
      await invoke('send_to_openclaw', { message, attachments: [] });
    },
  });

  const startWorkingItem = await MenuItem.new({
    text: 'Start working',
    async action() {
      console.log('[MENU] Start working clicked');
      switchAnimCallback?.('startworking');
    },
  });

  const workingItem = await MenuItem.new({
    text: 'Working',
    async action() {
      console.log('[MENU] Working clicked');
      switchAnimCallback?.('working');
    },
  });

  const workingPreviewItem = await MenuItem.new({
    text: 'Working Preview',
    async action() {
      console.log('[MENU] Working Preview clicked');
      switchAnimCallback?.('workingPreview');
    },
  });

  const enterReceivingItem = await MenuItem.new({
    text: 'Enter Receiving',
    async action() {
      console.log('[MENU] Enter Receiving clicked');
      switchAnimCallback?.('EnterReceiving');
    },
  });

  const receivingItem = await MenuItem.new({
    text: 'Receiving',
    async action() {
      console.log('[MENU] Receiving clicked');
      switchAnimCallback?.('Receiving');
    },
  });

  const responseItem = await MenuItem.new({
    text: 'Response',
    async action() {
      console.log('[MENU] Response clicked');
      switchAnimCallback?.('Response');
    },
  });

  // OpenClaw 命令子菜单
  const newCmdItem = await MenuItem.new({
    text: '/new',
    async action() {
      console.log('[MENU] /new clicked');
      await invoke('send_to_openclaw', { message: '/new', attachments: [] });
    },
  });

  const stopCmdItem = await MenuItem.new({
    text: '/stop',
    async action() {
      console.log('[MENU] /stop clicked');
      await invoke('send_to_openclaw', { message: '/stop', attachments: [] });
    },
  });

  const statusCmdItem = await MenuItem.new({
    text: '/status',
    async action() {
      console.log('[MENU] /status clicked');
      await invoke('send_to_openclaw', { message: '/status', attachments: [] });
    },
  });

  // 创建分身占位符
  const createCloneItem = await MenuItem.new({
    text: '创建分身',
    async action() {
      console.log('[MENU] 创建分身 clicked - TODO: implement clone creation');
    },
  });

  const commandsSubmenu = await Submenu.new({
    text: 'Commands',
    items: [newCmdItem, stopCmdItem, statusCmdItem],
  });

  const debugItems = showAll
    ? [startWorkingItem, workingItem, workingPreviewItem, enterReceivingItem, receivingItem, responseItem, createCloneItem]
    : [];

  const menu = await Menu.new({
    items: [shockItem, ...debugItems, commandsSubmenu],
  });

  const win = getCurrentWindow();
  await menu.popup(undefined, win);
}
