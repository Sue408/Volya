import { ref, watch } from 'vue'

type Theme = 'light' | 'dark' | 'system'

const theme = ref<Theme>(loadTheme())

/** 从 localStorage 加载主题偏好 */
function loadTheme(): Theme {
  const stored = localStorage.getItem('volya-theme')
  if (stored === 'light' || stored === 'dark' || stored === 'system') {
    return stored
  }
  return 'system'
}

/** 应用主题到 document */
function applyTheme(t: Theme) {
  const isDark = t === 'dark' || (t === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
  document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light')
}

/** 监听系统主题变化 */
let systemThemeListener: (() => void) | null = null

function setupSystemListener() {
  cleanupSystemListener()
  const mq = window.matchMedia('(prefers-color-scheme: dark)')
  const handler = () => {
    if (theme.value === 'system') {
      applyTheme('system')
    }
  }
  mq.addEventListener('change', handler)
  systemThemeListener = () => mq.removeEventListener('change', handler)
}

function cleanupSystemListener() {
  if (systemThemeListener) {
    systemThemeListener()
    systemThemeListener = null
  }
}

/** 切换主题 */
function setTheme(t: Theme) {
  theme.value = t
  localStorage.setItem('volya-theme', t)
  applyTheme(t)
  if (t === 'system') {
    setupSystemListener()
  } else {
    cleanupSystemListener()
  }
}

/** 当前有效主题 (light/dark，不考虑 system) */
const effectiveTheme = ref<'light' | 'dark'>('light')

function updateEffective() {
  const isDark = theme.value === 'dark' ||
    (theme.value === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
  effectiveTheme.value = isDark ? 'dark' : 'light'
}

// 初始化
applyTheme(theme.value)
updateEffective()
if (theme.value === 'system') {
  setupSystemListener()
}

// 监听有效主题变化
watch(theme, () => updateEffective())

/**
 * useTheme — 主题切换
 * 支持 light / dark / system (跟随系统)
 */
export function useTheme() {
  function toggleTheme() {
    if (effectiveTheme.value === 'light') {
      setTheme('dark')
    } else {
      setTheme('light')
    }
  }

  return {
    theme,           // 当前偏好: 'light' | 'dark' | 'system'
    effectiveTheme,  // 实际生效: 'light' | 'dark'
    setTheme,
    toggleTheme,
  }
}
