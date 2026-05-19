// 动画配置类型
export interface AnimationConfig {
  name: string;           // 动画名称
  cols: number;           // 精灵图列数
  rows: number;           // 精灵图行数
  totalFrames: number;    // 总帧数
  interval: number;       // 每帧间隔(ms)
  loop: boolean;          // 是否循环
  pingpong?: boolean;     // 是否来回播放（ping-pong），默认 false
  startFrame: number;     // 起始帧索引
  imagePath: string;      // 精灵图路径
  nextState?: AnimState;  // 非循环动画播放完成后的目标状态
}

export type WindowBehaviorAnimationKey =
  | 'dragging'
  | 'edgeHiddenLeft'
  | 'edgeHiddenRight'
  | 'edgePeekLeft'
  | 'edgePeekRight';

export type AnimState =
  | 'idle'
  | 'EnterInput'
  | 'shock'
  | 'startworking'
  | 'working'
  | 'workingPreview'
  | 'EnterReceiving'
  | 'Receiving'
  | 'received'
  | 'Response';

// 预设动画配置
export const ANIMATIONS: Record<string, AnimationConfig> = {
  idle: {
    name: 'idle',
    cols: 4,
    rows: 4,
    totalFrames: 16, // 4×4=16帧
    interval: 100,
    loop: true,
    startFrame: 0,
    imagePath: '/anim/ildesanimation.png',
  },
  shock: {
    name: 'shock',
    cols: 3,
    rows: 4,
    totalFrames: 10, // 10帧，1.5秒
    interval: 150,
    loop: false, // 单次播放
    startFrame: 0,
    imagePath: '/anim/shock.png',
  },
  EnterInput: {
    name: 'EnterInput',
    cols: 3,
    rows: 1,
    totalFrames: 3,
    interval: 667,
    loop: true,
    startFrame: 0,
    imagePath: '/anim/talkmode.png',
  },
  startworking: {
    name: 'startworking',
    cols: 7,
    rows: 7,
    totalFrames: 14, // 1~14帧（索引0~13）
    interval: 100,
    loop: false, // 一次播放
    startFrame: 0,
    imagePath: '/anim/startworking.png',
    nextState: 'working',
  },
  working: {
    name: 'working',
    cols: 7,
    rows: 7,
    totalFrames: 35, // 14~48帧（索引13~47），正向循环播放
    interval: 100,
    loop: true,
    pingpong: false, // 正向循环
    startFrame: 13,
    imagePath: '/anim/startworking.png',
  },
  workingPreview: {
    name: 'workingPreview',
    cols: 7,
    rows: 7,
    totalFrames: 35,
    interval: 100,
    loop: true,
    pingpong: false,
    startFrame: 13,
    imagePath: '/anim/startworking.png',
  },
  EnterReceiving: {
    name: 'EnterReceiving',
    cols: 4,
    rows: 3,
    totalFrames: 6, // 帧1-6（索引0-5）
    interval: 100,
    loop: false, // 单次播放
    startFrame: 0,
    imagePath: '/anim/receiving.png',
    nextState: 'Receiving', // 播完后进入 Receiving 循环
  },
  Receiving: {
    name: 'Receiving',
    cols: 4,
    rows: 3,
    totalFrames: 6, // 帧6-12（索引6-11）
    interval: 100,
    loop: true, // 循环播放
    startFrame: 6,
    imagePath: '/anim/receiving.png',
  },
  received: {
    name: 'received',
    cols: 1,
    rows: 1,
    totalFrames: 1, // 单帧静态
    interval: 100,
    loop: false,
    startFrame: 0,
    imagePath: '/anim/received.png',
  },
  Response: {
    name: 'Response',
    cols: 4,
    rows: 4,
    totalFrames: 16, // 与 idle 相同
    interval: 100,
    loop: true,
    startFrame: 0,
    imagePath: '/anim/ildesanimation.png',
  },
};

export const WINDOW_BEHAVIOR_ANIMATIONS: Record<WindowBehaviorAnimationKey, AnimationConfig> = {
  dragging: {
    name: 'dragging',
    cols: 2,
    rows: 1,
    totalFrames: 2,
    interval: 1000,
    loop: true,
    startFrame: 0,
    imagePath: '/anim/edge-placeholders/dragging.png',
  },
  edgeHiddenLeft: {
    name: 'edgeHiddenLeft',
    cols: 1,
    rows: 1,
    totalFrames: 1,
    interval: 100,
    loop: true,
    startFrame: 0,
    imagePath: '/anim/edge-placeholders/hide_right.png',
  },
  edgeHiddenRight: {
    name: 'edgeHiddenRight',
    cols: 1,
    rows: 1,
    totalFrames: 1,
    interval: 100,
    loop: true,
    startFrame: 0,
    imagePath: '/anim/edge-placeholders/hide_right.png',
  },
  edgePeekLeft: {
    name: 'edgePeekLeft',
    cols: 1,
    rows: 1,
    totalFrames: 1,
    interval: 100,
    loop: true,
    startFrame: 0,
    imagePath: '/anim/edge-placeholders/hide_right.png',
  },
  edgePeekRight: {
    name: 'edgePeekRight',
    cols: 1,
    rows: 1,
    totalFrames: 1,
    interval: 100,
    loop: true,
    startFrame: 0,
    imagePath: '/anim/edge-placeholders/hide_right.png',
  },
};

// 窗口尺寸
export const WINDOW_SIZE = {
  width: 180,
  height: 180,
};
