//! 虚拟滚动列表组件（完整实现）
//!
//! 基于 gpui-component 的 virtual_list 实现，简化为纵向滚动专用版本。
//! 仅渲染可视区域内的项目，实现高性能的大列表渲染。
//!
//! ## 特性
//!
//! - 支持不同高度的列表项
//! - 仅渲染可见范围内的项目
//! - 支持滚动到指定项
//! - 支持加载、错误、空状态显示
//! - 完整的 Element trait 集成
//!
//! ## 示例
//!
//! ```rust,ignore
//! use explorer_component::{VirtualList, VirtualListScrollHandle};
//! use gpui::{px, Size};
//! use std::rc::Rc;
//!
//! let scroll_handle = VirtualListScrollHandle::new();
//! let item_sizes = Rc::new(vec![Size { width: px(0.), height: px(30.) }; 1000]);
//!
//! VirtualList::new("my-list")
//!     .items(items)
//!     .item_sizes(item_sizes)
//!     .render_item(|item, index, theme| {
//!         div().child(format!("Item {}", index))
//!     })
//!     .track_scroll(&scroll_handle);
//! ```

use std::{cell::RefCell, cmp, rc::Rc};

use gpui::{
    AnyElement, App, AvailableSpace, Bounds, DeferredScrollToItem, Div, Element, ElementId,
    GlobalElementId, Half, Hitbox, Pixels, Point, ScrollHandle, ScrollStrategy, Size, Stateful,
    StatefulInteractiveElement, StyleRefinement, Window, div, point, prelude::*, px, size,
};
use smallvec::SmallVec;

use crate::Theme;

/// 虚拟滚动句柄的内部状态
struct VirtualListScrollHandleState {
    items_count: usize,
    deferred_scroll_to_item: Option<DeferredScrollToItem>,
}

/// 虚拟列表滚动句柄
///
/// 用于控制虚拟列表的滚动行为
#[derive(Clone)]
pub struct VirtualListScrollHandle {
    state: Rc<RefCell<VirtualListScrollHandleState>>,
    base_handle: ScrollHandle,
}

impl VirtualListScrollHandle {
    /// 创建新的滚动句柄
    pub fn new() -> Self {
        VirtualListScrollHandle {
            state: Rc::new(RefCell::new(VirtualListScrollHandleState {
                items_count: 0,
                deferred_scroll_to_item: None,
            })),
            base_handle: ScrollHandle::default(),
        }
    }

    /// 获取基础滚动句柄
    pub fn base_handle(&self) -> &ScrollHandle {
        &self.base_handle
    }

    /// 滚动到指定索引的项目
    pub fn scroll_to_item(&self, ix: usize, strategy: ScrollStrategy) {
        let mut state = self.state.borrow_mut();
        state.deferred_scroll_to_item = Some(DeferredScrollToItem {
            item_index: ix,
            strategy,
            offset: 0,
            scroll_strict: false,
        });
    }

    /// 滚动到列表底部
    pub fn scroll_to_bottom(&self) {
        let items_count = self.state.borrow().items_count;
        self.scroll_to_item(items_count.saturating_sub(1), ScrollStrategy::Top);
    }

    /// 获取当前滚动偏移
    pub fn offset(&self) -> Point<Pixels> {
        self.base_handle.offset()
    }

    /// 设置滚动偏移
    pub fn set_offset(&self, offset: Point<Pixels>) {
        self.base_handle.set_offset(offset);
    }
}

impl Default for VirtualListScrollHandle {
    fn default() -> Self {
        Self::new()
    }
}

/// 项目尺寸布局信息
#[derive(Default, Clone)]
struct ItemSizeLayout {
    items_sizes: Rc<Vec<Size<Pixels>>>,
    content_size: Size<Pixels>,
    sizes: Vec<Pixels>,
    origins: Vec<Pixels>,
}

/// 虚拟列表帧状态
pub struct VirtualListFrameState {
    items: SmallVec<[AnyElement; 32]>,
    size_layout: ItemSizeLayout,
}

