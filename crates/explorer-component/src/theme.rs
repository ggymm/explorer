use gpui::*;
use serde::{Deserialize, Serialize};

/// 主题模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
}

/// 主题颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: Rgba,
    pub foreground: Rgba,
    pub card: Rgba,
    pub card_foreground: Rgba,
    pub border: Rgba,
    pub input: Rgba,
    pub primary: Rgba,
    pub primary_foreground: Rgba,
    pub secondary: Rgba,
    pub secondary_foreground: Rgba,
    pub accent: Rgba,
    pub accent_foreground: Rgba,
    pub muted: Rgba,
    pub muted_foreground: Rgba,
    pub destructive: Rgba,
    pub destructive_foreground: Rgba,
}

/// 主题间距
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThemeSpacing {
    pub xs: Pixels,
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
    pub xl: Pixels,
}

impl Default for ThemeSpacing {
    fn default() -> Self {
        Self {
            xs: px(4.),
            sm: px(8.),
            md: px(16.),
            lg: px(24.),
            xl: px(32.),
        }
    }
}

/// 主题圆角
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThemeRadius {
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
}

impl Default for ThemeRadius {
    fn default() -> Self {
        Self {
            sm: px(4.),
            md: px(8.),
            lg: px(12.),
        }
    }
}

/// 主题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub radius: ThemeRadius,
    pub spacing: ThemeSpacing,
}

impl Theme {
    /// 暗色主题（基于 shadcn/ui）
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ThemeColors {
                background: rgb(0x020617),
                foreground: rgb(0xf8fafc),
                card: rgb(0x020617),
                card_foreground: rgb(0xf8fafc),
                border: rgb(0x1e293b),
                input: rgb(0x1e293b),
                primary: rgb(0xf8fafc),
                primary_foreground: rgb(0x0f172a),
                secondary: rgb(0x1e293b),
                secondary_foreground: rgb(0xf8fafc),
                accent: rgb(0x1e293b),
                accent_foreground: rgb(0xf8fafc),
                muted: rgb(0x1e293b),
                muted_foreground: rgb(0x94a3b8),
                destructive: rgb(0x7f1d1d),
                destructive_foreground: rgb(0xf8fafc),
            },
            spacing: ThemeSpacing::default(),
            radius: ThemeRadius::default(),
        }
    }

    /// 亮色主题（基于 shadcn/ui）
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ThemeColors {
                background: rgb(0xffffff),
                foreground: rgb(0x020617),
                card: rgb(0xffffff),
                card_foreground: rgb(0x020617),
                border: rgb(0xe2e8f0),
                input: rgb(0xe2e8f0),
                primary: rgb(0x0f172a),
                primary_foreground: rgb(0xf8fafc),
                secondary: rgb(0xf1f5f9),
                secondary_foreground: rgb(0x0f172a),
                accent: rgb(0xf1f5f9),
                accent_foreground: rgb(0x0f172a),
                muted: rgb(0xf1f5f9),
                muted_foreground: rgb(0x64748b),
                destructive: rgb(0xef4444),
                destructive_foreground: rgb(0xf8fafc),
            },
            spacing: ThemeSpacing::default(),
            radius: ThemeRadius::default(),
        }
    }
}

impl Global for Theme {}
