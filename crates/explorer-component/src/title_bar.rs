use gpui::{prelude::*, *};

use crate::Theme;

pub const TITLE_BAR_HEIGHT: Pixels = px(34.);

#[cfg(target_os = "macos")]
const TITLE_BAR_LEFT_PADDING: Pixels = px(80.);

#[cfg(not(target_os = "macos"))]
const TITLE_BAR_LEFT_PADDING: Pixels = px(12.);

/// 窗口标题栏组件
#[derive(IntoElement)]
pub struct TitleBar {
    children: Vec<AnyElement>,
}

impl TitleBar {
    /// 创建新的标题栏
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// 返回标题栏选项
    pub fn titlebar_options() -> TitlebarOptions {
        TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }
    }
}

impl ParentElement for TitleBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div().flex_shrink_0().child(
            div()
                .id("title-bar")
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(TITLE_BAR_HEIGHT)
                .pl(TITLE_BAR_LEFT_PADDING)
                .bg(theme.colors.background)
                .border_b_1()
                .border_color(theme.colors.border)
                .child(
                    div()
                        .id("bar")
                        .h_full()
                        .justify_between()
                        .flex_shrink_0()
                        .flex_1()
                        .flex()
                        .items_center()
                        .children(self.children),
                ),
        )
    }
}
