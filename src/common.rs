// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Rendered size of a [`Badge`] component.
///
/// Controls `min-width`, `height`, `padding`, and `font-size` when the badge
/// has children (label mode), or `width` and `height` when it is a dot.
///
/// # Default
///
/// [`Size::Md`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use badges_rs::Size;
///
/// let cls = Size::Sm.to_class();
/// assert_eq!(cls, "badge--sm");
///
/// let style = Size::Lg.to_label_style();
/// assert!(style.contains("24px"));
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Size {
    /// Small: 16 x 16 px container, 10 px font.
    Sm,

    /// Medium: 20 x 20 px container, 11 px font. This is the default.
    #[default]
    Md,

    /// Large: 24 x 24 px container, 12 px font.
    Lg,
}

impl Size {
    /// Returns the BEM modifier CSS class for this size.
    ///
    /// # Returns
    ///
    /// One of `"badge--sm"`, `"badge--md"`, or `"badge--lg"`.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Sm => "badge--sm",
            Self::Md => "badge--md",
            Self::Lg => "badge--lg",
        }
    }

    /// Returns inline CSS for label (with-content) mode.
    ///
    /// Sets `min-width`, `height`, `font-size`, and `padding`.
    pub fn to_label_style(self) -> &'static str {
        match self {
            Self::Sm => "min-width: 16px; height: 16px; font-size: 10px; padding: 0 4px;",
            Self::Md => "min-width: 20px; height: 20px; font-size: 11px; padding: 0 5px;",
            Self::Lg => "min-width: 24px; height: 24px; font-size: 12px; padding: 0 6px;",
        }
    }

    /// Returns inline CSS for dot (no-content) mode.
    ///
    /// Sets `width` and `height` only, no padding or min-width.
    pub fn to_dot_style(self) -> &'static str {
        match self {
            Self::Sm => "width: 8px; height: 8px;",
            Self::Md => "width: 10px; height: 10px;",
            Self::Lg => "width: 12px; height: 12px;",
        }
    }
}

/// Color theme applied to the [`Badge`] component.
///
/// Each variant maps to a distinct palette across all three visual variants
/// (`primary`, `secondary`, `soft`).
///
/// # Default
///
/// [`Color::Default`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use badges_rs::Color;
///
/// let style = Color::Danger.to_primary_style();
/// assert!(style.contains("#dc2626"));
///
/// let cls = Color::Accent.to_class();
/// assert_eq!(cls, "badge--accent");
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Color {
    /// Neutral gray.
    #[default]
    Default,

    /// Accent purple: `#7c3aed`.
    Accent,

    /// Success green: `#16a34a`.
    Success,

    /// Warning amber: `#d97706`.
    Warning,

    /// Danger red: `#dc2626`.
    Danger,
}

impl Color {
    /// Returns the BEM modifier CSS class for this color.
    ///
    /// # Returns
    ///
    /// One of `"badge--default"`, `"badge--accent"`, `"badge--success"`,
    /// `"badge--warning"`, or `"badge--danger"`.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Default => "badge--default",
            Self::Accent => "badge--accent",
            Self::Success => "badge--success",
            Self::Warning => "badge--warning",
            Self::Danger => "badge--danger",
        }
    }

    /// Returns the filled (primary variant) inline CSS for this color.
    ///
    /// Uses a saturated background with white text.
    pub fn to_primary_style(self) -> &'static str {
        match self {
            Self::Default => "background-color: #4b5563; color: #ffffff;",
            Self::Accent => "background-color: #7c3aed; color: #ffffff;",
            Self::Success => "background-color: #16a34a; color: #ffffff;",
            Self::Warning => "background-color: #d97706; color: #ffffff;",
            Self::Danger => "background-color: #dc2626; color: #ffffff;",
        }
    }

    /// Returns the outlined (secondary variant) inline CSS for this color.
    ///
    /// Uses a white background with a coloured border and matching text.
    pub fn to_secondary_style(self) -> &'static str {
        match self {
            Self::Default => {
                "background-color: #ffffff; color: #4b5563; border: 1px solid #4b5563;"
            }
            Self::Accent => "background-color: #ffffff; color: #7c3aed; border: 1px solid #7c3aed;",
            Self::Success => {
                "background-color: #ffffff; color: #16a34a; border: 1px solid #16a34a;"
            }
            Self::Warning => {
                "background-color: #ffffff; color: #d97706; border: 1px solid #d97706;"
            }
            Self::Danger => "background-color: #ffffff; color: #dc2626; border: 1px solid #dc2626;",
        }
    }

    /// Returns the tinted (soft variant) inline CSS for this color.
    ///
    /// Uses a lightly tinted background with the full-strength text color.
    pub fn to_soft_style(self) -> &'static str {
        match self {
            Self::Default => "background-color: #f3f4f6; color: #4b5563;",
            Self::Accent => "background-color: #ede9fe; color: #7c3aed;",
            Self::Success => "background-color: #dcfce7; color: #16a34a;",
            Self::Warning => "background-color: #fef3c7; color: #d97706;",
            Self::Danger => "background-color: #fee2e2; color: #dc2626;",
        }
    }
}

