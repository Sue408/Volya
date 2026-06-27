import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

/** 作品状态 */
export type WorkStatus = 'Draft' | 'InProgress' | 'Completed'

/** 作品摘要 */
export interface WorkSummary {
  id: string
  title: string
  status: WorkStatus
  completed_words: number
  updated_at: string
}

/** 作品状态标签 */
export const workStatusLabels: Record<WorkStatus, { label: string; icon: string }> = {
  Draft: { label: '草稿', icon: '📝' },
  InProgress: { label: '进行中', icon: '🔥' },
  Completed: { label: '已完成', icon: '✅' },
}

/**
 * useWorks — 作品列表管理
 */
export function useWorks() {
  const works = ref<WorkSummary[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  /**
   * 加载所有作品列表
   */
  async function listWorks() {
    loading.value = true
    error.value = null
    try {
      works.value = await invoke<WorkSummary[]>('list_works')
    } catch (e) {
      error.value = `加载作品列表失败: ${e}`
      console.error(error.value)
    } finally {
      loading.value = false
    }
  }

  /**
   * 创建新作品
   */
  async function createWork(title: string): Promise<string | null> {
    try {
      const result = await invoke<string>('create_work', { title })
      const parsed = JSON.parse(result)
      // 刷新列表
      await listWorks()
      return parsed.id
    } catch (e) {
      error.value = `创建作品失败: ${e}`
      console.error(error.value)
      return null
    }
  }

  /**
   * 删除作品
   */
  async function deleteWork(id: string): Promise<boolean> {
    try {
      await invoke('delete_work', { id })
      // 从列表中移除
      works.value = works.value.filter(w => w.id !== id)
      return true
    } catch (e) {
      console.error('删除作品失败:', e)
      return false
    }
  }

  /**
   * 格式化相对时间
   */
  function timeAgo(isoStr: string): string {
    const date = new Date(isoStr)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMin = Math.floor(diffMs / 60000)
    const diffHour = Math.floor(diffMs / 3600000)
    const diffDay = Math.floor(diffMs / 86400000)

    if (diffMin < 1) return '刚刚'
    if (diffMin < 60) return `${diffMin} 分钟前`
    if (diffHour < 24) return `${diffHour} 小时前`
    if (diffDay < 7) return `${diffDay} 天前`
    return date.toLocaleDateString('zh-CN')
  }

  return {
    works,
    loading,
    error,
    listWorks,
    createWork,
    deleteWork,
    timeAgo,
  }
}
