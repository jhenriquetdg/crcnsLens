use egui::Pos2;
use egui::{pos2, vec2, FontId, Rect};
use egui::{Color32, Context, Id, LayerId, Order, Rounding, Stroke, Vec2};
use std::{fmt::Debug, time::Duration};

/// Anchor where to show toasts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// Top right corner.
    TopRight,
    /// Top left corner.
    TopLeft,
    /// Bottom right corner.
    BottomRight,
    /// Bottom left corner
    BottomLeft,
}

impl Anchor {
    #[inline]
    pub(crate) fn anim_side(&self) -> f32 {
        match self {
            Anchor::TopRight | Anchor::BottomRight => 1.,
            Anchor::TopLeft | Anchor::BottomLeft => -1.,
        }
    }
}

impl Anchor {
    pub(crate) fn screen_corner(&self, sc: Pos2, margin: Vec2) -> Pos2 {
        let mut out = match self {
            Anchor::TopRight => pos2(sc.x, 0.),
            Anchor::TopLeft => pos2(0., 0.),
            Anchor::BottomRight => sc,
            Anchor::BottomLeft => pos2(0., sc.y),
        };
        self.apply_margin(&mut out, margin);
        out
    }

    pub(crate) fn apply_margin(&self, pos: &mut Pos2, margin: Vec2) {
        match self {
            Anchor::TopRight => {
                pos.x -= margin.x;
                pos.y += margin.y;
            }
            Anchor::TopLeft => {
                pos.x += margin.x;
                pos.y += margin.y
            }
            Anchor::BottomRight => {
                pos.x -= margin.x;
                pos.y -= margin.y;
            }
            Anchor::BottomLeft => {
                pos.x += margin.x;
                pos.y -= margin.y;
            }
        }
    }
}

/// Level of importance
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ToastLevel {
    #[default]
    Info,
    Warning,
    Error,
    Success,
    None,
    Custom(String, egui::Color32),
}

#[derive(Debug)]
pub(crate) enum ToastState {
    Appear,
    Disapper,
    Disappeared,
    Idle,
}

impl ToastState {
    pub fn appearing(&self) -> bool {
        matches!(self, Self::Appear)
    }
    pub fn disappearing(&self) -> bool {
        matches!(self, Self::Disapper)
    }
    pub fn disappeared(&self) -> bool {
        matches!(self, Self::Disappeared)
    }
    pub fn idling(&self) -> bool {
        matches!(self, Self::Idle)
    }
}

/// Container for options for initlizing toasts
pub struct ToastOptions {
    duration: Option<Duration>,
    level: ToastLevel,
    closable: bool,
    show_progress_bar: bool,
}

/// Single notification or *toast*
#[derive(Debug)]
pub struct Toast {
    pub(crate) level: ToastLevel,
    pub(crate) caption: String,
    pub(crate) font: Option<FontId>,
    // (initial, current)
    pub(crate) duration: Option<(f32, f32)>,
    pub(crate) height: f32,
    pub(crate) width: f32,
    pub(crate) closable: bool,
    pub(crate) show_progress_bar: bool,

    pub(crate) state: ToastState,
    pub(crate) value: f32,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            duration: Some(Duration::from_millis(3500)),
            level: ToastLevel::None,
            closable: true,
            show_progress_bar: true,
        }
    }
}

fn duration_to_seconds_f32(duration: Duration) -> f32 {
    duration.as_nanos() as f32 * 1e-9
}

impl Toast {
    fn new(caption: impl Into<String>, options: ToastOptions) -> Self {
        Self {
            caption: caption.into(),
            height: TOAST_HEIGHT,
            width: TOAST_WIDTH,
            duration: if let Some(dur) = options.duration {
                let max_dur = duration_to_seconds_f32(dur);
                Some((max_dur, max_dur))
            } else {
                None
            },
            closable: options.closable,
            show_progress_bar: options.show_progress_bar,
            level: options.level,

            value: 0.,
            state: ToastState::Appear,
            font: None,
        }
    }

