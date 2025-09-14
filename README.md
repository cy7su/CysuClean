# CysuClean

[![Rust](https://img.shields.io/badge/Rust-1.70+-B73222?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org) [![Tauri](https://img.shields.io/badge/Tauri-1.8+-732022?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app) [![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-B73222?style=for-the-badge&logo=typescript&logoColor=white)](https://typescriptlang.org) [![License](https://img.shields.io/badge/License-MIT-732022?style=for-the-badge&logo=opensourceinitiative&logoColor=white)](LICENSE)

---

## 🚀 Speed

```bash
git clone https://github.com/cy7su/cysuclean.git
cd cysuclean
npm install
npm run tauri dev
```

---

## 📦 Build

```bash
# Development build
npm run tauri dev

# Production build
npm run tauri build
```

---

## ⚙️ Config

```bash
# Edit configuration in src-tauri/src/config.rs
# Add new cleanup categories and paths
```

📋 **Click to see all configuration options**

```rust
// 🧹 CLEANUP CATEGORIES
pub struct AppConfig {
    pub safe_mode: bool,           // Safe mode for file deletion
    pub backup_enabled: bool,      // Enable backup before cleaning
    pub max_file_size_mb: u64,     // Maximum file size to clean (MB)
    pub excluded_paths: Vec<PathBuf>, // Paths to exclude from cleaning
}

// 🗂️ SUPPORTED CATEGORIES
- temp_files: "Временные файлы"
- browser_cache: "Кеш браузеров"
- logs: "Логи системы"
- recycle_bin: "Корзина"
- windows_cache: "Кеш Windows"
- thumbnails: "Миниатюры"
- old_downloads: "Старые загрузки"
- app_cache: "Кеш приложений"
- installers: "Установщики"
- windows_update: "Windows Update"
- microsoft_store: "Microsoft Store"
- office_cache: "Кеш Office"
- visual_studio: "Visual Studio"
- dotnet_cache: "Кеш .NET"
- nodejs_cache: "Node.js кеш"
- python_cache: "Python кеш"
- java_cache: "Java кеш"
- adobe_cache: "Adobe кеш"
- nvidia_cache: "NVIDIA кеш"
- amd_cache: "AMD кеш"
- intel_cache: "Intel кеш"
- antivirus_cache: "Антивирус кеш"
- vpn_cache: "VPN кеш"
- torrent_cache: "Торрент кеш"
- media_cache: "Медиаплееры"
- games_cache: "Игровой кеш"
- system_utils: "Системные утилиты"
- archivers_cache: "Архиваторы"
- cloud_cache: "Облачные хранилища"
```

---


## 🚀 Launch

```bash
npm run tauri dev
# 🌐 Available at: http://localhost:1420
```

---

## 📋 Requirements

| Component   | Version | Description         |
| ----------- | ------- | ------------------- |
| **Node.js** | `18+`   | Frontend build tool |
| **Rust**    | `1.70+` | Backend language    |
| **Tauri**   | `1.8+`  | Desktop framework   |
| **Windows** | `10+`   | Target platform     |

---

[![GitHub](https://img.shields.io/badge/GitHub-cy7su%2Fcysuclean-B73222?style=for-the-badge&logo=github)](https://github.com/cy7su/cysuclean) [![Website](https://img.shields.io/badge/Website-cysu.ru-732022?style=for-the-badge&logo=firefox)](https://cysu.ru) [![Email](https://img.shields.io/badge/Email-contact%40cysu.ru-B73222?style=for-the-badge&logo=mail.ru)](mailto:support@cysu.ru)

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Made with ❤️ by cysu**
