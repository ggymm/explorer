use gpui::*;

/// 侧边栏布局组件
/// 提供左侧边栏和右侧主面板的两栏布局
pub struct SidebarLayout {
    sidebar_width: Pixels,
}

impl SidebarLayout {
    pub fn new() -> Self {
        Self {
            sidebar_width: px(250.0),
        }
    }

    pub fn sidebar_width(mut self, width: Pixels) -> Self {
        self.sidebar_width = width;
        self
    }

    pub fn render_with_children(
        self,
        sidebar: impl IntoElement,
        content: impl IntoElement,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .child(
                div()
                    .w(self.sidebar_width)
                    .h_full()
                    .flex_shrink_0()
                    .bg(rgb(0x2d2d2d))
                    .child(sidebar),
            )
            .child(div().flex_1().h_full().bg(rgb(0x1e1e1e)).child(content))
    }
}

impl Default for SidebarLayout {
    fn default() -> Self {
        Self::new()
    }
}
