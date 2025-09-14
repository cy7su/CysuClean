use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub safe_mode: bool,
    pub backup_enabled: bool,
    pub max_file_size_mb: u64,
    pub excluded_paths: Vec<PathBuf>,
    pub cleanup_categories: HashMap<String, CleanupCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupCategory {
    pub enabled: bool,
    pub paths: Vec<PathBuf>,
    pub file_patterns: Vec<String>,
    pub min_age_days: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut categories = HashMap::new();
        
        // Временные файлы
        categories.insert("temp_files".to_string(), CleanupCategory {
            enabled: true,
            paths: get_temp_paths(),
            file_patterns: vec!["*.tmp".to_string(), "*.temp".to_string(), "*.bak".to_string()],
            min_age_days: 0,
        });
        
        // Кеш браузеров
        categories.insert("browser_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_browser_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 7,
        });
        
        // Логи системы
        categories.insert("logs".to_string(), CleanupCategory {
            enabled: true,
            paths: get_log_paths(),
            file_patterns: vec!["*.log".to_string(), "*.log.*".to_string()],
            min_age_days: 30,
        });
        
        // Корзина
        categories.insert("recycle_bin".to_string(), CleanupCategory {
            enabled: true,
            paths: get_recycle_bin_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 0,
        });

        // Кеш Windows
        categories.insert("windows_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_windows_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 7,
        });

        // Thumbnails
        categories.insert("thumbnails".to_string(), CleanupCategory {
            enabled: true,
            paths: get_thumbnails_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Downloads (старые файлы)
        categories.insert("old_downloads".to_string(), CleanupCategory {
            enabled: true,
            paths: get_downloads_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 90,
        });

        // Кеш приложений
        categories.insert("app_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_app_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 14,
        });

        // Временные установщики
        categories.insert("installers".to_string(), CleanupCategory {
            enabled: true,
            paths: get_installer_paths(),
            file_patterns: vec!["*.msi".to_string(), "*.exe".to_string(), "*.zip".to_string()],
            min_age_days: 7,
        });

        // Кеш Windows Update
        categories.insert("windows_update".to_string(), CleanupCategory {
            enabled: true,
            paths: get_windows_update_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш Microsoft Store
        categories.insert("microsoft_store".to_string(), CleanupCategory {
            enabled: true,
            paths: get_microsoft_store_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 14,
        });

        // Кеш Office
        categories.insert("office_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_office_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш Visual Studio
        categories.insert("visual_studio".to_string(), CleanupCategory {
            enabled: true,
            paths: get_visual_studio_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 60,
        });

        // Кеш .NET
        categories.insert("dotnet_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_dotnet_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш Node.js
        categories.insert("nodejs_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_nodejs_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 14,
        });

        // Кеш Python
        categories.insert("python_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_python_cache_paths(),
            file_patterns: vec!["*".to_string(), "__pycache__".to_string()],
            min_age_days: 14,
        });

        // Кеш Java
        categories.insert("java_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_java_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш Adobe
        categories.insert("adobe_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_adobe_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш NVIDIA
        categories.insert("nvidia_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_nvidia_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш AMD
        categories.insert("amd_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_amd_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш Intel
        categories.insert("intel_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_intel_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш антивирусов
        categories.insert("antivirus_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_antivirus_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 7,
        });

        // Кеш VPN
        categories.insert("vpn_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_vpn_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 14,
        });

        // Кеш торрентов
        categories.insert("torrent_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_torrent_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 7,
        });

        // Кеш медиаплееров
        categories.insert("media_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_media_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш игр
        categories.insert("games_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_games_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 60,
        });

        // Кеш системных утилит
        categories.insert("system_utils".to_string(), CleanupCategory {
            enabled: true,
            paths: get_system_utils_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 14,
        });

        // Кеш архиваторов
        categories.insert("archivers_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_archivers_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 30,
        });

        // Кеш облачных хранилищ
        categories.insert("cloud_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_cloud_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 14,
        });

        Self {
            safe_mode: true,
            backup_enabled: false,
            max_file_size_mb: 100,
            excluded_paths: vec![],
            cleanup_categories: categories,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = get_config_path();
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: AppConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = AppConfig::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
}

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("cleaner")
        .join("config.json")
}

fn get_temp_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    // Windows temp paths
    if let Some(temp) = std::env::var_os("TEMP") {
        paths.push(PathBuf::from(temp));
    }
    if let Some(temp) = std::env::var_os("TMP") {
        paths.push(PathBuf::from(temp));
    }
    
    // System temp
    if let Some(temp) = dirs::cache_dir() {
        paths.push(temp);
    }
    
    paths
}

fn get_browser_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(data_dir) = dirs::data_dir() {
        // Chrome
        paths.push(data_dir.join("Google").join("Chrome").join("User Data").join("Default").join("Cache"));
        // Firefox
        paths.push(data_dir.join("Mozilla").join("Firefox").join("Profiles"));
        // Edge
        paths.push(data_dir.join("Microsoft").join("Edge").join("User Data").join("Default").join("Cache"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        // Additional browser caches
        paths.push(cache_dir.join("Google").join("Chrome"));
        paths.push(cache_dir.join("Mozilla").join("Firefox"));
    }
    
    paths
}

fn get_log_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    // Windows Event Logs
    paths.push(PathBuf::from("C:\\Windows\\Logs"));
    paths.push(PathBuf::from("C:\\Windows\\System32\\winevt\\Logs"));
    
    // Application logs
    if let Some(data_dir) = dirs::data_dir() {
        paths.push(data_dir.join("logs"));
    }
    
    paths
}

fn get_recycle_bin_paths() -> Vec<PathBuf> {
    vec![
        PathBuf::from("C:\\$Recycle.Bin"),
    ]
}

fn get_windows_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    // Windows cache directories
    paths.push(PathBuf::from("C:\\Windows\\Temp"));
    paths.push(PathBuf::from("C:\\Windows\\Prefetch"));
    paths.push(PathBuf::from("C:\\Windows\\SoftwareDistribution\\Download"));
    
    // User cache
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir);
    }
    
    paths
}