/// 项目渲染回调
pub type VirtualListItemRenderer<T> = Rc<dyn Fn(&T, usize, &Theme) -> AnyElement>;

/// 虚拟滚动列表组件
///
/// 完整实现，仅渲染可视区域内的项目
pub struct VirtualList<T: Clone + 'static> {
    id: ElementId,
    base: Stateful<Div>,
    scroll_handle: VirtualListScrollHandle,
    items: Vec<T>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    render_item: Option<VirtualListItemRenderer<T>>,
    loading: bool,
    error: Option<String>,
    empty_text: String,
    loading_text: String,
}

impl<T: Clone + 'static> VirtualList<T> {
    /// 创建新的虚拟列表
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        let scroll_handle = VirtualListScrollHandle::new();

        Self {
            base: div()
                .id(id.clone())
                .size_full()
                .overflow_scroll()
                .track_scroll(scroll_handle.base_handle()),
            id,
            scroll_handle,
            items: Vec::new(),
            item_sizes: Rc::new(Vec::new()),
            render_item: None,
            loading: false,
            error: None,
            empty_text: "暂无数据".to_string(),
            loading_text: "加载中...".to_string(),
        }
    }

    /// 设置列表项数据
    pub fn items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    /// 设置每个项目的高度
    pub fn item_sizes(mut self, item_sizes: Rc<Vec<Size<Pixels>>>) -> Self {
        self.item_sizes = item_sizes;
        self
    }

    /// 设置项目渲染回调
    pub fn render_item<F>(mut self, renderer: F) -> Self
    where
        F: Fn(&T, usize, &Theme) -> AnyElement + 'static,
    {
        self.render_item = Some(Rc::new(renderer));
        self
    }

    /// 设置加载状态
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// 设置错误信息
    pub fn error(mut self, error: Option<String>) -> Self {
        self.error = error;
        self
    }

    /// 设置空状态提示文本
    pub fn empty_text(mut self, text: impl Into<String>) -> Self {
        self.empty_text = text.into();
        self
    }

    /// 设置加载中提示文本
    pub fn loading_text(mut self, text: impl Into<String>) -> Self {
        self.loading_text = text.into();
        self
    }

    /// 设置滚动句柄
    pub fn track_scroll(mut self, scroll_handle: &VirtualListScrollHandle) -> Self {
        self.scroll_handle = scroll_handle.clone();
        self.base = div()
            .id(self.id.clone())
            .size_full()
            .overflow_scroll()
            .track_scroll(scroll_handle.base_handle());
        self
    }

    /// 渲染状态消息
    fn render_state_message(&self, message: &str, is_error: bool, theme: &Theme) -> Div {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .child(
                div()
                    .text_sm()
                    .text_color(if is_error {
                        theme.colors.danger
                    } else {
                        theme.colors.muted_foreground
                    })
                    .child(message.to_string()),
            )
    }

    /// 处理延迟滚动到项目
    fn scroll_to_deferred_item(
        &self,
        scroll_offset: Point<Pixels>,
        items_bounds: &[Bounds<Pixels>],
        content_bounds: &Bounds<Pixels>,
        scroll_to_item: DeferredScrollToItem,
    ) -> Point<Pixels> {
        let Some(bounds) = items_bounds
            .get(scroll_to_item.item_index + scroll_to_item.offset)
            .cloned()
        else {
            return scroll_offset;
        };

        let mut scroll_offset = scroll_offset;
        match scroll_to_item.strategy {
            ScrollStrategy::Center => {
                scroll_offset.y = content_bounds.top() + content_bounds.size.height.half()
                    - bounds.top()
                    - bounds.size.height.half()
            }
            _ => {
                // Top strategy or default
                if bounds.top() + scroll_offset.y < content_bounds.top() {
                    scroll_offset.y = content_bounds.top() - bounds.top()
                } else if bounds.bottom() + scroll_offset.y > content_bounds.bottom() {
                    scroll_offset.y = content_bounds.bottom() - bounds.bottom();
                }
            }
        }
        self.scroll_handle.set_offset(scroll_offset);
        scroll_offset
    }

    /// 测量第一个项目的尺寸
    fn measure_item(
        &self,
        list_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        if self.items.is_empty() {
            return Size::default();
        }

        let theme = cx.global::<Theme>();
        if let Some(renderer) = &self.render_item {
            let mut item_to_measure = renderer(&self.items[0], 0, theme);
            let available_space = size(
                list_width.map_or(AvailableSpace::MinContent, |width| {
                    AvailableSpace::Definite(width)
                }),
                AvailableSpace::MinContent,
            );
            item_to_measure.layout_as_root(available_space, window, cx)
        } else {
            Size::default()
        }
    }
}

