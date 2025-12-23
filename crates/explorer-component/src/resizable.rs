use std::ops::Range;

use gpui::{prelude::*, *};

use crate::Theme;

const HANDLE_SIZE: Pixels = px(3.);

/// 可调整大小组件的状态管理
pub struct ResizableState {
    axis: Axis,
    size: Pixels,
    range: Range<Pixels>,
    bounds: Bounds<Pixels>,
    resizing: bool,
}

impl ResizableState {
    pub fn new(axis: Axis, size: Pixels, range: Range<Pixels>) -> Self {
        Self {
            axis,
            size,
            range,
            bounds: Bounds::default(),
            resizing: false,
        }
    }

    /// 获取第一个面板的大小
    pub fn size(&self) -> Pixels {
        self.size
    }

    /// 获取容器的边界
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    /// 设置第一个面板的大小
    fn resize_first_panel(&mut self, new_size: Pixels, cx: &mut Context<Self>) {
        let container_size = match self.axis {
            Axis::Horizontal => self.bounds.size.width,
            Axis::Vertical => self.bounds.size.height,
        };

        if container_size.is_zero() {
            return;
        }

        // 限制大小在 range 之间
        let clamped_size = new_size
            .max(self.range.start)
            .min(self.range.end)
            .min(container_size); // 允许第一个面板占满容器，不限制第二个面板的最小尺寸

        self.size = clamped_size;

        cx.notify();
    }

    /// 开始调整大小
    fn start_resizing(&mut self, cx: &mut Context<Self>) {
        self.resizing = true;
        cx.notify();
    }

    /// 结束调整大小
    fn stop_resizing(&mut self, cx: &mut Context<Self>) {
        self.resizing = false;
        cx.notify();
    }
}

impl Render for ResizableState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

/// 可调整大小的两栏布局组件
#[derive(IntoElement)]
pub struct Resizable {
    id: ElementId,
    axis: Axis,
    size: Pixels,
    range: Range<Pixels>,
    first: AnyElement,
    second: AnyElement,
    state: Option<Entity<ResizableState>>,
}

impl Resizable {
    pub fn new(
        id: impl Into<ElementId>,
        first: impl IntoElement,
        second: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            axis: Axis::Horizontal,
            size: px(240.),
            range: px(180.)..px(480.),
            first: first.into_any_element(),
            second: second.into_any_element(),
            state: None,
        }
    }

    /// 设置方向（横向或纵向）
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// 设置初始宽度/高度
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    /// 设置大小范围
    pub fn range(mut self, range: Range<Pixels>) -> Self {
        self.range = range;
        self
    }

    /// 绑定到现有的状态实体
    pub fn with_state(mut self, state: Entity<ResizableState>) -> Self {
        self.state = Some(state);
        self
    }
}

impl RenderOnce for Resizable {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // 获取或创建状态
        let state = self.state.unwrap_or_else(|| {
            window.use_keyed_state(self.id.clone(), cx, |_, _| {
                ResizableState::new(self.axis, self.size, self.range.clone())
            })
        });

        // 获取第一个面板的大小
        let first_size = state.read(cx).size();
        let is_resizing = state.read(cx).resizing;

        let theme = cx.global::<Theme>();
        let axis = self.axis;
        let resizable_id = self.id.clone();  // 保存 ID 用于生成唯一的手柄 ID

        let container_state = state.clone();