fn get_thumbnails_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    // Windows thumbnails
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Microsoft").join("Windows").join("Explorer"));
    }
    
    // Thumbnails cache
    paths.push(PathBuf::from("C:\\Users\\Public\\Thumbnails"));
    
    paths
}

fn get_downloads_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(downloads) = dirs::download_dir() {
        paths.push(downloads);
    }
    
    paths
}

fn get_app_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        // Steam cache
        paths.push(app_data.join("Steam").join("htmlcache"));
        // Discord cache
        paths.push(app_data.join("discord").join("Cache"));
        // Spotify cache
        paths.push(app_data.join("Spotify").join("Storage"));
        // Adobe cache
        paths.push(app_data.join("Adobe").join("Common").join("Media Cache Files"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        // Various app caches
        paths.push(cache_dir.join("Microsoft").join("Windows").join("INetCache"));
        paths.push(cache_dir.join("Microsoft").join("Windows").join("WebCache"));
    }
    
    paths
}

fn get_installer_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    // Common installer locations
    if let Some(downloads) = dirs::download_dir() {
        paths.push(downloads);
    }
    
    if let Some(desktop) = dirs::desktop_dir() {
        paths.push(desktop);
    }
    
    // Windows installer cache
    paths.push(PathBuf::from("C:\\Windows\\Installer"));
    
    paths
}

fn get_windows_update_paths() -> Vec<PathBuf> {
    vec![
        PathBuf::from("C:\\Windows\\SoftwareDistribution\\Download"),
        PathBuf::from("C:\\Windows\\SoftwareDistribution\\DataStore"),
        PathBuf::from("C:\\Windows\\Logs\\WindowsUpdate"),
    ]
}

fn get_microsoft_store_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Microsoft").join("Windows").join("INetCache"));
        paths.push(app_data.join("Packages"));
    }
    
    paths
}

fn get_office_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Microsoft").join("Office").join("16.0").join("OfficeFileCache"));
        paths.push(app_data.join("Microsoft").join("Office").join("UnsavedFiles"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("Microsoft").join("Office"));
    }
    
    paths
}

fn get_visual_studio_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Microsoft").join("VisualStudio"));
        paths.push(app_data.join("Microsoft").join("VSCommon"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("Microsoft").join("VisualStudio"));
    }
    
    paths
}

