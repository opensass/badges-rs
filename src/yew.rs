// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../YEW.md")]

use crate::common::{
    Color, Placement, Shape, Size, Variant, base_anchor_style, base_badge_style, base_label_style,
};
use yew::prelude::*;

/// Props for the [`Anchor`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct AnchorProps {
    /// The target element and the [`Badge`] itself.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the anchor wrapper `<span>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the anchor wrapper `<span>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the anchor wrapper element.
    #[prop_or_default]
    pub id: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
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
/// use badges_rs::yew::{Badge, Anchor};
/// use badges_rs::Color;
/// use yew::prelude::*;
///
/// #[function_component(MyBadge)]
/// pub fn my_badge() -> Html {
///     html! {
///         <Anchor>
///             <span>{"Icon"}</span>
///             <Badge color={Color::Danger}>{"5"}</Badge>
///         </Anchor>
///     }
/// }
/// ```
#[function_component(Anchor)]
pub fn badge_anchor(props: &AnchorProps) -> Html {
    html! {
        <span
            id={props.id}
            class={format!("badge-anchor {}", props.class)}
            style={format!("{} {}", base_anchor_style(), props.style)}
            data-testid={props.data_testid}
        >
            { for props.children.iter() }
        </span>
    }
}

/// Props for the [`Label`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct LabelProps {
    /// Text, number, or icon content for the label slot.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the label `<span>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the label `<span>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the label element.
    #[prop_or_default]
    pub id: &'static str,
}

/// A thin wrapper for text or icon content inside a [`Badge`].
///
/// Plain-text children passed to a [`Badge`] are automatically wrapped in
/// this component. Use it explicitly when you need to apply extra CSS classes
/// or styles to the label slot.
///
/// # Examples
///
/// ```rust
/// use badges_rs::yew::{Badge, Anchor, Label};
/// use badges_rs::Color;
/// use yew::prelude::*;
///
/// #[function_component(MyLabel)]
/// pub fn my_label() -> Html {
///     html! {
///         <Anchor>
///             <span>{"Box"}</span>
///             <Badge color={Color::Accent}>
///                 <Label class="font-semibold">{"New"}</Label>
///             </Badge>
///         </Anchor>
///     }
/// }
/// ```
#[function_component(Label)]
pub fn badge_label(props: &LabelProps) -> Html {
    html! {
        <span
            id={props.id}
            class={format!("badge__label {}", props.class)}
            style={format!("{} {}", base_label_style(), props.style)}
        >
            { for props.children.iter() }
        </span>
    }
}

/// Props for the [`Badge`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    /// Content displayed inside the badge, text, number, or icon.
    ///
    /// When omitted entirely the badge renders as a compact dot indicator.
    #[prop_or_default]
    pub children: Children,

    /// Color theme of the badge.
    #[prop_or_default]
    pub color: Color,

    /// Visual style variant, filled, outlined, or soft-tinted.
    #[prop_or_default]
    pub variant: Variant,

    /// Size of the badge.
    #[prop_or_default]
    pub size: Size,

    /// Corner placement relative to the parent [`Anchor`].
    #[prop_or_default]
    pub placement: Placement,

    /// Target bounding box corner math representation.
    #[prop_or_default]
    pub shape: Shape,

    /// Additional CSS class names on the badge `<span>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the badge `<span>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the badge element.
    #[prop_or_default]
    pub id: &'static str,

    /// Accessible label announced by assistive technology.
    #[prop_or("Badge")]
    pub aria_label: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A small indicator positioned relative to another element via [`Anchor`].
///
/// When `children` are supplied the badge renders content (text, number, icon)
/// inside a [`Label`] wrapper. When `children` is empty the badge renders
/// as a compact dot, useful for online/offline status indicators.
///
/// # Accessibility
///
/// - Renders with `role="status"` and a required `aria_label` so screen readers
///   can announce the badge content without the user navigating to it.
/// - `aria-atomic="true"` ensures the full label is re-read on updates.
///
/// # Examples
///
/// ```rust
/// use badges_rs::yew::{Badge, Anchor};
/// use badges_rs::{Color, Size, Variant, Placement};
/// use yew::prelude::*;
///
/// #[function_component(NotificationBadge)]
/// pub fn notification_badge() -> Html {
///     html! {
///         <Anchor>
///             <span aria-label="Inbox, 5 unread messages">{"📬"}</span>
///             <Badge
///                 color={Color::Danger}
///                 size={Size::Sm}
///                 variant={Variant::Primary}
///                 placement={Placement::TopRight}
///                 aria_label="5 unread messages"
///             >
///                 {"5"}
///             </Badge>
///         </Anchor>
///     }
/// }
/// ```
#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let has_children = props.children.iter().next().is_some();

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

    let full_style = format!(
        "{} {} {} {} {}",
        base_badge_style(),
        color_style,
        size_style,
        props.placement.to_style(props.shape),
        props.style,
    );

    let badge_class = format!(
        "badge {} {} {} {} {}",
        props.variant.to_class(),
        props.color.to_class(),
        props.size.to_class(),
        props.placement.to_class(),
        props.class,
    );

    html! {
        <span
            id={props.id}
            class={badge_class}
            style={full_style}
            role="status"
            aria-label={props.aria_label}
            aria-atomic="true"
            data-testid={props.data_testid}
        >
            if has_children {
                <Label>
                    { for props.children.iter() }
                </Label>
            }
        </span>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
