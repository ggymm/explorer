use std::rc::Rc;

use gpui::{prelude::*, *};

use crate::Theme;

/// 面板标题栏组件（仅显示路径）
#[derive(IntoElement)]
pub struct PanelTitleBar {
    current_path: String,
    is_active: bool,
    on_click: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl PanelTitleBar {
    /// 创建新的标题栏
    pub fn new(current_path: impl Into<String>) -> Self {
        Self {
            current_path: current_path.into(),
            is_active: false,
            on_click: None,
        }
    }

    /// 设置是否激活
    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

    /// 设置点击回调
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for PanelTitleBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        // 根据激活状态选择颜色
        let bg_color = if self.is_active {
            theme.colors.accent
        } else {
            theme.colors.background
        };

        let border_color = if self.is_active {
            theme.colors.accent
        } else {
            theme.colors.border
        };

        let text_color = if self.is_active {
            theme.colors.accent_foreground
        } else {
            theme.colors.muted_foreground
        };

        let mut title_bar = div()
            .flex()
            .items_center()
            .h(px(32.))
            .px(theme.spacing.md)
            .bg(bg_color)
            .border_b_1()
            .border_color(border_color)
            .cursor_pointer()
            .hover(|style| {
                if !self.is_active {
                    style.bg(theme.colors.muted)
                } else {
                    style
                }
            })
            .child(
                // 路径显示
                div()
                    .flex()
                    .items_center()
                    .text_sm()
                    .text_color(text_color)
                    .child(self.current_path),
            );

        // 如果有点击回调，添加点击事件
        if let Some(callback) = self.on_click {
            title_bar = title_bar.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                callback(window, cx);
            });
        }

        title_bar
    }
}