        // 使用绝对定位布局，不使用 flex
        let content = div()
            .relative()
            .size_full()
            // 第一个面板（横向时在左侧，纵向时在顶部）
            .child({
                let panel = div()
                    .absolute()
                    .flex()
                    .flex_col()
                    .child(self.first);

                // 根据方向设置位置和尺寸
                match axis {
                    Axis::Horizontal => {
                        // 横向：第一个面板在左侧
                        panel.left_0().top_0().h_full().w(first_size)
                    }
                    Axis::Vertical => {
                        // 纵向：第一个面板在顶部
                        panel.left_0().top_0().w_full().h(first_size)
                    }
                }
            })
            // 第二个面板（横向时在右侧，纵向时在底部）
            .child({
                let panel = div()
                    .absolute()
                    .flex()
                    .flex_col()
                    .child(self.second);

                // 根据方向设置位置和尺寸
                match axis {
                    Axis::Horizontal => {
                        // 横向：第二个面板在右侧，从 first_size 开始到容器右边界
                        panel.left(first_size).top_0().right_0().h_full()
                    }
                    Axis::Vertical => {
                        // 纵向：第二个面板在底部，从 first_size 开始到容器底边界
                        panel.left_0().top(first_size).bottom_0().w_full()
                    }
                }
            })
            // 拖拽手柄（独立元素，放在最上层）
            .child({
                let handle = match axis {
                    Axis::Horizontal => div()
                        .id(ElementId::Name(
                            format!("{}-resize-handle", resizable_id.clone()).into(),
                        ))
                        .absolute()
                        .left(first_size - HANDLE_SIZE / 2.0)
                        .top_0()
                        .h_full()
                        .w(HANDLE_SIZE)
                        .cursor_col_resize()
                        .bg(if is_resizing {
                            theme.colors.accent
                        } else {
                            theme.colors.border
                        })
                        .hover(|style| style.bg(theme.colors.accent)),
                    Axis::Vertical => div()
                        .id(ElementId::Name(
                            format!("{}-resize-handle", resizable_id.clone()).into(),
                        ))
                        .absolute()
                        .top(first_size - HANDLE_SIZE / 2.0)
                        .left_0()
                        .w_full()
                        .h(HANDLE_SIZE)
                        .cursor_row_resize()
                        .bg(if is_resizing {
                            theme.colors.accent
                        } else {
                            theme.colors.border
                        })
                        .hover(|style| style.bg(theme.colors.accent)),
                };

                handle.on_mouse_down(MouseButton::Left, {
                    let state = state.clone();
                    move |_, _, cx| {
                        state.update(cx, |s, cx| s.start_resizing(cx));
                    }
                })
            })
            // 全局鼠标移动事件处理
            .child(ResizableMouseHandler {
                state: state.clone(),
                axis,
            })
            .into_any_element();

        ResizableContainer {
            id: self.id.clone(),
            state: container_state,
            content,
        }
    }
}

/// 容器元素，用于捕获bounds
struct ResizableContainer {
    id: ElementId,
    state: Entity<ResizableState>,
    content: AnyElement,
}

impl IntoElement for ResizableContainer {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ResizableContainer {
    type RequestLayoutState = LayoutId;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = self.content.request_layout(window, cx);
        (layout_id, layout_id)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        // 更新状态中的bounds，如果bounds发生变化则通知重新渲染
        self.state.update(cx, |s, cx| {
            if s.bounds != bounds {
                s.bounds = bounds;
                cx.notify();
            }
        });

        self.content.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.content.paint(window, cx);
    }
}

/// 处理鼠标移动和释放事件的隐藏元素
struct ResizableMouseHandler {
    state: Entity<ResizableState>,
    axis: Axis,
}

impl IntoElement for ResizableMouseHandler {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ResizableMouseHandler {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (window.request_layout(Style::default(), None, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Window,
        _: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let state = self.state.clone();
        let axis = self.axis;

        // 处理鼠标移动事件
        window.on_mouse_event({
            let state = state.clone();
            move |e: &MouseMoveEvent, phase, _, cx| {
                if !phase.bubble() {
                    return;
                }

                // 动态检查是否正在调整大小
                let is_resizing = state.read(cx).resizing;
                if !is_resizing {
                    return;
                }

                let bounds = state.read(cx).bounds;

                // 根据方向计算新的大小
                let new_size = match axis {
                    Axis::Horizontal => e.position.x - bounds.left(),
                    Axis::Vertical => e.position.y - bounds.top(),
                };

                state.update(cx, |s, cx| {
                    s.resize_first_panel(new_size, cx);
                });
            }
        });

        // 处理鼠标释放事件
        window.on_mouse_event({
            let state = state.clone();
            move |_: &MouseUpEvent, phase, _, cx| {
                if !phase.bubble() {
                    return;
                }

                // 动态检查是否正在调整大小
                let is_resizing = state.read(cx).resizing;
                if !is_resizing {
                    return;
                }

                state.update(cx, |s, cx| s.stop_resizing(cx));
            }
        });
    }
}