fn get_dotnet_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Microsoft").join("dotnet"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("Microsoft").join("dotnet"));
    }
    
    paths
}

fn get_nodejs_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("npm"));
        paths.push(cache_dir.join("yarn"));
    }
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("npm-cache"));
    }
    
    paths
}

fn get_python_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("pip"));
    }
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Python"));
    }
    
    paths
}

fn get_java_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Oracle").join("Java"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("Oracle").join("Java"));
    }
    
    paths
}

fn get_adobe_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Adobe").join("Common").join("Media Cache Files"));
        paths.push(app_data.join("Adobe").join("Common").join("Media Cache"));
        paths.push(app_data.join("Adobe").join("Common").join("Cache"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("Adobe"));
    }
    
    paths
}

fn get_nvidia_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("NVIDIA Corporation"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("NVIDIA Corporation"));
    }
    
    paths
}

fn get_amd_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("AMD"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("AMD"));
    }
    
    paths
}

fn get_intel_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("Intel"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("Intel"));
    }
    
    paths
}

fn get_antivirus_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        // Kaspersky
        paths.push(app_data.join("Kaspersky Lab"));
        // Avast
        paths.push(app_data.join("AVAST Software"));
        // AVG
        paths.push(app_data.join("AVG"));
        // Norton
        paths.push(app_data.join("Norton"));
        // McAfee
        paths.push(app_data.join("McAfee"));
    }
    
    paths
}

fn get_vpn_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        // NordVPN
        paths.push(app_data.join("NordVPN"));
        // ExpressVPN
        paths.push(app_data.join("ExpressVPN"));
        // CyberGhost
        paths.push(app_data.join("CyberGhost"));
        // ProtonVPN
        paths.push(app_data.join("ProtonVPN"));
    }
    
    paths
}

fn get_torrent_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        // qBittorrent
        paths.push(app_data.join("qBittorrent"));
        // uTorrent
        paths.push(app_data.join("uTorrent"));
        // BitTorrent
        paths.push(app_data.join("BitTorrent"));
        // Deluge
        paths.push(app_data.join("Deluge"));
    }
    
    paths
}

fn get_media_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        // VLC
        paths.push(app_data.join("vlc"));
        // Media Player Classic
        paths.push(app_data.join("MPC-HC"));
        // PotPlayer
        paths.push(app_data.join("PotPlayer"));
        // KMPlayer
        paths.push(app_data.join("KMPlayer"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("vlc"));
    }
    
    paths
}

fn get_games_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        // Steam
        paths.push(app_data.join("Steam").join("htmlcache"));
        // Epic Games
        paths.push(app_data.join("Epic").join("EpicGamesLauncher").join("Saved"));
        // Origin
        paths.push(app_data.join("Origin"));
        // Uplay
        paths.push(app_data.join("Ubisoft Game Launcher"));
    }
    
    paths
}

fn get_system_utils_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        // CCleaner
        paths.push(app_data.join("Piriform").join("CCleaner"));
        // WinRAR
        paths.push(app_data.join("WinRAR"));
        // 7-Zip
        paths.push(app_data.join("7-Zip"));
        // Notepad++
        paths.push(app_data.join("Notepad++"));
    }
    
    paths
}

fn get_archivers_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        paths.push(app_data.join("WinRAR"));
        paths.push(app_data.join("7-Zip"));
        paths.push(app_data.join("Bandizip"));
        paths.push(app_data.join("PeaZip"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("WinRAR"));
        paths.push(cache_dir.join("7-Zip"));
    }
    
    paths
}

fn get_cloud_cache_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    if let Some(app_data) = dirs::data_dir() {
        // OneDrive
        paths.push(app_data.join("Microsoft").join("OneDrive"));
        // Google Drive
        paths.push(app_data.join("Google").join("Drive"));
        // Dropbox
        paths.push(app_data.join("Dropbox"));
        // iCloud
        paths.push(app_data.join("Apple Computer").join("iCloud"));
    }
    
    if let Some(cache_dir) = dirs::cache_dir() {
        paths.push(cache_dir.join("Microsoft").join("OneDrive"));
        paths.push(cache_dir.join("Google").join("Drive"));
    }
    
    paths
}
