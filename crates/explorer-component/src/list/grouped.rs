use gpui::{prelude::*, *};

use crate::Theme;

/// 列表分组
pub struct ListGroup<T: Clone + 'static> {
    /// 分组标题
    pub title: String,
    /// 分组项
    pub items: Vec<T>,
}

impl<T: Clone + 'static> ListGroup<T> {
    pub fn new(title: impl Into<String>, items: Vec<T>) -> Self {
        Self {
            title: title.into(),
            items,
        }
    }
}

/// 分组列表组件
///
/// 支持将列表项按分组显示，每个分组有标题
#[derive(IntoElement)]
pub struct GroupedList<T: Clone + 'static> {
    /// 分组列表
    groups: Vec<ListGroup<T>>,
    /// 项目渲染回调
    render_item: Option<Box<dyn Fn(&T, &Theme) -> AnyElement>>,
    /// 是否显示加载状态
    loading: bool,
    /// 错误信息
    error: Option<String>,
    /// 空状态提示文本
    empty_text: String,
    /// 加载中提示文本
    loading_text: String,
}

impl<T: Clone + 'static> GroupedList<T> {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            render_item: None,
            loading: false,
            error: None,
            empty_text: "暂无数据".to_string(),
            loading_text: "加载中...".to_string(),
        }
    }

    pub fn groups(mut self, groups: Vec<ListGroup<T>>) -> Self {
        self.groups = groups;
        self
    }

    pub fn render_item<F>(mut self, renderer: F) -> Self
    where
        F: Fn(&T, &Theme) -> AnyElement + 'static,
    {
        self.render_item = Some(Box::new(renderer));
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

    pub fn empty_text(mut self, text: impl Into<String>) -> Self {
        self.empty_text = text.into();
        self
    }

    pub fn loading_text(mut self, text: impl Into<String>) -> Self {
        self.loading_text = text.into();
        self
    }

    fn is_empty(&self) -> bool {
        self.groups.iter().all(|g| g.items.is_empty())
    }
}

impl<T: Clone + 'static> Default for GroupedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + 'static> RenderOnce for GroupedList<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .when(self.loading, |this| {
                let loading_text = self.loading_text.clone();
                this.size_full().child(
                    div().flex().items_center().justify_center().h_full().child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.muted_foreground)
                            .child(loading_text),
                    ),
                )
            })
            .when(self.error.is_some(), |this| {
                let error_msg = self.error.clone().unwrap();
                this.size_full().child(
                    div().flex().items_center().justify_center().h_full().child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.danger)
                            .child(error_msg),
                    ),
                )
            })
            .when(
                !self.loading && self.error.is_none() && self.is_empty(),
                |this| {
                    let empty_text = self.empty_text.clone();
                    this.size_full().child(
                        div().flex().items_center().justify_center().h_full().child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.muted_foreground)
                                .child(empty_text),
                        ),
                    )
                },
            )
            .when(
                !self.loading && self.error.is_none() && !self.is_empty(),
                |this| {
                    if let Some(renderer) = &self.render_item {
                        let mut container = div().flex().flex_col();

                        for (group_idx, group) in self.groups.iter().enumerate() {
                            if !group.items.is_empty() {
                                // 添加分组标题
                                container = container.child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.colors.muted_foreground)
                                        .mb(theme.spacing.sm)
                                        .when(group_idx > 0, |this| this.mt(theme.spacing.lg))
                                        .child(group.title.clone()),
                                );

                                // 添加分组项
                                container = container.child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(theme.spacing.xs)
                                        .mb(theme.spacing.sm)
                                        .children(
                                            group.items.iter().map(|item| renderer(item, theme)),
                                        ),
                                );
                            }
                        }

                        this.child(container)
                    } else {
                        this
                    }
                },
            )
    }
}