    /// Creates new basic toast, can be closed by default.
    pub fn basic(caption: impl Into<String>) -> Self {
        Self::new(caption, ToastOptions::default())
    }

    /// Creates new success toast, can be closed by default.
    pub fn success(caption: impl Into<String>) -> Self {
        Self::new(
            caption,
            ToastOptions {
                level: ToastLevel::Success,
                ..ToastOptions::default()
            },
        )
    }

    /// Creates new info toast, can be closed by default.
    pub fn info(caption: impl Into<String>) -> Self {
        Self::new(
            caption,
            ToastOptions {
                level: ToastLevel::Info,
                ..ToastOptions::default()
            },
        )
    }

    /// Creates new warning toast, can be closed by default.
    pub fn warning(caption: impl Into<String>) -> Self {
        Self::new(
            caption,
            ToastOptions {
                level: ToastLevel::Warning,
                ..ToastOptions::default()
            },
        )
    }

    /// Creates new error toast, can not be closed by default.
    pub fn error(caption: impl Into<String>) -> Self {
        Self::new(
            caption,
            ToastOptions {
                closable: false,
                level: ToastLevel::Error,
                ..ToastOptions::default()
            },
        )
    }

    /// Creates new custom toast, can be closed by default.
    pub fn custom(caption: impl Into<String>, level: ToastLevel) -> Self {
        Self::new(
            caption,
            ToastOptions {
                level,
                ..ToastOptions::default()
            },
        )
    }

    /// Set the options with a ToastOptions
    pub fn set_options(&mut self, options: ToastOptions) -> &mut Self {
        self.set_closable(options.closable);
        self.set_duration(options.duration);
        self.set_level(options.level);
        self
    }

    /// Change the level of the toast
    pub fn set_level(&mut self, level: ToastLevel) -> &mut Self {
        self.level = level;
        self
    }

    /// Changes the font used to draw the caption, it takes precedence over the value from
    /// [`Toasts`].
    pub fn set_font(&mut self, font: FontId) -> &mut Self {
        self.font = Some(font);
        self
    }

    /// Can use close the toast?
    pub fn set_closable(&mut self, closable: bool) -> &mut Self {
        self.closable = closable;
        self
    }

    /// Should a progress bar be shown?
    pub fn set_show_progress_bar(&mut self, show_progress_bar: bool) -> &mut Self {
        self.show_progress_bar = show_progress_bar;
        self
    }

    /// In what time should the toast expire? Set to `None` for no expiry.
    pub fn set_duration(&mut self, duration: Option<Duration>) -> &mut Self {
        if let Some(duration) = duration {
            let max_dur = duration_to_seconds_f32(duration);
            self.duration = Some((max_dur, max_dur));
        } else {
            self.duration = None;
        }
        self
    }

    /// Toast's box height
    pub fn set_height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }

    /// Toast's box width
    pub fn set_width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }

    /// Dismiss this toast
    pub fn dismiss(&mut self) {
        self.state = ToastState::Disapper;
    }

    pub(crate) fn calc_anchored_rect(&self, pos: Pos2, anchor: Anchor) -> Rect {
        match anchor {
            Anchor::TopRight => Rect {
                min: pos2(pos.x - self.width, pos.y),
                max: pos2(pos.x, pos.y + self.height),
            },
            Anchor::TopLeft => Rect {
                min: pos,
                max: pos + vec2(self.width, self.height),
            },
            Anchor::BottomRight => Rect {
                min: pos - vec2(self.width, self.height),
                max: pos,
            },
            Anchor::BottomLeft => Rect {
                min: pos2(pos.x, pos.y - self.height),
                max: pos2(pos.x + self.width, pos.y),
            },
        }
    }

    pub(crate) fn adjust_next_pos(&self, pos: &mut Pos2, anchor: Anchor, spacing: f32) {
        match anchor {
            Anchor::TopRight | Anchor::TopLeft => pos.y += self.height + spacing,
            Anchor::BottomRight | Anchor::BottomLeft => pos.y -= self.height + spacing,
        }
    }
}

