import { describe, it, expect } from 'vitest'
import { ANIMATIONS, WINDOW_BEHAVIOR_ANIMATIONS } from '../animations'

describe('动画配置测试', () => {
  // EnterReceiving 配置验证
  describe('EnterReceiving', () => {
    const anim = ANIMATIONS.EnterReceiving

    it('列数为4', () => {
      expect(anim.cols).toBe(4)
    })

    it('行数为3', () => {
      expect(anim.rows).toBe(3)
    })

    it('总帧数为6', () => {
      expect(anim.totalFrames).toBe(6)
    })

    it('起始帧为0', () => {
      expect(anim.startFrame).toBe(0)
    })

    it('单次播放', () => {
      expect(anim.loop).toBe(false)
    })

    it('播放完切换到 Receiving', () => {
      expect(anim.nextState).toBe('Receiving')
    })

    it('精灵图路径正确', () => {
      expect(anim.imagePath).toBe('/anim/receiving.png')
    })
  })

  // Receiving 配置验证
  describe('Receiving', () => {
    const anim = ANIMATIONS.Receiving

    it('列数为4', () => {
      expect(anim.cols).toBe(4)
    })

    it('行数为3', () => {
      expect(anim.rows).toBe(3)
    })

    it('总帧数为6', () => {
      expect(anim.totalFrames).toBe(6)
    })

    it('起始帧为6', () => {
      expect(anim.startFrame).toBe(6)
    })

    it('循环播放', () => {
      expect(anim.loop).toBe(true)
    })

    it('精灵图路径正确', () => {
      expect(anim.imagePath).toBe('/anim/receiving.png')
    })
  })

  // startworking 配置验证
  describe('startworking', () => {
    const anim = ANIMATIONS.startworking

    it('单次播放', () => {
      expect(anim.loop).toBe(false)
    })

    it('播放完切换到 working', () => {
      expect(anim.nextState).toBe('working')
    })
  })

  // working 配置验证
  describe('working', () => {
    const anim = ANIMATIONS.working

    it('循环播放', () => {
      expect(anim.loop).toBe(true)
    })

    it('无 nextState', () => {
      expect(anim.nextState).toBeUndefined()
    })
  })

  describe('window behavior dragging', () => {
    const anim = WINDOW_BEHAVIOR_ANIMATIONS.dragging

    it('列数为5', () => {
      expect(anim.cols).toBe(5)
    })

    it('行数为1', () => {
      expect(anim.rows).toBe(1)
    })

    it('总帧数为5', () => {
      expect(anim.totalFrames).toBe(5)
    })

    it('循环播放', () => {
      expect(anim.loop).toBe(true)
    })

    it('精灵图路径正确', () => {
      expect(anim.imagePath).toBe('/anim/edge-placeholders/dragging.png')
    })
  })

  describe('window behavior edgeHiddenLeft', () => {
    const anim = WINDOW_BEHAVIOR_ANIMATIONS.edgeHiddenLeft

    it('使用单帧贴边动画（左侧通过 CSS scaleX(-1) 镜像 hide_right）', () => {
      expect(anim.cols).toBe(1)
      expect(anim.rows).toBe(1)
      expect(anim.totalFrames).toBe(1)
      expect(anim.loop).toBe(true)
    })

    it('精灵图路径正确（使用 hide_right 通过 CSS 镜像）', () => {
      expect(anim.imagePath).toBe('/anim/edge-placeholders/hide_right.png')
    })
  })

  describe('window behavior edgeHiddenRight', () => {
    const anim = WINDOW_BEHAVIOR_ANIMATIONS.edgeHiddenRight

    it('使用单帧贴边动画', () => {
      expect(anim.cols).toBe(1)
      expect(anim.rows).toBe(1)
      expect(anim.totalFrames).toBe(1)
      expect(anim.loop).toBe(true)
    })

    it('精灵图路径正确', () => {
      expect(anim.imagePath).toBe('/anim/edge-placeholders/hide_right.png')
    })
  })

  describe('window behavior edgePeekLeft', () => {
    const anim = WINDOW_BEHAVIOR_ANIMATIONS.edgePeekLeft

    it('使用单帧占位图（peek 由 Canvas 渲染）', () => {
      expect(anim.cols).toBe(1)
      expect(anim.rows).toBe(1)
      expect(anim.totalFrames).toBe(1)
      expect(anim.loop).toBe(true)
    })

    it('精灵图路径正确', () => {
      expect(anim.imagePath).toBe('/anim/edge-placeholders/hide_right.png')
    })
  })

  describe('window behavior edgePeekRight', () => {
    const anim = WINDOW_BEHAVIOR_ANIMATIONS.edgePeekRight

    it('使用单帧占位图（peek 由 Canvas 渲染）', () => {
      expect(anim.cols).toBe(1)
      expect(anim.rows).toBe(1)
      expect(anim.totalFrames).toBe(1)
      expect(anim.loop).toBe(true)
    })

    it('精灵图路径正确', () => {
      expect(anim.imagePath).toBe('/anim/edge-placeholders/hide_right.png')
    })
  })
})
