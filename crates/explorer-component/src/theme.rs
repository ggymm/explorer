use gpui::*;
use serde::{Deserialize, Serialize};

/// 主题模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
}

/// 主题颜色（基于 Fluent UI Design System）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    // 基础颜色
    pub background: Rgba,
    pub foreground: Rgba,
    pub border: Rgba,

    // 品牌色（Fluent UI 蓝色系）
    pub brand: Rgba,
    pub brand_foreground: Rgba,
    pub brand_hover: Rgba,
    pub brand_pressed: Rgba,
    pub brand_background_hover: Rgba, // 品牌色的浅背景（用于悬浮）

    // 中性色
    pub neutral_background: Rgba,
    pub neutral_foreground: Rgba,
    pub neutral_stroke: Rgba,

    // List 组件专用颜色
    pub list_item_background: Rgba,
    pub list_item_background_hover: Rgba,
    pub list_item_background_selected: Rgba,
    pub list_item_background_selected_hover: Rgba,
    pub list_item_foreground: Rgba,
    pub list_item_foreground_selected: Rgba,
    pub list_item_border_selected: Rgba,

    // 面包屑组件专用颜色
    pub breadcrumb_background: Rgba,
    pub breadcrumb_foreground: Rgba,
    pub breadcrumb_foreground_active: Rgba, // 最后一项（当前目录）的颜色
    pub breadcrumb_item_background_hover: Rgba,
    pub breadcrumb_border: Rgba,
    pub breadcrumb_border_active: Rgba, // 激活面板的边框颜色

    // 语义颜色
    pub success: Rgba,
    pub warning: Rgba,
    pub danger: Rgba,
    pub danger_foreground: Rgba,

    // 卡片和输入
    pub card: Rgba,
    pub card_foreground: Rgba,
    pub input: Rgba,

    // 静音色
    pub muted: Rgba,
    pub muted_foreground: Rgba,
}

/// 主题间距
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThemeSpacing {
    pub xs: Pixels,
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
    pub xl: Pixels,
    pub xxs: Pixels,
}

impl Default for ThemeSpacing {
    fn default() -> Self {
        Self {
            xxs: px(2.),
            xs: px(4.),
            sm: px(8.),
            md: px(16.),
            lg: px(24.),
            xl: px(32.),
        }
    }
}

/// 主题圆角（Fluent UI 使用较小的圆角）
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThemeRadius {
    pub sm: Pixels,
    pub md: Pixels,
    pub lg: Pixels,
}

impl Default for ThemeRadius {
    fn default() -> Self {
        Self {
            sm: px(2.), // Fluent UI 使用较小的圆角
            md: px(6.), // 中等圆角，使其更明显
            lg: px(8.),
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
    /// 暗色主题（基于 Fluent UI Design System）
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ThemeColors {
                // 基础颜色
                background: rgb(0x1f1f1f), // Neutral Background 1
                foreground: rgb(0xffffff), // White
                border: rgb(0x3b3b3b),     // Neutral Stroke 1

                // 品牌色（Fluent 蓝色）
                brand: rgb(0x0078d4), // Communication Blue
                brand_foreground: rgb(0xffffff),
                brand_hover: rgb(0x106ebe),
                brand_pressed: rgb(0x005a9e),
                brand_background_hover: rgba(0x0078d414), // 品牌色 8% 透明度

                // 中性色
                neutral_background: rgb(0x292929),
                neutral_foreground: rgb(0xe1e1e1),
                neutral_stroke: rgb(0x3b3b3b),

                // List 组件颜色（符合 Fluent UI 规范）
                list_item_background: rgba(0x00000000), // 透明
                list_item_background_hover: rgba(0xffffff0a), // 白色 4% 透明度
                list_item_background_selected: rgba(0xffffff14), // 白色 8% 透明度
                list_item_background_selected_hover: rgba(0xffffff1f), // 白色 12% 透明度
                list_item_foreground: rgb(0xffffff),
                list_item_foreground_selected: rgb(0xffffff),
                list_item_border_selected: rgb(0x0078d4), // 品牌色边框

                // 面包屑组件颜色
                breadcrumb_background: rgb(0x1f1f1f), // 与主背景一致
                breadcrumb_foreground: rgb(0xa3a3a3), // 普通项：muted 灰色
                breadcrumb_foreground_active: rgb(0xffffff), // 最后一项：白色
                breadcrumb_item_background_hover: rgba(0xffffff06), // 白色 2.5% 透明度
                breadcrumb_border: rgb(0x3b3b3b),     // 非激活：中性边框
                breadcrumb_border_active: rgb(0x0078d4), // 激活：品牌色

                // 语义颜色
                success: rgb(0x107c10),
                warning: rgb(0xfce100),
                danger: rgb(0xe81123),
                danger_foreground: rgb(0xffffff),

                // 卡片和输入
                card: rgb(0x292929),
                card_foreground: rgb(0xffffff),
                input: rgb(0x3b3b3b),

                // 静音色
                muted: rgb(0x3b3b3b),
                muted_foreground: rgb(0xa3a3a3),
            },
            spacing: ThemeSpacing::default(),
            radius: ThemeRadius::default(),
        }
    }

