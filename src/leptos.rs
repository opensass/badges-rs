// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../LEPTOS.md")]

use crate::common::{
    Color, Placement, Shape, Size, Variant, base_anchor_style, base_badge_style, base_label_style,
};
use leptos::prelude::*;

/// A positioning wrapper that makes a [`Badge`] absolute-positioned relative
/// to its sibling element.
///
/// Place the content element and a [`Badge`] as direct children. The badge
/// will anchor itself to a corner of the content element.
///
/// # Accessibility
///
/// - Renders as `<span>` with `position: relative` without disrupting
///   document flow or semantic structure.
///
/// # Examples
///
/// ```rust
/// use badges_rs::leptos::{Badge, Anchor};
/// use badges_rs::Color;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyBadge() -> impl IntoView {
///     view! {
///         <Anchor>
///             <span>"Icon"</span>
///             <Badge color=Color::Danger>"5"</Badge>
///         </Anchor>
///     }
/// }
/// ```
#[component]
pub fn Anchor(
    /// The target element and the [`Badge`] itself.
    children: Children,

    /// Additional CSS class names on the anchor wrapper `<span>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the anchor wrapper `<span>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the anchor wrapper element.
    #[prop(default = "")]
    id: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    view! {
        <span
            id=id
            class=format!("badge-anchor {}", class)
            style=format!("{} {}", base_anchor_style(), style)
            data-testid=data_testid
        >
            {children()}
        </span>
    }
}

/// A thin wrapper for text or icon content inside a [`Badge`].
///
/// Plain text children passed to a [`Badge`] are automatically wrapped in
/// this component. Use it explicitly when you need additional styling on the
/// label slot.
///
/// # Examples
///
/// ```rust
/// use badges_rs::leptos::{Badge, Anchor, Label};
/// use badges_rs::Color;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyLabel() -> impl IntoView {
///     view! {
///         <Anchor>
///             <span>"Box"</span>
///             <Badge color=Color::Accent>
///                 <Label class="font-semibold">"New"</Label>
///             </Badge>
///         </Anchor>
///     }
/// }
/// ```
#[component]
pub fn Label(
    /// Text, number, or icon content for the label slot.
    children: Children,

    /// Additional CSS class names on the label `<span>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the label `<span>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the label element.
    #[prop(default = "")]
    id: &'static str,
) -> impl IntoView {
    view! {
        <span
            id=id
            class=format!("badge__label {}", class)
            style=format!("{} {}", base_label_style(), style)
        >
            {children()}
        </span>
    }
}

/// A small indicator positioned relative to another element via [`Anchor`].
///
/// When `children` are `Some` the badge renders content inside a [`Label`].
/// When `children` is `None` the badge renders as a compact dot.
///
/// # Accessibility
///
/// - Renders with `role="status"` and `aria-atomic="true"` so screen readers
///   announce the full label on updates.
/// - The `aria_label` prop is required for meaningful announcements when
///   the badge contains only a number or icon.
///
/// # Examples
///
/// ```rust
/// use badges_rs::leptos::{Badge, Anchor};
/// use badges_rs::{Color, Size, Variant, Placement};
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn NotificationBadge() -> impl IntoView {
///     view! {
///         <Anchor>
///             <span aria-label="Inbox, 5 unread messages">"📬"</span>
///             <Badge
///                 color=Color::Danger
///                 size=Size::Sm
///                 variant=Variant::Primary
///                 placement=Placement::TopRight
///                 aria_label="5 unread messages"
///             >
///                 "5"
///             </Badge>
///         </Anchor>
///     }
/// }
/// ```
#[component]
pub fn Badge(
    /// Content displayed inside the badge, text, number, or icon.
    ///
    /// When omitted the badge renders as a compact dot indicator.
    #[prop(optional)]
    children: Option<Children>,

    /// Color theme of the badge.
    #[prop(default = Color::Default)]
    color: Color,

    /// Visual style variant, filled, outlined, or soft-tinted.
    #[prop(default = Variant::Primary)]
    variant: Variant,

    /// Size of the badge.
    #[prop(default = Size::Md)]
    size: Size,

    /// Corner placement relative to the parent [`Anchor`].
    #[prop(default = Placement::TopRight)]
    placement: Placement,

    /// Target bounding box corner math representation.
    #[prop(default = Shape::Rectangle)]
    shape: Shape,

    /// Additional CSS class names on the badge `<span>`.
    #[prop(default = "")]
    class: &'static str,

    /// Inline CSS on the badge `<span>`.
    #[prop(default = "")]
    style: &'static str,

    /// `id` attribute on the badge element.
    #[prop(default = "")]
    id: &'static str,

    /// Accessible label announced by assistive technology.
    #[prop(default = "Badge")]
    aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let has_children = children.is_some();

    let color_style = match variant {
        Variant::Primary => color.to_primary_style(),
        Variant::Secondary => color.to_secondary_style(),
        Variant::Soft => color.to_soft_style(),
    };

    let size_style = if has_children {
        size.to_label_style()
    } else {
        size.to_dot_style()
    };

    let full_style = format!(
        "{} {} {} {} {}",
        base_badge_style(),
        color_style,
        size_style,
        placement.to_style(shape),
        style,
    );

    let badge_class = format!(
        "badge {} {} {} {} {}",
        variant.to_class(),
        color.to_class(),
        size.to_class(),
        placement.to_class(),
        class,
    );

    view! {
        <span
            id=id
            class=badge_class
            style=full_style
            role="status"
            aria-label=aria_label
            aria-atomic="true"
            data-testid=data_testid
        >
            {children.map(|ch| view! { <Label>{ch()}</Label> }.into_any())}
        </span>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
