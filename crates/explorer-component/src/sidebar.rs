use gpui::*;

use explorer_common::{ProviderType, RootItem};

/// 侧边栏组件
#[derive(IntoElement)]
pub struct Sidebar {
    roots: Vec<RootItem>,
    on_root_click: Option<Box<dyn Fn(&RootItem) + 'static>>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            on_root_click: None,
        }
    }

    pub fn roots(mut self, roots: Vec<RootItem>) -> Self {
        self.roots = roots;
        self
    }

    pub fn on_root_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&RootItem) + 'static,
    {
        self.on_root_click = Some(Box::new(callback));
        self
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p_2()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xcccccc))
                    .mb_2()
                    .child("存储位置"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(self.roots.iter().map(|root| {
                        let icon = match root.provider_type {
                            ProviderType::LocalFileSystem => "💾",
                            ProviderType::NetworkDrive => "🌐",
                            ProviderType::CloudStorage => "☁️",
                        };

                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .p_2()
                            .rounded_md()
                            .hover(|style| style.bg(rgb(0x3d3d3d)))
                            .cursor_pointer()
                            .child(div().text_base().child(icon))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xeeeeee))
                                    .child(root.name.clone()),
                            )
                    })),
            )
    }
}
