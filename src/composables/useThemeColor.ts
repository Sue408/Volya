/**
 * useThemeColor — 主题色系生成器
 *
 * 从单一主色出发，自动衍生 50~900 十阶色板，
 * 并映射为完整的语义 Token（背景/文字/边框/强调色）。
 *
 * 用法：
 *   const { setPrimaryColor } = useThemeColor()
 *   setPrimaryColor('#C15F3C')  // 珊瑚橙（Claude 风格）
 *
 * 原理：
 *   1. 主色 → HSL
 *   2. 固定色相，调整明度/饱和度 → 生成 50~900 色阶
 *   3. 色阶 → 语义 Token → 写入 document.documentElement.style
 */

// ─── 颜色工具函数 ───

function hexToHSL(hex: string): { h: number; s: number; l: number } {
  let r = 0, g = 0, b = 0
  const h = hex.replace('#', '')
  if (h.length === 6) {
    r = parseInt(h.substring(0, 2), 16) / 255
    g = parseInt(h.substring(2, 4), 16) / 255
    b = parseInt(h.substring(4, 6), 16) / 255
  }
  const max = Math.max(r, g, b), min = Math.min(r, g, b)
  let hue = 0, sat = 0
  const lit = (max + min) / 2

  if (max !== min) {
    const d = max - min
    sat = lit > 0.5 ? d / (2 - max - min) : d / (max + min)
    switch (max) {
      case r: hue = ((g - b) / d + (g < b ? 6 : 0)) / 6; break
      case g: hue = ((b - r) / d + 2) / 6; break
      case b: hue = ((r - g) / d + 4) / 6; break
    }
  }
  return { h: hue * 360, s: sat * 100, l: lit * 100 }
}

function hslToHex(h: number, s: number, l: number): string {
  s /= 100; l /= 100
  const a = s * Math.min(l, 1 - l)
  const f = (n: number) => {
    const k = (n + h / 30) % 12
    const color = l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1)
    return Math.round(255 * color).toString(16).padStart(2, '0')
  }
  return `#${f(0)}${f(8)}${f(4)}`
}

// ─── 色阶生成 ───

interface ColorScale {
  '50': string
  '100': string
  '200': string
  '300': string
  '400': string
  '500': string
  '600': string
  '700': string
  '800': string
  '900': string
}

/**
 * 从主色生成 50~900 色阶
 * 固定色相，50~900 明度递减，两端饱和度略降
 */
function generateScale(hex: string): ColorScale {
  const { h, s } = hexToHSL(hex)
  // 明度配置：从 95 → 14，步进约 9
  const lightnessSteps = [95, 86, 77, 68, 59, 50, 41, 32, 23, 14]
  // 饱和度微调：中间饱和，两端略降
  const satFactor = [0.5, 0.7, 0.85, 0.95, 1.0, 1.0, 0.95, 0.85, 0.75, 0.6]

  const keys: (keyof ColorScale)[] = ['50', '100', '200', '300', '400', '500', '600', '700', '800', '900']
  const scale = {} as ColorScale

  keys.forEach((key, i) => {
    scale[key] = hslToHex(h, s * satFactor[i], lightnessSteps[i])
  })

  return scale
}

// ─── 预设主题 ───

export const THEMES: Record<string, { primary: string; name: string }> = {
  coral:   { primary: '#C9603A', name: '珊瑚橙' },
  sage:    { primary: '#6C8360', name: '鼠尾绿' },
  apricot: { primary: '#E58F53', name: '暖杏' },
  plum:    { primary: '#8B5E7C', name: '紫罗兰' },
  ocean:   { primary: '#5B8DB8', name: '海洋蓝' },
}

// ─── 主题色管理 ───

import { ref } from 'vue'

const currentPrimary = ref('#C9603A')

export function useThemeColor() {
  /** 从色阶取色 */
  function shade(scale: ColorScale, key: keyof ColorScale): string {
    return scale[key]
  }

  /** 计算 hex 的相对亮度 (0~1)，用于判断深色底还是浅色底 */
  function luminance(hex: string): number {
    const h = hex.replace('#', '')
    const r = parseInt(h.substring(0, 2), 16) / 255
    const g = parseInt(h.substring(2, 4), 16) / 255
    const b = parseInt(h.substring(4, 6), 16) / 255
    const toLinear = (c: number) => c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4)
    return 0.2126 * toLinear(r) + 0.7152 * toLinear(g) + 0.0722 * toLinear(b)
  }

  /** 根据背景亮度返回浅色或深色文字 */
  function textOnBg(bgHex: string): string {
    return luminance(bgHex) > 0.5 ? '#1C1B1A' : '#FFFFFF'
  }

  /** 解析主色并写入 CSS 变量（仅强调色，不影响背景/边框） */
  function applyScale(primary: string) {
    const scale = generateScale(primary)
    const root = document.documentElement

    // 写入色阶变量
    const keys = Object.keys(scale) as (keyof ColorScale)[]
    keys.forEach(k => {
      root.style.setProperty(`--coral-${k}`, scale[k])
    })

    const primary400 = shade(scale, '400')
    const primary500 = shade(scale, '500')

    // 覆盖强调色，并自动计算文字颜色
    root.style.setProperty('--accent-primary', primary400)
    root.style.setProperty('--accent-secondary', shade(scale, '300'))
    root.style.setProperty('--accent-hover', primary500)
    root.style.setProperty('--text-link', primary400)
    root.style.setProperty('--text-on-color', textOnBg(primary400))
    root.style.setProperty('--text-on-accent', textOnBg(primary400))
    root.style.setProperty('--shadow-focus', `0 0 0 3px ${primary400}33`)

    currentPrimary.value = primary
  }

  /**
   * 设置主色
   */
  function setPrimaryColor(hex: string) {
    applyScale(hex)
    localStorage.setItem('volya-primary-color', hex)
  }

  /** 从 localStorage 恢复 */
  function loadPrimaryColor() {
    const saved = localStorage.getItem('volya-primary-color')
    if (saved && /^#[0-9a-f]{6}$/i.test(saved)) {
      applyScale(saved)
    } else {
      applyScale('#C9603A') // 默认珊瑚
    }
  }

  return {
    currentPrimary,
    setPrimaryColor,
    loadPrimaryColor,
    generateScale,   // 导出供调试/预览用
    THEMES,
  }
}
