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
            file_patterns: vec!["*.tmp".to_string(), "*.temp".to_string()],
            min_age_days: 0,
        });
        
        // Кеш браузеров
        categories.insert("browser_cache".to_string(), CleanupCategory {
            enabled: true,
            paths: get_browser_cache_paths(),
            file_patterns: vec!["*".to_string()],
            min_age_days: 7,
        });
        
        // Логи
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