#[doc(hidden)]

pub(crate) const TOAST_WIDTH: f32 = 180.;
pub(crate) const TOAST_HEIGHT: f32 = 34.;

const ERROR_COLOR: Color32 = Color32::from_rgb(200, 90, 90);
const INFO_COLOR: Color32 = Color32::from_rgb(150, 200, 210);
const WARNING_COLOR: Color32 = Color32::from_rgb(230, 220, 140);
const SUCCESS_COLOR: Color32 = Color32::from_rgb(140, 230, 140);

/// Main notifications collector.
/// # Usage
/// You need to create [`Toasts`] once and call `.show(ctx)` in every frame.
/// ```
/// # use std::time::Duration;
/// use egui_notify::Toasts;
///
/// # egui_notify::__run_test_ctx(|ctx| {
/// let mut t = Toasts::default();
/// t.info("Hello, World!").set_duration(Some(Duration::from_secs(5))).set_closable(true);
/// // More app code
/// t.show(ctx);
/// # });
/// ```
pub struct Toasts {
    toasts: Vec<Toast>,
    anchor: Anchor,
    margin: Vec2,
    spacing: f32,
    padding: Vec2,
    reverse: bool,
    speed: f32,
    font: Option<FontId>,

    held: bool,
}

impl Toasts {
    /// Creates new [`Toasts`] instance.
    pub const fn new() -> Self {
        Self {
            anchor: Anchor::TopRight,
            margin: vec2(8., 8.),
            toasts: vec![],
            spacing: 8.,
            padding: vec2(10., 10.),
            held: false,
            speed: 4.,
            reverse: false,
            font: None,
        }
    }

    /// Adds new toast to the collection.
    /// By default adds toast at the end of the list, can be changed with `self.reverse`.
    pub fn add(&mut self, toast: Toast) -> &mut Toast {
        if self.reverse {
            self.toasts.insert(0, toast);
            return self.toasts.get_mut(0).unwrap();
        } else {
            self.toasts.push(toast);
            let l = self.toasts.len() - 1;
            return self.toasts.get_mut(l).unwrap();
        }
    }

    /// Dismisses the oldest toast
    pub fn dismiss_oldest_toast(&mut self) {
        if let Some(toast) = self.toasts.get_mut(0) {
            toast.dismiss();
        }
    }

    /// Dismisses the most recent toast
    pub fn dismiss_latest_toast(&mut self) {
        if let Some(toast) = self.toasts.last_mut() {
            toast.dismiss();
        }
    }

    /// Dismisses all toasts
    pub fn dismiss_all_toasts(&mut self) {
        for toast in self.toasts.iter_mut() {
            toast.dismiss();
        }
    }

    /// Shortcut for adding a toast with info `success`.
    pub fn success(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::success(caption))
    }

    /// Shortcut for adding a toast with info `level`.
    pub fn info(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::info(caption))
    }

    /// Shortcut for adding a toast with warning `level`.
    pub fn warning(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::warning(caption))
    }

    /// Shortcut for adding a toast with error `level`.
    pub fn error(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::error(caption))
    }

    /// Shortcut for adding a toast with no level.
    pub fn basic(&mut self, caption: impl Into<String>) -> &mut Toast {
        self.add(Toast::basic(caption))
    }

    /// Shortcut for adding a toast with custom `level`.
    pub fn custom(
        &mut self,
        caption: impl Into<String>,
        level_string: String,
        level_color: egui::Color32,
    ) -> &mut Toast {
        self.add(Toast::custom(
            caption,
            ToastLevel::Custom(level_string, level_color),
        ))
    }

    /// Should toasts be added in reverse order?
    pub const fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Where toasts should appear.
    pub const fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets spacing between adjacent toasts.
    pub const fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Margin or distance from screen to toasts' bounding boxes
    pub const fn with_margin(mut self, margin: Vec2) -> Self {
        self.margin = margin;
        self
    }

    /// Padding or distance from toasts' bounding boxes to inner contents.
    pub const fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }

    /// Changes the default font used for all toasts.
    pub fn with_default_font(mut self, font: FontId) -> Self {
        self.font = Some(font);
        self
    }
}

