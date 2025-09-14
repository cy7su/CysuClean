import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { getCurrent } from '@tauri-apps/api/window'
import { 
  Trash2, 
  HardDrive, 
  Clock, 
  Settings, 
  Play, 
  Pause, 
  CheckCircle,
  XCircle,
  Minus,
  X
} from 'lucide-react'

interface ScanResult {
  category: string
  total_files: number
  total_size: number
  files: FileInfo[]
}

interface FileInfo {
  path: string
  size: number
  modified: number
  is_directory: boolean
}

interface CleanResult {
  category: string
  files_removed: number
  space_freed: number
  errors: string[]
}

const CATEGORY_NAMES: Record<string, string> = {
  temp_files: 'Временные файлы',
  browser_cache: 'Кеш браузеров',
  logs: 'Логи системы',
  recycle_bin: 'Корзина',
  windows_cache: 'Кеш Windows',
  thumbnails: 'Миниатюры',
  old_downloads: 'Старые загрузки',
  app_cache: 'Кеш приложений',
  installers: 'Установщики',
  windows_update: 'Windows Update',
  microsoft_store: 'Microsoft Store',
  office_cache: 'Кеш Office',
  visual_studio: 'Visual Studio',
  dotnet_cache: 'Кеш .NET',
  nodejs_cache: 'Node.js кеш',
  python_cache: 'Python кеш',
  java_cache: 'Java кеш',
  adobe_cache: 'Adobe кеш',
  nvidia_cache: 'NVIDIA кеш',
  amd_cache: 'AMD кеш',
  intel_cache: 'Intel кеш',
  antivirus_cache: 'Антивирус кеш',
  vpn_cache: 'VPN кеш',
  torrent_cache: 'Торрент кеш',
  media_cache: 'Медиаплееры',
  games_cache: 'Игровой кеш',
  system_utils: 'Системные утилиты',
  archivers_cache: 'Архиваторы',
  cloud_cache: 'Облачные хранилища'
}