/// Visual style variant of the [`Badge`] component.
///
/// Controls whether the badge appears filled, outlined, or softly tinted.
///
/// # Default
///
/// [`Variant::Primary`] is the default variant.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Variant {
    /// Filled background, the most visually prominent style.
    #[default]
    Primary,

    /// Outlined, white background with a coloured border.
    Secondary,

    /// Lightly tinted background with full-strength text.
    Soft,
}

impl Variant {
    /// Returns the BEM modifier CSS class for this variant.
    ///
    /// # Returns
    ///
    /// One of `"badge--primary"`, `"badge--secondary"`, or `"badge--soft"`.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Primary => "badge--primary",
            Self::Secondary => "badge--secondary",
            Self::Soft => "badge--soft",
        }
    }
}

/// Absolute position of the [`Badge`] relative to its [`Anchor`].
///
/// Each variant maps to a combination of `top`/`bottom`/`left`/`right` and a
/// CSS `transform` for precise corner placement.
///
/// # Default
///
/// [`Placement::TopRight`] is the default variant.
///
/// # Examples
///
/// ```rust
/// use badges_rs::{Placement, Shape};
///
/// let cls = Placement::BottomLeft.to_class();
/// assert_eq!(cls, "badge--bottom-left");
///
/// let style = Placement::TopRight.to_style(Shape::default());
/// assert!(style.contains("translate(50%, -50%)"));
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Placement {
    /// Top-right corner, the most common placement. This is the default.
    #[default]
    TopRight,

    /// Top-left corner.
    TopLeft,

    /// Bottom-right corner.
    BottomRight,

    /// Bottom-left corner.
    BottomLeft,
}

impl Placement {
    /// Returns the BEM modifier CSS class for this placement.
    ///
    /// # Returns
    ///
    /// One of `"badge--top-right"`, `"badge--top-left"`,
    /// `"badge--bottom-right"`, or `"badge--bottom-left"`.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::TopRight => "badge--top-right",
            Self::TopLeft => "badge--top-left",
            Self::BottomRight => "badge--bottom-right",
            Self::BottomLeft => "badge--bottom-left",
        }
    }

    /// Returns the inline CSS positioning for this placement.
    ///
    /// Uses `top`/`bottom`/`left`/`right` anchors combined with `transform`
    /// to centre the badge on the corner of its anchor element. Fits securely
    /// on either a rectangle box or precisely curves onto a 45° arc for circles.
    pub fn to_style(self, shape: Shape) -> &'static str {
        match (self, shape) {
            (Self::TopRight, Shape::Rectangle) => {
                "top: 0; right: 0; transform: translate(50%, -50%);"
            }
            (Self::TopLeft, Shape::Rectangle) => {
                "top: 0; left: 0; transform: translate(-50%, -50%);"
            }
            (Self::BottomRight, Shape::Rectangle) => {
                "bottom: 0; right: 0; transform: translate(50%, 50%);"
            }
            (Self::BottomLeft, Shape::Rectangle) => {
                "bottom: 0; left: 0; transform: translate(-50%, 50%);"
            }

            (Self::TopRight, Shape::Circle) => {
                "top: 14.64%; right: 14.64%; transform: translate(50%, -50%);"
            }
            (Self::TopLeft, Shape::Circle) => {
                "top: 14.64%; left: 14.64%; transform: translate(-50%, -50%);"
            }
            (Self::BottomRight, Shape::Circle) => {
                "bottom: 14.64%; right: 14.64%; transform: translate(50%, 50%);"
            }
            (Self::BottomLeft, Shape::Circle) => {
                "bottom: 14.64%; left: 14.64%; transform: translate(-50%, 50%);"
            }
        }
    }
}

/// Boundary shape of the element this [`Badge`] anchors to.
///
/// Modifies the absolute offset to sit either precisely on a perfectly
/// round corner (14.64% bounding box offset) or a standard rectangle corner.
///
/// # Default
///
/// [`Shape::Rectangle`] is the default.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Shape {
    /// Bounding coordinates calculate to 0%.
    #[default]
    Rectangle,

    /// Bounding coordinates calculate to ~14.64% (`1 - sin(45°)`).
    Circle,
}

/// Returns the base inline CSS applied to every [`Badge`] span element.
///
/// Sets `position: absolute`, flex centering, full border-radius, font
/// weight, and pointer-events suppression so the badge does not intercept
/// mouse events on its anchor.
pub fn base_badge_style() -> &'static str {
    "position: absolute; display: inline-flex; align-items: center; justify-content: center; border-radius: 9999px; font-weight: 600; white-space: nowrap; pointer-events: none; line-height: 1; box-sizing: border-box;"
}

/// Returns the base inline CSS applied to the [`Anchor`] span element.
///
/// Sets `position: relative` so that the absolutely-positioned [`Badge`]
/// resolves against this element, and `display: inline-flex` so the anchor
/// hugs its child element.
pub fn base_anchor_style() -> &'static str {
    "position: relative; display: inline-flex; flex-shrink: 0;"
}

/// Returns the base inline CSS applied to the [`Label`] span element.
///
/// Inherits font settings from the parent [`Badge`] for consistent
/// typography without re-declaring size values.
pub fn base_label_style() -> &'static str {
    "font-size: inherit; font-weight: inherit; line-height: 1;"
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
