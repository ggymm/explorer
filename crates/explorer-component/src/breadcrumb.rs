use std::rc::Rc;

use gpui::{prelude::*, *};

use crate::{IconName, Theme};

/// 面包屑状态
pub struct BreadcrumbState {
}

impl BreadcrumbState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for BreadcrumbState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

/// 面包屑项数据
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    /// 显示文本
    pub label: String,
    /// 关联数据（如完整路径）
    pub value: String,
}

impl BreadcrumbItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

/// 面包屑导航组件
#[derive(IntoElement)]
pub struct Breadcrumb {
    id: ElementId,
    items: Vec<BreadcrumbItem>,
    is_active: bool,
    on_navigate: Option<Rc<dyn Fn(String, &mut Window, &mut App)>>,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    state: Option<Entity<BreadcrumbState>>,
}

impl Breadcrumb {
    /// 创建新的面包屑
    pub fn new() -> Self {
        Self {
            id: ElementId::Name("breadcrumb".into()),
            items: Vec::new(),
            is_active: false,
            on_navigate: None,
            prefix: None,
            suffix: None,
            state: None,
        }
    }

    /// 设置面包屑项
    pub fn items(mut self, items: Vec<BreadcrumbItem>) -> Self {
        self.items = items;
        self
    }

    /// 设置是否激活
    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

    /// 设置导航回调
    pub fn on_navigate<F>(mut self, f: F) -> Self
    where
        F: Fn(String, &mut Window, &mut App) + 'static,
    {
        self.on_navigate = Some(Rc::new(f));
        self
    }

    /// 设置前缀元素
    pub fn prefix(mut self, element: impl IntoElement) -> Self {
        self.prefix = Some(element.into_any_element());
        self
    }

    /// 设置后缀元素
    pub fn suffix(mut self, element: impl IntoElement) -> Self {
        self.suffix = Some(element.into_any_element());
        self
    }

    /// 设置状态实体
    pub fn state(mut self, state: Entity<BreadcrumbState>) -> Self {
        self.state = Some(state);
        self
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
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

        // 构建面包屑链
        let mut breadcrumb_elements = vec![];

        for (i, item) in self.items.iter().enumerate() {
            let is_last = i == self.items.len() - 1;
            let item_clone = item.clone();

            // 渲染面包屑项
            let item_element = div()
                .px(theme.spacing.xs)
                .py(theme.spacing.xs)
                .rounded(theme.radius.sm)
                .cursor_pointer()
                .hover(|style| style.bg(theme.colors.muted))
                .when(is_last, |this| {
                    this.font_weight(FontWeight::SEMIBOLD)
                        .text_color(if self.is_active {
                            theme.colors.accent_foreground
                        } else {
                            theme.colors.foreground
                        })
                })
                .when(!is_last, |this| this.text_color(text_color))
                // 单个元素文本溢出省略
                .max_w(px(200.))
                .overflow_hidden()
                .child(
                    div()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .child(item.label.clone())
                )
                .when_some(self.on_navigate.clone(), move |this, callback| {
                    let value = item_clone.value.clone();
                    this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        callback(value.clone(), window, cx);
                    })
                });

            breadcrumb_elements.push(item_element.into_any_element());

            // 添加分隔符（除了最后一个）
            if !is_last {
                breadcrumb_elements.push(
                    div()
                        .mx(theme.spacing.xs)
                        .child(IconName::ChevronRight)
                        .text_color(theme.colors.muted_foreground)
                        .into_any_element(),
                );
            }
        }

        // 主容器
        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(32.))
            .w_full()
            .px(theme.spacing.md)
            .bg(bg_color)
            .border_b_1()
            .border_color(border_color)
            // 前缀（可选）
            .when_some(self.prefix, |this: gpui::Div, prefix| {
                this.child(
                    div()
                        .flex_shrink_0()
                        .mr(theme.spacing.sm)
                        .child(prefix),
                )
            })
            // 中间：面包屑链
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .children(breadcrumb_elements),
                    )
            )
            // 后缀（可选）
            .when_some(self.suffix, |this: gpui::Div, suffix| {
                this.child(
                    div()
                        .flex_shrink_0()
                        .ml(theme.spacing.sm)
                        .child(suffix),
                )
            })
    }
}