impl Toasts {
    /// Displays toast queue
    pub fn show(&mut self, ctx: &Context) {
        let Self {
            anchor,
            margin,
            spacing,
            padding,
            toasts,
            held,
            speed,
            ..
        } = self;

        let mut pos = anchor.screen_corner(ctx.input(|i| i.screen_rect.max), *margin);
        let p = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("toasts")));

        let mut dismiss = None;

        // Remove disappeared toasts
        toasts.retain(|t| !t.state.disappeared());

        // Start disappearing expired toasts
        toasts.iter_mut().for_each(|t| {
            if let Some((_initial_d, current_d)) = t.duration {
                if current_d <= 0. {
                    t.state = ToastState::Disapper
                }
            }
        });

        // `held` used to prevent sticky removal
        if ctx.input(|i| i.pointer.primary_released()) {
            *held = false;
        }

        let visuals = ctx.style().visuals.widgets.noninteractive;
        let mut update = false;

        for (i, toast) in toasts.iter_mut().enumerate() {
            // Decrease duration if idling
            if let Some((_, d)) = toast.duration.as_mut() {
                if toast.state.idling() {
                    *d -= ctx.input(|i| i.stable_dt);
                    update = true;
                }
            }

            let caption_font = toast
                .font
                .as_ref()
                .or(self.font.as_ref())
                .or(ctx.style().override_font_id.as_ref())
                .cloned()
                .unwrap_or_else(|| FontId::proportional(16.));

            // Create toast label
            let caption_galley = ctx.fonts(|f| {
                f.layout(
                    toast.caption.clone(),
                    caption_font,
                    visuals.fg_stroke.color,
                    f32::INFINITY,
                )
            });

            let (caption_width, caption_height) =
                (caption_galley.rect.width(), caption_galley.rect.height());

            let line_count = toast.caption.chars().filter(|c| *c == '\n').count() + 1;
            let icon_width = caption_height / line_count as f32;

            // Create toast icon
            let icon_font = FontId::proportional(icon_width);
            let icon_galley = match &toast.level {
                ToastLevel::Info => {
                    Some(ctx.fonts(|f| f.layout("ℹ".into(), icon_font, INFO_COLOR, f32::INFINITY)))
                }
                ToastLevel::Warning => Some(
                    ctx.fonts(|f| f.layout("⚠".into(), icon_font, WARNING_COLOR, f32::INFINITY)),
                ),
                ToastLevel::Error => Some(
                    ctx.fonts(|f| f.layout("！".into(), icon_font, ERROR_COLOR, f32::INFINITY)),
                ),
                ToastLevel::Success => Some(
                    ctx.fonts(|f| f.layout("✅".into(), icon_font, SUCCESS_COLOR, f32::INFINITY)),
                ),
                ToastLevel::Custom(s, c) => {
                    Some(ctx.fonts(|f| f.layout(s.clone(), icon_font, *c, f32::INFINITY)))
                }
                ToastLevel::None => None,
            };

            let (action_width, action_height) = if let Some(icon_galley) = icon_galley.as_ref() {
                (icon_galley.rect.width(), icon_galley.rect.height())
            } else {
                (0., 0.)
            };

            // Create closing cross
            let cross_galley = if toast.closable {
                let cross_fid = FontId::proportional(icon_width);
                let cross_galley = ctx.fonts(|f| {
                    f.layout(
                        "❌".into(),
                        cross_fid,
                        visuals.fg_stroke.color,
                        f32::INFINITY,
                    )
                });
                Some(cross_galley)
            } else {
                None
            };

            let (cross_width, cross_height) = if let Some(cross_galley) = cross_galley.as_ref() {
                (cross_galley.rect.width(), cross_galley.rect.height())
            } else {
                (0., 0.)
            };

            let icon_x_padding = (0., padding.x);
            let cross_x_padding = (padding.x, 0.);

            let icon_width_padded = if icon_width == 0. {
                0.
            } else {
                icon_width + icon_x_padding.0 + icon_x_padding.1
            };
            let cross_width_padded = if cross_width == 0. {
                0.
            } else {
                cross_width + cross_x_padding.0 + cross_x_padding.1
            };

            toast.width = icon_width_padded + caption_width + cross_width_padded + (padding.x * 2.);
            toast.height = action_height.max(caption_height).max(cross_height) + padding.y * 2.;

            let anim_offset = toast.width * (1. - ease_in_cubic(toast.value));
            pos.x += anim_offset * anchor.anim_side();
            let rect = toast.calc_anchored_rect(pos, *anchor);

            // Required due to positioning of the next toast
            pos.x -= anim_offset * anchor.anim_side();

            // Draw background
            p.rect_filled(rect, Rounding::same(4.), visuals.bg_fill);

            // Paint icon
            if let Some((icon_galley, true)) =
                icon_galley.zip(Some(toast.level != ToastLevel::None))
            {
                let oy = toast.height / 2. - action_height / 2.;
                let ox = padding.x + icon_x_padding.0;
                p.galley(rect.min + vec2(ox, oy), icon_galley, Color32::BLACK);
            }

            // Paint caption
            let oy = toast.height / 2. - caption_height / 2.;
            let o_from_icon = if action_width == 0. {
                0.
            } else {
                action_width + icon_x_padding.1
            };
            let o_from_cross = if cross_width == 0. {
                0.
            } else {
                cross_width + cross_x_padding.0
            };
            let ox = (toast.width / 2. - caption_width / 2.) + o_from_icon / 2. - o_from_cross / 2.;
            p.galley(rect.min + vec2(ox, oy), caption_galley, Color32::BLACK);

            // Paint cross
            if let Some(cross_galley) = cross_galley {
                let cross_rect = cross_galley.rect;
                let oy = toast.height / 2. - cross_height / 2.;
                let ox = toast.width - cross_width - cross_x_padding.1 - padding.x;
                let cross_pos = rect.min + vec2(ox, oy);
                p.galley(cross_pos, cross_galley, Color32::BLACK);

                let screen_cross = Rect {
                    max: cross_pos + cross_rect.max.to_vec2(),
                    min: cross_pos,
                };

                if let Some(pos) = ctx.input(|i| i.pointer.press_origin()) {
                    if screen_cross.contains(pos) && !*held {
                        dismiss = Some(i);
                        *held = true;
                    }
                }
            }

            // Draw duration
            if toast.show_progress_bar {
                if let Some((initial, current)) = toast.duration {
                    if !toast.state.disappearing() {
                        p.line_segment(
                            [
                                rect.min + vec2(0., toast.height),
                                rect.max - vec2((1. - (current / initial)) * toast.width, 0.),
                            ],
                            Stroke::new(4., visuals.fg_stroke.color),
                        );
                    }
                }
            }

            toast.adjust_next_pos(&mut pos, *anchor, *spacing);

            // Animations
            if toast.state.appearing() {
                update = true;
                toast.value += ctx.input(|i| i.stable_dt) * (*speed);

                if toast.value >= 1. {
                    toast.value = 1.;
                    toast.state = ToastState::Idle;
                }
            } else if toast.state.disappearing() {
                update = true;
                toast.value -= ctx.input(|i| i.stable_dt) * (*speed);

                if toast.value <= 0. {
                    toast.state = ToastState::Disappeared;
                }
            }
        }

        if update {
            ctx.request_repaint();
        }

        if let Some(i) = dismiss {
            self.toasts[i].dismiss();
        }
    }
}

impl Default for Toasts {
    fn default() -> Self {
        Self::new()
    }
}

fn ease_in_cubic(x: f32) -> f32 {
    1. - (1. - x).powi(3)
}
