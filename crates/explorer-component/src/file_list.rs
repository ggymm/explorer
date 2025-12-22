use gpui::{prelude::FluentBuilder, *};

use explorer_common::{FileItem, ItemType};

/// 文件列表组件
#[derive(IntoElement)]
pub struct FileList {
    entries: Vec<FileItem>,
    current_path: String,
    loading: bool,
    error: Option<String>,
}

impl FileList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current_path: String::new(),
            loading: false,
            error: None,
        }
    }

    pub fn entries(mut self, entries: Vec<FileItem>) -> Self {
        self.entries = entries;
        self
    }

    pub fn current_path(mut self, path: String) -> Self {
        self.current_path = path;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn error(mut self, error: Option<String>) -> Self {
        self.error = error;
        self
    }

    fn render_entry(&self, entry: &FileItem) -> impl IntoElement {
        let icon = match entry.item_type {
            ItemType::Directory => "📁",
            ItemType::File => "📄",
            ItemType::Symlink => "🔗",
        };

        let name_color = if entry.is_hidden {
            rgb(0x888888)
        } else {
            rgb(0xeeeeee)
        };

        div()
            .flex()
            .items_center()
            .gap_2()
            .p_2()
            .rounded_md()
            .hover(|style| style.bg(rgb(0x2d2d2d)))
            .cursor_pointer()
            .child(div().text_base().child(icon))
            .child(
                div()
                    .text_sm()
                    .text_color(name_color)
                    .child(entry.name.clone()),
            )
    }
}

impl Default for FileList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FileList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                // 路径栏
                div()
                    .flex()
                    .items_center()
                    .p_3()
                    .border_b_1()
                    .border_color(rgb(0x3d3d3d))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xcccccc))
                            .child(self.current_path.clone()),
                    ),
            )
            .child(
                // 文件列表内容
                div()
                    .flex_1()
                    .overflow_hidden()
                    .p_2()
                    .when(self.loading, |this: Div| {
                        this.child(
                            div().flex().items_center().justify_center().h_full().child(
                                div().text_sm().text_color(rgb(0x888888)).child("加载中..."),
                            ),
                        )
                    })
                    .when(self.error.is_some(), |this: Div| {
                        this.child(
                            div().flex().items_center().justify_center().h_full().child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xff6b6b))
                                    .child(self.error.clone().unwrap()),
                            ),
                        )
                    })
                    .when(
                        !self.loading && self.error.is_none() && self.entries.is_empty(),
                        |this: Div| {
                            this.child(
                                div().flex().items_center().justify_center().h_full().child(
                                    div().text_sm().text_color(rgb(0x888888)).child("目录为空"),
                                ),
                            )
                        },
                    )
                    .when(
                        !self.loading && self.error.is_none() && !self.entries.is_empty(),
                        |this: Div| {
                            this.child(div().flex().flex_col().gap_1().children(
                                self.entries.iter().map(|entry| self.render_entry(entry)),
                            ))
                        },
                    ),
            )
    }
}
