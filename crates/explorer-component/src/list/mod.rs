mod grouped;
mod virtual_list;

pub use grouped::*;
pub use virtual_list::*;

use std::rc::Rc;

use gpui::{prelude::*, *};

use crate::Theme;

/// 列表项渲染回调
pub type ListItemRenderer<T> = Box<dyn Fn(&T, &Theme) -> AnyElement>;

/// 通用列表组件
///
/// 可以渲染任何类型的项目列表，项目内容由外部自定义
#[derive(IntoElement)]
pub struct List<T: Clone + 'static> {
    /// 列表项数据
    items: Vec<T>,
    /// 项目渲染回调
    render_item: Option<ListItemRenderer<T>>,
    /// 是否显示加载状态
    loading: bool,
    /// 错误信息
    error: Option<String>,
    /// 空状态提示文本
    empty_text: String,
    /// 加载中提示文本
    loading_text: String,
}

impl<T: Clone + 'static> List<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            render_item: None,
            loading: false,
            error: None,
            empty_text: "暂无数据".to_string(),
            loading_text: "加载中...".to_string(),
        }
    }

    pub fn items(mut self, items: Vec<T>) -> Self {
        self.items = items;
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
}

impl<T: Clone + 'static> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + 'static> RenderOnce for List<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .when(self.loading, |this| {
                this.size_full().child(
                    div().flex().items_center().justify_center().h_full().child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.muted_foreground)
                            .child(self.loading_text),
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
                !self.loading && self.error.is_none() && self.items.is_empty(),
                |this| {
                    this.size_full().child(
                        div().flex().items_center().justify_center().h_full().child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.muted_foreground)
                                .child(self.empty_text),
                        ),
                    )
                },
            )
            .when(
                !self.loading && self.error.is_none() && !self.items.is_empty(),
                |this| {
                    if let Some(renderer) = &self.render_item {
                        let items = self.items.clone();
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing.xs)
                                .children(items.iter().map(|item| renderer(item, theme))),
                        )
                    } else {
                        this
                    }
                },
            )
    }
}

/// 列表项组件
///
/// 提供统一的列表项样式和交互
#[derive(IntoElement)]
pub struct ListItem {
    /// 唯一标识符
    id: SharedString,
    /// 是否选中
    selected: bool,
    /// 项目内容
    content: AnyElement,
    /// 点击回调
    on_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    /// 双击回调
    on_double_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl ListItem {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            content: div().into_any_element(),
            on_click: None,
            on_double_click: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn child(mut self, content: impl IntoElement) -> Self {
        self.content = content.into_any_element();
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn on_double_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_double_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let is_selected = self.selected;

        // 根据选中状态选择对应的颜色
        let bg_color = if is_selected {
            theme.colors.list_item_background_selected
        } else {
            theme.colors.list_item_background
        };

        let bg_hover_color = if is_selected {
            theme.colors.list_item_background_selected_hover
        } else {
            theme.colors.list_item_background_hover
        };

        // 构建容器：Fluent UI 风格的列表项（仅背景色高亮）
        let container = div()
            .id(self.id)
            .w_full()
            .flex()
            .items_center()
            .gap(theme.spacing.sm)
            .p_2()
            .rounded(theme.radius.md)
            .cursor_pointer()
            .bg(bg_color)
            .hover(move |style| style.bg(bg_hover_color))
            .child(self.content)
            // 事件处理：统一处理单击和双击
            .when_some(
                self.on_click.as_ref().or(self.on_double_click.as_ref()),
                |this, _| {
                    let click = self.on_click.clone();
                    let double_click = self.on_double_click.clone();
                    this.on_click(move |event, window, cx| match event.click_count() {
                        2 => {
                            if let Some(handler) = double_click.as_ref() {
                                handler(window, cx);
                            }
                        }
                        1 => {
                            if let Some(handler) = click.as_ref() {
                                handler(window, cx);
                            }
                        }
                        _ => {}
                    })
                },
            );

        container
    }
}