impl<T: Clone + 'static> Styled for VirtualList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + 'static> IntoElement for VirtualList<T> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<T: Clone + 'static> Element for VirtualList<T> {
    type RequestLayoutState = VirtualListFrameState;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        // 如果处于非正常状态，直接返回简单布局
        if self.loading || self.error.is_some() || self.items.is_empty() {
            let layout_id = self.base.interactivity().request_layout(
                global_id,
                inspector_id,
                window,
                cx,
                |style, window, cx| {
                    window.with_text_style(style.text_style().cloned(), |window| {
                        window.request_layout(style, None, cx)
                    })
                },
            );

            return (
                layout_id,
                VirtualListFrameState {
                    items: SmallVec::new(),
                    size_layout: ItemSizeLayout::default(),
                },
            );
        }

        let rem_size = window.rem_size();
        let font_size = window.text_style().font_size.to_pixels(rem_size);
        let mut size_layout = ItemSizeLayout::default();
        let longest_item_size = self.measure_item(None, window, cx);
        let items_count = self.items.len();
        let item_sizes = self.item_sizes.clone();

        let layout_id = self.base.interactivity().request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| {
                size_layout = window.with_element_state(
                    global_id.unwrap(),
                    |state: Option<ItemSizeLayout>, _window| {
                        let mut state = state.unwrap_or_default();

                        let gap = style.gap.height.to_pixels(font_size.into(), rem_size);

                        if state.items_sizes != item_sizes {
                            state.items_sizes = item_sizes.clone();

                            // 计算每个项目的实际高度（包含 gap）
                            state.sizes = item_sizes
                                .iter()
                                .enumerate()
                                .map(|(i, size)| {
                                    let height = size.height;
                                    if i + 1 == items_count {
                                        height
                                    } else {
                                        height + gap
                                    }
                                })
                                .collect();

                            // 计算每个项目的起始 Y 坐标
                            state.origins = state
                                .sizes
                                .iter()
                                .scan(px(0.), |cumulative, &size| {
                                    let y = *cumulative;
                                    *cumulative += size;
                                    Some(y)
                                })
                                .collect();

                            // 计算总内容高度
                            state.content_size = Size {
                                width: longest_item_size.width,
                                height: state.sizes.iter().copied().sum(),
                            };
                        }

                        (state.clone(), state)
                    },
                );

                window.with_text_style(style.text_style().cloned(), |window| {
                    window.request_layout(style, None, cx)
                })
            },
        );

        (
            layout_id,
            VirtualListFrameState {
                items: SmallVec::new(),
                size_layout,
            },
        )
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        // 如果处于非正常状态，直接返回
        if self.loading || self.error.is_some() || self.items.is_empty() {
            return self.base.interactivity().prepaint(
                global_id,
                inspector_id,
                bounds,
                size(px(0.), px(0.)),
                window,
                cx,
                |_style, _scroll_offset, hitbox, _window, _cx| hitbox,
            );
        }

        let item_sizes = &layout.size_layout.sizes;
        let item_origins = &layout.size_layout.origins;

        // 简化：直接使用 bounds 作为 content_bounds，不考虑 border 和 padding
        let content_bounds = bounds;

        // 计算项目边界
        let items_bounds: Vec<Bounds<Pixels>> = item_origins
            .iter()
            .enumerate()
            .map(|(i, &origin)| {
                let item_size = item_sizes[i];
                Bounds {
                    origin: point(px(0.), content_bounds.top() + origin),
                    size: size(content_bounds.size.width, item_size),
                }
            })
            .collect();

        // 更新滚动句柄状态
        let mut scroll_state = self.scroll_handle.state.borrow_mut();
        scroll_state.items_count = self.items.len();

        let mut scroll_offset = self.scroll_handle.offset();
        if let Some(scroll_to_item) = scroll_state.deferred_scroll_to_item.take() {
            scroll_offset = self.scroll_to_deferred_item(
                scroll_offset,
                &items_bounds,
                &content_bounds,
                scroll_to_item,
            );
        }
        drop(scroll_state);

        // 限制滚动偏移
        let min_scroll_y = content_bounds.size.height - layout.size_layout.content_size.height;
        scroll_offset.y = scroll_offset.y.max(min_scroll_y).min(px(0.));
        if scroll_offset.y != self.scroll_handle.offset().y {
            self.scroll_handle.set_offset(scroll_offset);
        }

        let items = self.items.clone();
        let render_item = self.render_item.clone();
        let items_count = self.items.len();

        self.base.interactivity().prepaint(
            global_id,
            inspector_id,
            bounds,
            layout.size_layout.content_size,
            window,
            cx,
            |_style, scroll_offset, hitbox, window, cx| {
                if items_count > 0 && render_item.is_some() {
                    // 计算可见范围
                    let mut cumulative_size = px(0.);
                    let mut first_visible_ix = 0;
                    for (i, &size) in item_sizes.iter().enumerate() {
                        cumulative_size += size;
                        if cumulative_size > -scroll_offset.y {
                            first_visible_ix = i;
                            break;
                        }
                    }

                    cumulative_size = px(0.);
                    let mut last_visible_ix = 0;
                    for (i, &size) in item_sizes.iter().enumerate() {
                        cumulative_size += size;
                        if cumulative_size > (-scroll_offset.y + content_bounds.size.height) {
                            last_visible_ix = i + 1;
                            break;
                        }
                    }
                    if last_visible_ix == 0 {
                        last_visible_ix = items_count;
                    } else {
                        last_visible_ix = (last_visible_ix + 1).min(items_count);
                    }

                    let visible_range = first_visible_ix..cmp::min(last_visible_ix, items_count);

                    // 渲染可见项
                    let renderer = render_item.as_ref().unwrap();

                    for ix in visible_range {
                        let item = &items[ix];
                        let theme = cx.global::<Theme>();
                        let mut element = renderer(item, ix, theme);

                        let item_origin = content_bounds.origin
                            + point(scroll_offset.x, item_origins[ix] + scroll_offset.y);

                        let available_space = size(
                            AvailableSpace::Definite(content_bounds.size.width),
                            AvailableSpace::Definite(item_sizes[ix]),
                        );

                        element.layout_as_root(available_space, window, cx);
                        element.prepaint_at(item_origin, window, cx);
                        layout.items.push(element);
                    }
                }

                hitbox
            },
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.base.interactivity().paint(
            global_id,
            inspector_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |_, window, cx| {
                for item in &mut layout.items {
                    item.paint(window, cx);
                }
            },
        );
    }
}

impl<T: Clone + 'static> RenderOnce for VirtualList<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        // 显示加载状态
        if self.loading {
            return self
                .render_state_message(&self.loading_text, false, theme)
                .into_any_element();
        }

        // 显示错误状态
        if let Some(error_msg) = &self.error {
            return self
                .render_state_message(error_msg, true, theme)
                .into_any_element();
        }

        // 显示空状态
        if self.items.is_empty() {
            return self
                .render_state_message(&self.empty_text, false, theme)
                .into_any_element();
        }

        // 正常渲染虚拟列表
        self.into_any_element()
    }
}
