use crate::config::{AppConfig, CleanupCategory};
use crate::error::CleanerError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};
use walkdir::WalkDir;
use std::io;

/// Переводит системные ошибки на человеческий язык
fn translate_error(error: &io::Error) -> String {
    match error.raw_os_error() {
        Some(32) => "Файл используется другой программой".to_string(),
        Some(5) => "Нет доступа к файлу (требуются права администратора)".to_string(),
        Some(2) => "Файл не найден".to_string(),
        Some(3) => "Путь не найден".to_string(),
        Some(13) => "Нет доступа к файлу".to_string(),
        Some(17) => "Файл уже существует".to_string(),
        Some(20) => "Не файл".to_string(),
        Some(21) => "Не директория".to_string(),
        Some(22) => "Неверный аргумент".to_string(),
        Some(28) => "Нет места на диске".to_string(),
        Some(30) => "Устройство не готово".to_string(),
        Some(31) => "Устройство не подключено".to_string(),
        Some(39) => "Диск заполнен".to_string(),
        Some(80) => "Файл уже существует".to_string(),
        Some(87) => "Неверный параметр".to_string(),
        Some(112) => "Диск заполнен".to_string(),
        Some(123) => "Неверное имя файла".to_string(),
        Some(145) => "Директория не пуста".to_string(),
        Some(183) => "Файл уже существует".to_string(),
        Some(206) => "Имя файла или расширение слишком длинное".to_string(),
        Some(267) => "Неверное имя директории".to_string(),
        Some(1005) => "Неверная операция с диском".to_string(),
        _ => {
            // Если это не системная ошибка, попробуем перевести по тексту
            let error_msg = error.to_string().to_lowercase();
            if error_msg.contains("permission denied") {
                "Нет доступа к файлу".to_string()
            } else if error_msg.contains("access denied") {
                "Нет доступа к файлу".to_string()
            } else if error_msg.contains("file not found") {
                "Файл не найден".to_string()
            } else if error_msg.contains("directory not empty") {
                "Директория не пуста".to_string()
            } else if error_msg.contains("no space left") {
                "Нет места на диске".to_string()
            } else if error_msg.contains("read-only") {
                "Файл только для чтения".to_string()
            } else if error_msg.contains("being used by another process") {
                "Файл используется другой программой".to_string()
            } else {
                format!("Ошибка: {}", error)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub category: String,
    pub total_files: usize,
    pub total_size: u64,
    pub files: Vec<FileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub modified: u64,
    pub is_directory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanResult {
    pub category: String,
    pub files_removed: usize,
    pub space_freed: u64,
    pub errors: Vec<String>,
}

pub struct CleanerService {
    config: AppConfig,
    scan_progress: f64,
    clean_progress: f64,
    scan_results: HashMap<String, ScanResult>,
}

impl CleanerService {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            scan_progress: 0.0,
            clean_progress: 0.0,
            scan_results: HashMap::new(),
        }
    }

    pub async fn scan_system(&mut self) -> Result<serde_json::Value, CleanerError> {
        self.scan_progress = 0.0;
        self.scan_results.clear();

        let total_categories = self.config.cleanup_categories.len();
        let mut processed = 0;

        for (category_name, category) in &self.config.cleanup_categories {
            if !category.enabled {
                processed += 1;
                self.scan_progress = processed as f64 / total_categories as f64;
                continue;
            }

            let result = self.scan_category(category_name, category).await?;
            self.scan_results.insert(category_name.clone(), result);
            
            processed += 1;
            self.scan_progress = processed as f64 / total_categories as f64;
        }

        Ok(serde_json::to_value(&self.scan_results)?)
    }

    async fn scan_category(&self, category_name: &str, category: &CleanupCategory) -> Result<ScanResult, CleanerError> {
        let mut total_files = 0;
        let mut total_size = 0u64;
        let mut files = Vec::new();

        for path in &category.paths {
            if !path.exists() {
                continue;
            }

            let walker = WalkDir::new(path)
                .follow_links(false)
                .max_depth(10);

            for entry in walker {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                let file_path = entry.path();
                
                // Проверяем, что файл не в исключенных путях
                if self.is_excluded(file_path) {
                    continue;
                }

                // Проверяем паттерн файла
                if !self.matches_pattern(file_path, &category.file_patterns) {
                    continue;
                }

                // Проверяем возраст файла
                if !self.is_old_enough(file_path, category.min_age_days) {
                    continue;
                }

                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                if metadata.is_file() {
                    let size = metadata.len();
                    
                    // Проверяем максимальный размер файла
                    if size > self.config.max_file_size_mb * 1024 * 1024 {
                        continue;
                    }

                    total_size += size;
                    total_files += 1;

                    let modified = metadata
                        .modified()
                        .unwrap_or(UNIX_EPOCH)
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or(Duration::from_secs(0))
                        .as_secs();

                    files.push(FileInfo {
                        path: file_path.to_string_lossy().to_string(),
                        size,
                        modified,
                        is_directory: false,
                    });
                }
            }
        }

        Ok(ScanResult {
            category: category_name.to_string(),
            total_files,
            total_size,
            files,
        })
    }

    pub async fn clean_categories(&mut self, categories: Vec<String>) -> Result<serde_json::Value, CleanerError> {
        self.clean_progress = 0.0;
        let mut results = Vec::new();
        let total_categories = categories.len();
        let mut processed = 0;

        for category_name in categories {
            if let Some(category) = self.config.cleanup_categories.get(&category_name) {
                if !category.enabled {
                    continue;
                }

                let result = self.clean_category(&category_name, category).await?;
                results.push(result);
            }
            
            processed += 1;
            self.clean_progress = processed as f64 / total_categories as f64;
        }

        Ok(serde_json::to_value(&results)?)
    }

    async fn clean_category(&self, category_name: &str, category: &CleanupCategory) -> Result<CleanResult, CleanerError> {
        let mut files_removed = 0;
        let mut space_freed = 0u64;
        let mut error_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for path in &category.paths {
            if !path.exists() {
                continue;
            }

            let walker = WalkDir::new(path)
                .follow_links(false)
                .max_depth(10);

            for entry in walker {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                let file_path = entry.path();
                
                if self.is_excluded(file_path) {
                    continue;
                }

                if !self.matches_pattern(file_path, &category.file_patterns) {
                    continue;
                }

                if !self.is_old_enough(file_path, category.min_age_days) {
                    continue;
                }

                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                if metadata.is_file() {
                    let size = metadata.len();
                    
                    if size > self.config.max_file_size_mb * 1024 * 1024 {
                        continue;
                    }

                    match std::fs::remove_file(file_path) {
                        Ok(_) => {
                            files_removed += 1;
                            space_freed += size;
                        }
                        Err(e) => {
                            let translated_error = translate_error(&e);
                            *error_counts.entry(translated_error).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        // Преобразуем подсчеты ошибок в читаемый формат
        let mut errors = Vec::new();
        for (error_type, count) in error_counts {
            if count == 1 {
                errors.push(error_type);
            } else {
                errors.push(format!("{} ({} файлов)", error_type, count));
            }
        }

        Ok(CleanResult {
            category: category_name.to_string(),
            files_removed,
            space_freed,
            errors,
        })
    }

    fn is_excluded(&self, path: &Path) -> bool {
        self.config.excluded_paths.iter().any(|excluded| {
            path.starts_with(excluded)
        })
    }

    fn matches_pattern(&self, path: &Path, patterns: &[String]) -> bool {
        if patterns.is_empty() {
            return true;
        }

        patterns.iter().any(|pattern| {
            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_string_lossy();
                self.matches_glob(&file_name, pattern)
            } else {
                false
            }
        })
    }

    fn matches_glob(&self, text: &str, pattern: &str) -> bool {
        // Простая реализация glob matching
        if pattern == "*" {
            return true;
        }
        
        if pattern.starts_with("*.") {
            let ext = &pattern[2..];
            return text.ends_with(ext);
        }
        
        text == pattern
    }

    fn is_old_enough(&self, path: &Path, min_age_days: u32) -> bool {
        if min_age_days == 0 {
            return true;
        }

        let metadata = match std::fs::metadata(path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let modified = metadata
            .modified()
            .unwrap_or(UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));

        let min_age = Duration::from_secs(min_age_days as u64 * 24 * 60 * 60);
        modified >= min_age
    }

    pub fn get_scan_progress(&self) -> f64 {
        self.scan_progress
    }

    pub fn get_clean_progress(&self) -> f64 {
        self.clean_progress
    }
}