    /// 亮色主题（基于 Fluent UI Design System）
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ThemeColors {
                // 基础颜色
                background: rgb(0xffffff), // White
                foreground: rgb(0x242424), // Neutral Foreground 1
                border: rgb(0xd1d1d1),     // Neutral Stroke 1

                // 品牌色（Fluent 蓝色）
                brand: rgb(0x0078d4), // Communication Blue
                brand_foreground: rgb(0xffffff),
                brand_hover: rgb(0x106ebe),
                brand_pressed: rgb(0x005a9e),
                brand_background_hover: rgba(0x0078d414), // 品牌色 8% 透明度

                // 中性色
                neutral_background: rgb(0xf5f5f5),
                neutral_foreground: rgb(0x242424),
                neutral_stroke: rgb(0xd1d1d1),

                // List 组件颜色（符合 Fluent UI 规范）
                list_item_background: rgba(0x00000000), // 透明
                list_item_background_hover: rgba(0x0000000a), // 黑色 4% 透明度
                list_item_background_selected: rgba(0x00000014), // 黑色 8% 透明度
                list_item_background_selected_hover: rgba(0x0000001f), // 黑色 12% 透明度
                list_item_foreground: rgb(0x242424),
                list_item_foreground_selected: rgb(0x242424),
                list_item_border_selected: rgb(0x0078d4), // 品牌色边框

                // 面包屑组件颜色
                breadcrumb_background: rgb(0xffffff), // 与主背景一致
                breadcrumb_foreground: rgb(0x616161), // 普通项：muted 灰色
                breadcrumb_foreground_active: rgb(0x242424), // 最后一项：深色
                breadcrumb_item_background_hover: rgba(0x00000006), // 黑色 2.5% 透明度
                breadcrumb_border: rgb(0xd1d1d1),     // 非激活：中性边框
                breadcrumb_border_active: rgb(0x0078d4), // 激活：品牌色

                // 语义颜色
                success: rgb(0x107c10),
                warning: rgb(0xfce100),
                danger: rgb(0xe81123),
                danger_foreground: rgb(0xffffff),

                // 卡片和输入
                card: rgb(0xffffff),
                card_foreground: rgb(0x242424),
                input: rgb(0xf5f5f5),

                // 静音色
                muted: rgb(0xf5f5f5),
                muted_foreground: rgb(0x616161),
            },
            spacing: ThemeSpacing::default(),
            radius: ThemeRadius::default(),
        }
    }

    // 保留旧的颜色属性作为兼容性别名
    #[allow(dead_code)]
    pub fn primary(&self) -> Rgba {
        self.colors.brand
    }

    #[allow(dead_code)]
    pub fn primary_foreground(&self) -> Rgba {
        self.colors.brand_foreground
    }

    #[allow(dead_code)]
    pub fn accent(&self) -> Rgba {
        self.colors.brand
    }

    #[allow(dead_code)]
    pub fn accent_foreground(&self) -> Rgba {
        self.colors.brand_foreground
    }

    #[allow(dead_code)]
    pub fn destructive(&self) -> Rgba {
        self.colors.danger
    }

    #[allow(dead_code)]
    pub fn destructive_foreground(&self) -> Rgba {
        self.colors.danger_foreground
    }
}

impl Global for Theme {}
