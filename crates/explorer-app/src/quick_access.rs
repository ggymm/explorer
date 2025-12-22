use dirs;

use explorer_common::QuickAccessItem;

/// 获取快捷访问项列表（跨平台）
pub fn get_quick_access_items() -> Vec<QuickAccessItem> {
    let mut items = Vec::new();

    // 主文件夹
    if let Some(home) = dirs::home_dir() {
        items.push(QuickAccessItem {
            name: "主文件夹".to_string(),
            path: home.to_string_lossy().to_string(),
            icon: "home".to_string(),
        });
    }

    // 桌面
    if let Some(desktop) = dirs::desktop_dir() {
        items.push(QuickAccessItem {
            name: "桌面".to_string(),
            path: desktop.to_string_lossy().to_string(),
            icon: "desktop".to_string(),
        });
    }

    // 文档
    if let Some(documents) = dirs::document_dir() {
        items.push(QuickAccessItem {
            name: "文档".to_string(),
            path: documents.to_string_lossy().to_string(),
            icon: "documents".to_string(),
        });
    }

    // 下载
    if let Some(downloads) = dirs::download_dir() {
        items.push(QuickAccessItem {
            name: "下载".to_string(),
            path: downloads.to_string_lossy().to_string(),
            icon: "downloads".to_string(),
        });
    }

    // 图片
    if let Some(pictures) = dirs::picture_dir() {
        items.push(QuickAccessItem {
            name: "图片".to_string(),
            path: pictures.to_string_lossy().to_string(),
            icon: "pictures".to_string(),
        });
    }

    // 音乐
    if let Some(audio) = dirs::audio_dir() {
        items.push(QuickAccessItem {
            name: "音乐".to_string(),
            path: audio.to_string_lossy().to_string(),
            icon: "music".to_string(),
        });
    }

    // 视频
    if let Some(videos) = dirs::video_dir() {
        items.push(QuickAccessItem {
            name: "视频".to_string(),
            path: videos.to_string_lossy().to_string(),
            icon: "videos".to_string(),
        });
    }

    items
}