const CATEGORY_ICONS: Record<string, React.ReactNode> = {
  temp_files: <Clock className="w-4 h-4" />,
  browser_cache: <HardDrive className="w-4 h-4" />,
  logs: <Settings className="w-4 h-4" />,
  recycle_bin: <Trash2 className="w-4 h-4" />,
  windows_cache: <Settings className="w-4 h-4" />,
  thumbnails: <HardDrive className="w-4 h-4" />,
  old_downloads: <HardDrive className="w-4 h-4" />,
  app_cache: <Settings className="w-4 h-4" />,
  installers: <Settings className="w-4 h-4" />,
  windows_update: <Settings className="w-4 h-4" />,
  microsoft_store: <Settings className="w-4 h-4" />,
  office_cache: <Settings className="w-4 h-4" />,
  visual_studio: <Settings className="w-4 h-4" />,
  dotnet_cache: <Settings className="w-4 h-4" />,
  nodejs_cache: <Settings className="w-4 h-4" />,
  python_cache: <Settings className="w-4 h-4" />,
  java_cache: <Settings className="w-4 h-4" />,
  adobe_cache: <Settings className="w-4 h-4" />,
  nvidia_cache: <Settings className="w-4 h-4" />,
  amd_cache: <Settings className="w-4 h-4" />,
  intel_cache: <Settings className="w-4 h-4" />,
  antivirus_cache: <Settings className="w-4 h-4" />,
  vpn_cache: <Settings className="w-4 h-4" />,
  torrent_cache: <Settings className="w-4 h-4" />,
  media_cache: <Settings className="w-4 h-4" />,
  games_cache: <Settings className="w-4 h-4" />,
  system_utils: <Settings className="w-4 h-4" />,
  archivers_cache: <Settings className="w-4 h-4" />,
  cloud_cache: <Settings className="w-4 h-4" />
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function App() {
  const [scanResults, setScanResults] = useState<Record<string, ScanResult>>({})
  const [cleanResults, setCleanResults] = useState<CleanResult[]>([])
  const [isScanning, setIsScanning] = useState(false)
  const [isCleaning, setIsCleaning] = useState(false)
  const [scanProgress, setScanProgress] = useState(0)
  const [cleanProgress, setCleanProgress] = useState(0)
  const [selectedCategories, setSelectedCategories] = useState<string[]>([])
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState<string | null>(null)

  const totalSize = Object.values(scanResults).reduce((sum, result) => sum + result.total_size, 0)
  const totalFiles = Object.values(scanResults).reduce((sum, result) => sum + result.total_files, 0)

  useEffect(() => {
    const interval = setInterval(async () => {
      if (isScanning) {
        try {
          const progress = await invoke<number>('get_scan_progress')
          setScanProgress(progress * 100)
        } catch (e) {
          console.error('Failed to get scan progress:', e)
        }
      }
      if (isCleaning) {
        try {
          const progress = await invoke<number>('get_clean_progress')
          setCleanProgress(progress * 100)
        } catch (e) {
          console.error('Failed to get clean progress:', e)
        }
      }
    }, 100)

    return () => clearInterval(interval)
  }, [isScanning, isCleaning])

  const handleScan = async () => {
    setIsScanning(true)
    setError(null)
    setSuccess(null)
    setScanProgress(0)
    
    // Расширяем окно при сканировании
    await expandWindow()
    
    try {
      const results = await invoke<Record<string, ScanResult>>('scan_system')
      setScanResults(results)
      setSelectedCategories(Object.keys(results))
      setSuccess('Сканирование завершено успешно!')
    } catch (e) {
      setError(`Ошибка сканирования: ${e}`)
    } finally {
      setIsScanning(false)
    }
  }

  const handleClean = async () => {
    if (selectedCategories.length === 0) {
      setError('Выберите категории для очистки')
      return
    }

    setIsCleaning(true)
    setError(null)
    setSuccess(null)
    setCleanProgress(0)
    
    try {
      const results = await invoke<CleanResult[]>('clean_system', { 
        categories: selectedCategories 
      })
      setCleanResults(results)
      
      const totalFilesRemoved = results.reduce((sum, r) => sum + r.files_removed, 0)
      const totalSpaceFreed = results.reduce((sum, r) => sum + r.space_freed, 0)
      
      setSuccess(`Очистка завершена! Удалено файлов: ${totalFilesRemoved}, освобождено места: ${formatBytes(totalSpaceFreed)}`)
      
      // Обновляем результаты сканирования
      await handleScan()
    } catch (e) {
      setError(`Ошибка очистки: ${e}`)
    } finally {
      setIsCleaning(false)
    }
  }

  const toggleCategory = (category: string) => {
    setSelectedCategories(prev => 
      prev.includes(category)
        ? prev.filter(c => c !== category)
        : [...prev, category]
    )
  }

  const minimizeWindow = async () => {
    try {
      await invoke('minimize_window')
    } catch (error) {
      console.error('Failed to minimize window:', error)
    }
  }


  const closeWindow = async () => {
    try {
      await invoke('close_window')
    } catch (error) {
      console.error('Failed to close window:', error)
    }
  }

  const expandWindow = async () => {
    try {
      const window = getCurrent()
      await window.setSize({ width: 800, height: 620, type: 'Logical' })
    } catch (error) {
      console.error('Failed to expand window:', error)
    }
  }


  return (
    <div 
      className="min-h-screen bg-dark-950 overflow-hidden"
      onContextMenu={(e) => e.preventDefault()}
    >
      {/* Custom Title Bar - Fixed */}
      <div 
        className="fixed top-0 left-0 right-0 z-50 flex items-center justify-between border-b border-dark-700 px-4 py-2 cursor-move"
        style={{ backgroundColor: '#0b0806' }}
        data-tauri-drag-region
      >
        <div className="flex items-center">
          <div className="w-4 h-4 mr-3 flex items-center justify-center">
            <Settings className="w-3 h-3 text-accent-950" />
          </div>
          <h1 className="text-sm font-semibold text-gray-100">
            CysuClean
          </h1>
        </div>
        
        <div className="flex items-center space-x-1">
          <button
            onClick={minimizeWindow}
            className="w-8 h-6 flex items-center justify-center hover:bg-dark-700 rounded text-gray-400 hover:text-gray-200 transition-colors"
          >
            <Minus className="w-3 h-3" />
          </button>
          <button
            onClick={closeWindow}
            className="w-8 h-6 flex items-center justify-center hover:bg-danger-600 rounded text-gray-400 hover:text-white transition-colors"
          >
            <X className="w-3 h-3" />
          </button>
        </div>
      </div>

      <div className="max-w-7xl mx-auto p-4 pt-16 overflow-hidden">

        {/* Status Messages */}
        {error && (
          <div className="mb-4 p-3 bg-danger-900/20 border border-danger-700 rounded-lg flex items-center">
            <XCircle className="w-4 h-4 text-danger-400 mr-2" />
            <span className="text-danger-300 text-sm">{error}</span>
          </div>
        )}

        {success && (
          <div className="mb-4 p-3 bg-success-900/20 border border-success-700 rounded-lg flex items-center">
            <CheckCircle className="w-4 h-4 text-success-400 mr-2" />
            <span className="text-success-300 text-sm">{success}</span>
          </div>
        )}

        {/* Controls - Fixed Panel */}
        <div className="sticky top-0 z-10 card p-4 mb-4">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-semibold text-gray-100">Управление</h2>
            
            <div className="flex gap-3">
              <button
                onClick={handleScan}
                disabled={isScanning || isCleaning}
                className="btn btn-primary btn-md flex items-center"
              >
                {isScanning ? <Pause className="w-4 h-4 mr-2" /> : <Play className="w-4 h-4 mr-2" />}
                {isScanning ? 'Сканирование...' : 'Сканировать'}
              </button>

              <button
                onClick={handleClean}
                disabled={isCleaning || selectedCategories.length === 0}
                className="btn btn-danger btn-md flex items-center"
              >
                <Trash2 className="w-4 h-4 mr-2" />
                {isCleaning ? 'Очистка...' : 'Очистить'}
              </button>
            </div>
          </div>

          {/* Progress Bars */}
          {isScanning && (
            <div className="mt-3">
              <div className="flex justify-between text-xs text-gray-400 mb-1">
                <span>Сканирование системы</span>
                <span>{Math.round(scanProgress)}%</span>
              </div>
              <div className="progress">
                <div 
                  className="progress-bar" 
                  style={{ width: `${scanProgress}%` }}
                />
              </div>
            </div>
          )}

          {isCleaning && (
            <div className="mt-3">
              <div className="flex justify-between text-xs text-gray-400 mb-1">
                <span>Очистка системы</span>
                <span>{Math.round(cleanProgress)}%</span>
              </div>
              <div className="progress">
                <div 
                  className="progress-bar bg-danger-600" 
                  style={{ width: `${cleanProgress}%` }}
                />
              </div>
            </div>
          )}
        </div>

        {/* Summary */}
        {Object.keys(scanResults).length > 0 && (
          <div className="card p-4 mb-4">
            <h2 className="text-lg font-semibold mb-3 text-gray-100">Статистика</h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              <div className="text-center p-3 bg-accent-950/20 rounded-lg border border-accent-950">
                <div className="text-xl font-bold text-accent-400">{totalFiles}</div>
                <div className="text-xs text-gray-400">Файлов найдено</div>
              </div>
              <div className="text-center p-3 bg-success-900/20 rounded-lg border border-success-700">
                <div className="text-xl font-bold text-success-400">{formatBytes(totalSize)}</div>
                <div className="text-xs text-gray-400">Общий размер</div>
              </div>
              <div className="text-center p-3 bg-primary-900/20 rounded-lg border border-primary-700">
                <div className="text-xl font-bold text-primary-400">{selectedCategories.length}</div>
                <div className="text-xs text-gray-400">Выбрано</div>
              </div>
            </div>
          </div>
        )}

        {/* Categories */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {Object.entries(scanResults).map(([category, result]) => (
            <div key={category} className="card p-4">
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="text-accent-400">
                    {CATEGORY_ICONS[category]}
                  </div>
                  <h3 className="text-sm font-semibold ml-2 text-gray-100">
                    {CATEGORY_NAMES[category] || category}
                  </h3>
                </div>
                <input
                  type="checkbox"
                  checked={selectedCategories.includes(category)}
                  onChange={() => toggleCategory(category)}
                  className="w-4 h-4 text-accent-950 rounded focus:ring-accent-950 bg-dark-700 border-dark-600"
                  disabled={isScanning || isCleaning}
                />
              </div>

              <div className="space-y-1">
                <div className="flex justify-between text-xs">
                  <span className="text-gray-400">Файлов:</span>
                  <span className="font-medium text-gray-200">{result.total_files}</span>
                </div>
                <div className="flex justify-between text-xs">
                  <span className="text-gray-400">Размер:</span>
                  <span className="font-medium text-gray-200">{formatBytes(result.total_size)}</span>
                </div>
              </div>

            </div>
          ))}
        </div>

        {/* Clean Results */}
        {cleanResults.length > 0 && (
          <div className="card p-4 mt-4">
            <h2 className="text-lg font-semibold mb-3 text-gray-100">Результаты очистки</h2>
            <div className="space-y-3">
              {cleanResults.map((result, index) => (
                <div key={index} className="p-3 rounded-lg" style={{backgroundColor: '#2a2520', border: '1px solid #3d3630'}}>
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="font-medium text-gray-200 text-sm">
                      {CATEGORY_NAMES[result.category] || result.category}
                    </h3>
                    <div className="flex items-center text-xs text-success-400">
                      <CheckCircle className="w-3 h-3 mr-1" />
                      Готово
                    </div>
                  </div>
                  <div className="grid grid-cols-2 gap-3 text-xs">
                    <div>
                      <span className="text-gray-400">Удалено:</span>
                      <span className="ml-1 font-medium text-gray-200">{result.files_removed}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Освобождено:</span>
                      <span className="ml-1 font-medium text-gray-200">{formatBytes(result.space_freed)}</span>
                    </div>
                  </div>
                  {result.errors.length > 0 && (
                    <div className="mt-2">
                      <div className="text-xs text-danger-400 font-medium">Ошибки:</div>
                      <ul className="text-xs text-danger-300 mt-1">
                        {result.errors.map((error, i) => (
                          <li key={i}>• {error}</li>
                        ))}
                      </ul>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default App
