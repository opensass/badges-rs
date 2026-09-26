// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../DIOXUS.md")]

use crate::common::{
    Color, Placement, Shape, Size, Variant, base_anchor_style, base_badge_style, base_label_style,
};
use dioxus::prelude::*;

/// Props for the [`Anchor`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct AnchorProps {
    /// The target element and the [`Badge`] itself.
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the anchor wrapper `<span>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the anchor wrapper `<span>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the anchor wrapper element.
    #[props(default)]
    pub id: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

/// A positioning wrapper that makes a [`Badge`] absolute-positioned relative
/// to its sibling element.
///
/// Place the content element and a [`Badge`] as direct children. The badge
/// will anchor itself to a corner of the content element as determined by
/// its own [`Placement`] prop.
///
/// # Accessibility
///
/// - Renders as `<span>` with `position: relative` so it does not disrupt
///   document flow or semantic structure.
///
/// # Examples
///
/// ```rust
/// use badges_rs::dioxus::{Badge, Anchor};
/// use badges_rs::Color;
/// use dioxus::prelude::*;
///
/// fn MyBadge() -> Element {
///     rsx! {
///         Anchor {
///             span { "Icon" }
///             Badge { color: Color::Danger, "5" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Anchor(props: AnchorProps) -> Element {
    rsx! {
        span {
            id: props.id,
            class: "badge-anchor {props.class}",
            style: "{base_anchor_style()} {props.style}",
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Props for the [`Label`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// Text, number, or icon content for the label slot.
    #[props(default)]
    pub children: Element,

    /// Additional CSS class names on the label `<span>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the label `<span>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the label element.
    #[props(default)]
    pub id: &'static str,
}

/// A thin wrapper for text or icon content inside a [`Badge`].
///
/// Plain-text children passed to a [`Badge`] are automatically wrapped in
/// this component. Use it explicitly when you need to apply extra classes or
/// styles to the label slot.
///
/// # Examples
///
/// ```rust
/// use badges_rs::dioxus::{Badge, Anchor, Label};
/// use badges_rs::Color;
/// use dioxus::prelude::*;
///
/// fn MyLabel() -> Element {
///     rsx! {
///         Anchor {
///             span { "Box" }
///             Badge { color: Color::Accent,
///                 Label { class: "font-semibold", "New" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        span {
            id: props.id,
            class: "badge__label {props.class}",
            style: "{base_label_style()} {props.style}",
            {props.children}
        }
    }
}

/// Props for the [`Badge`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Content displayed inside the badge, text, number, or icon.
    ///
    /// When `None` the badge renders as a compact dot indicator.
    #[props(default = None)]
    pub children: Option<Element>,

    /// Color theme of the badge.
    #[props(default = Color::Default)]
    pub color: Color,

    /// Visual style variant, filled, outlined, or soft-tinted.
    #[props(default = Variant::Primary)]
    pub variant: Variant,

    /// Size of the badge.
    #[props(default = Size::Md)]
    pub size: Size,

    /// Corner placement relative to the parent [`Anchor`].
    #[props(default = Placement::TopRight)]
    pub placement: Placement,

    /// Target bounding box corner math representation.
    #[props(default = Shape::Rectangle)]
    pub shape: Shape,

    /// Additional CSS class names on the badge `<span>`.
    #[props(default)]
    pub class: &'static str,

    /// Inline CSS on the badge `<span>`.
    #[props(default)]
    pub style: &'static str,

    /// `id` attribute on the badge element.
    #[props(default)]
    pub id: &'static str,

    /// Accessible label announced by assistive technology.
    #[props(default = "Badge")]
    pub aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[props(default)]
    pub data_testid: &'static str,
}

/// A small indicator positioned relative to another element via [`Anchor`].
///
/// When `children` are `Some` the badge renders content inside a [`Label`].
/// When `children` is `None` the badge renders as a compact dot.
///
/// # Accessibility
///
/// - Renders with `role="status"` and `aria-atomic="true"` so screen readers
///   announce updates correctly.
///
/// # Examples
///
/// ```rust
/// use badges_rs::dioxus::{Badge, Anchor};
/// use badges_rs::{Color, Size, Variant, Placement};
/// use dioxus::prelude::*;
///
/// fn NotificationBadge() -> Element {
///     rsx! {
///         Anchor {
///             span { aria_label: "Inbox, 5 unread messages", "📬" }
///             Badge {
///                 color: Color::Danger,
///                 size: Size::Sm,
///                 variant: Variant::Primary,
///                 placement: Placement::TopRight,
///                 aria_label: "5 unread messages",
///                 "5"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let has_children = props.children.is_some();

    let color_style = match props.variant {
        Variant::Primary => props.color.to_primary_style(),
        Variant::Secondary => props.color.to_secondary_style(),
        Variant::Soft => props.color.to_soft_style(),
    };

    let size_style = if has_children {
        props.size.to_label_style()
    } else {
        props.size.to_dot_style()
    };

    let badge_class = format!(
        "badge {} {} {} {} {}",
        props.variant.to_class(),
        props.color.to_class(),
        props.size.to_class(),
        props.placement.to_class(),
        props.class,
    );

    let full_style = format!(
        "{} {} {} {} {}",
        base_badge_style(),
        color_style,
        size_style,
        props.placement.to_style(props.shape),
        props.style,
    );

    rsx! {
        span {
            id: props.id,
            class: "{badge_class}",
            style: "{full_style}",
            role: "status",
            aria_label: props.aria_label,
            aria_atomic: "true",
            "data-testid": props.data_testid,
            if let Some(children) = props.children {
                Label { {children} }
            }
        }
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
