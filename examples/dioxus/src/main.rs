// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use badges_rs::dioxus::{Anchor, Badge};
use badges_rs::{Color, Placement, Shape, Size, Variant};
use dioxus::prelude::*;

static AVATAR: &str = "display:inline-flex;width:40px;height:40px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static AVATAR_SM: &str = "display:inline-flex;width:32px;height:32px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:12px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static AVATAR_LG: &str = "display:inline-flex;width:48px;height:48px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static ICON_BTN: &str = "display:inline-flex;width:44px;height:44px;border-radius:0.5rem;background:#1e293b;border:1px solid #334155;align-items:center;justify-content:center;cursor:default;flex-shrink:0;";
static SQUARE: &str =
    "display:inline-block;width:32px;height:32px;background:#3b82f6;flex-shrink:0;";
static CIRCLE: &str = "display:inline-block;width:32px;height:32px;border-radius:50%;background:#3b82f6;flex-shrink:0;";

#[component]
fn Example1() -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;",
            Anchor {
                span { style: "{AVATAR}", aria_label: "FP user, 5 notifications", "FP" }
                Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "5 notifications", "5" }
            }
            Anchor {
                span { style: "{AVATAR}", aria_label: "AB user, new", "AB" }
                Badge { shape: Shape::Circle, color: Color::Accent, size: Size::Sm, aria_label: "New", "New" }
            }
            Anchor {
                span { style: "{AVATAR}", aria_label: "CD user, online", "CD" }
                Badge { shape: Shape::Circle, color: Color::Success, placement: Placement::BottomRight, size: Size::Sm, aria_label: "Online" }
            }
        }
    }
}

#[component]
fn Example2() -> Element {
    let mut visible = use_signal(|| true);
    rsx! {
        div { style: "display:flex;flex-direction:column;align-items:center;gap:1.5rem;",
            Anchor {
                span { style: "{AVATAR}",
                    aria_label: if visible() { "User, 4 unread messages" } else { "User inbox" },
                    "FP"
                }
                if visible() {
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "4 unread messages", "4" }
                }
            }
            button {
                onclick: move |_| visible.set(!visible()),
                aria_pressed: if visible() { "true" } else { "false" },
                style: "padding:0.4rem 1rem;border:1px solid #334155;border-radius:9999px;background:#1e293b;color:#e2e8f0;cursor:pointer;font-size:0.875rem;",
                if visible() { "Hide Badge" } else { "Show Badge" }
            }
        }
    }
}

#[component]
fn Example3() -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;",
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{ICON_BTN}", aria_label: "show 99 unread messages", "✉" }
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "99", "99" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "99" }
            }
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{ICON_BTN}", aria_label: "show more than 99 unread messages", "✉" }
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "99+", "99+" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "100" }
            }
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{ICON_BTN}", aria_label: "show more than 999 unread messages", "✉" }
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "999+", "999+" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "1000" }
            }
        }
    }
}

#[component]
fn Example4() -> Element {
    const VARIANTS: &[(Variant, &str)] = &[
        (Variant::Primary, "primary"),
        (Variant::Secondary, "secondary"),
        (Variant::Soft, "soft"),
    ];
    const COLORS: &[(Color, &str)] = &[
        (Color::Accent, "Accent"),
        (Color::Default, "Default"),
        (Color::Success, "Success"),
        (Color::Warning, "Warning"),
        (Color::Danger, "Danger"),
    ];
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:1.5rem;",
            for (variant, vlabel) in VARIANTS {
                div {
                    p { style: "font-size:0.75rem;font-weight:600;color:#6b7280;margin-bottom:0.5rem;", "{vlabel}" }
                    div { style: "display:flex;align-items:center;gap:1.25rem;flex-wrap:wrap;",
                        for (color, _) in COLORS {
                            Anchor {
                                span { style: "{AVATAR}", "FP" }
                                Badge { shape: Shape::Circle, color: *color, size: Size::Sm, variant: *variant, aria_label: "5", "5" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Example5() -> Element {
    rsx! {
        div { style: "display:flex;align-items:flex-end;gap:2rem;flex-wrap:wrap;justify-content:center;",
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{AVATAR_SM}", aria_label: "Small user", "FP" }
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "5", "5" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "Sm" }
            }
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{AVATAR}", aria_label: "Medium user", "FP" }
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Md, aria_label: "5", "5" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "Md" }
            }
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{AVATAR_LG}", aria_label: "Large user", "FP" }
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Lg, aria_label: "5", "5" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "Lg" }
            }
        }
    }
}

#[component]
fn Example6() -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;gap:2rem;flex-wrap:wrap;justify-content:center;",
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{SQUARE}", aria_label: "Rectangular with count" }
                    Badge { shape: Shape::Rectangle, color: Color::Danger, size: Size::Sm, aria_label: "1", "1" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "rect + count" }
            }
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{SQUARE}", aria_label: "Rectangular dot" }
                    Badge { shape: Shape::Rectangle, color: Color::Danger, size: Size::Sm, aria_label: "Active" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "rect + dot" }
            }
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{CIRCLE}", aria_label: "Circular with count" }
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "1", "1" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "circle + count" }
            }
            div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                Anchor {
                    span { style: "{CIRCLE}", aria_label: "Circular dot" }
                    Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "Active" }
                }
                span { style: "font-size:0.75rem;color:#6b7280;", "circle + dot" }
            }
        }
    }
}

#[component]
fn Example7() -> Element {
    const COLORS: &[(Color, &str)] = &[
        (Color::Accent, "Accent"),
        (Color::Default, "Default"),
        (Color::Success, "Success"),
        (Color::Warning, "Warning"),
        (Color::Danger, "Danger"),
    ];
    rsx! {
        div { style: "display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;",
            for (color, label) in COLORS {
                Anchor {
                    span { style: "{AVATAR}", aria_label: "User status", "FP" }
                    Badge { shape: Shape::Circle, color: *color, size: Size::Sm, aria_label: *label }
                }
            }
        }
    }
}

#[component]
fn Example8() -> Element {
    const STATUSES: &[(&str, Color, &str)] = &[
        ("Online", Color::Success, "Online"),
        ("Away", Color::Warning, "Away"),
        ("Busy", Color::Danger, "Busy"),
        ("Offline", Color::Default, "Offline"),
    ];
    rsx! {
        div { style: "display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;",
            for (label, color, aria) in STATUSES {
                div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                    Anchor {
                        span { style: "{AVATAR}", aria_label: "User, {label}", "FP" }
                        Badge { shape: Shape::Circle, color: *color, placement: Placement::BottomRight, size: Size::Sm, aria_label: *aria }
                    }
                    span { style: "font-size:0.75rem;color:#6b7280;", "{label}" }
                }
            }
        }
    }
}

#[component]
fn Example9() -> Element {
    const PLACEMENTS: &[(Placement, &str)] = &[
        (Placement::TopRight, "top-right"),
        (Placement::TopLeft, "top-left"),
        (Placement::BottomRight, "bottom-right"),
        (Placement::BottomLeft, "bottom-left"),
    ];
    rsx! {
        div { style: "display:flex;align-items:center;gap:2rem;flex-wrap:wrap;justify-content:center;",
            for (placement, label) in PLACEMENTS {
                div { style: "display:flex;flex-direction:column;align-items:center;gap:0.5rem;",
                    Anchor {
                        span { style: "{AVATAR}", "FP" }
                        Badge { shape: Shape::Circle, color: Color::Accent, placement: *placement, size: Size::Sm, aria_label: *label }
                    }
                    span { style: "font-size:0.75rem;color:#6b7280;", "{label}" }
                }
            }
        }
    }
}

#[component]
fn Example10() -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;",
            Anchor {
                span { style: "{AVATAR}", aria_label: "User, 5 unread", "FP" }
                Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "5 unread messages", "5" }
            }
            Anchor {
                span { style: "{AVATAR}", aria_label: "User, new", "FP" }
                Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "New", "New" }
            }
            Anchor {
                span { style: "{AVATAR}", aria_label: "User, 99+ notifications", "FP" }
                Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "99+ notifications", "99+" }
            }
        }
    }
}

#[component]
fn Example11() -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;",
            Anchor {
                span { style: "{ICON_BTN}", aria_label: "show 4 unread messages", "✉" }
                Badge { shape: Shape::Circle, color: Color::Danger, size: Size::Sm, aria_label: "4 unread", "4" }
            }
            Anchor {
                span { style: "{ICON_BTN}", aria_label: "show 8 shared files", "📁" }
                Badge { shape: Shape::Circle, color: Color::Default, size: Size::Sm, aria_label: "8 files", "8" }
            }
            Anchor {
                span { style: "{ICON_BTN}", aria_label: "show 2 confirmed events", "📅" }
                Badge { shape: Shape::Circle, color: Color::Success, size: Size::Sm, aria_label: "2 events", "2" }
            }
            Anchor {
                span { style: "{ICON_BTN}", aria_label: "show 1 critical alert", "⚠" }
                Badge { shape: Shape::Circle, color: Color::Warning, size: Size::Sm, aria_label: "1 alert", "1" }
            }
        }
    }
}

#[component]
fn Example12() -> Element {
    rsx! {
        nav { aria_label: "Mail folders",
            style: "background:#1e293b;border-radius:0.5rem;border:1px solid #334155;width:100%;max-width:280px;",
            ul { style: "list-style:none;margin:0;padding:0;",
                li {
                    style: "display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;background:#334155;border-left:3px solid #7c3aed;border-bottom:1px solid #334155;",
                    aria_label: "Inbox, 4 unread messages", aria_current: "page",
                    span { style: "color:#e2e8f0;font-size:0.875rem;", "Inbox" }
                    Badge { shape: Shape::Circle, color: Color::Accent, variant: Variant::Soft, size: Size::Sm,
                        style: "position:static;transform:none;", aria_label: "4 unread", "4" }
                }
                li {
                    style: "display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;border-bottom:1px solid #334155;",
                    aria_label: "Sent",
                    span { style: "color:#e2e8f0;font-size:0.875rem;", "Sent" }
                }
                li {
                    style: "display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;border-bottom:1px solid #334155;",
                    aria_label: "Drafts, 12 unread messages",
                    span { style: "color:#e2e8f0;font-size:0.875rem;", "Drafts" }
                    Badge { shape: Shape::Circle, color: Color::Default, variant: Variant::Soft, size: Size::Sm,
                        style: "position:static;transform:none;", aria_label: "12 unread", "12" }
                }
                li {
                    style: "display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;",
                    aria_label: "Spam, more than 99 unread messages",
                    span { style: "color:#e2e8f0;font-size:0.875rem;", "Spam" }
                    Badge { shape: Shape::Circle, color: Color::Default, variant: Variant::Soft, size: Size::Sm,
                        style: "position:static;transform:none;", aria_label: "99+ unread", "99+" }
                }
            }
        }
    }
}

#[component]
fn Example13() -> Element {
    const VARIANTS: &[(Variant, &str)] = &[
        (Variant::Primary, "primary"),
        (Variant::Secondary, "secondary"),
        (Variant::Soft, "soft"),
    ];
    const COLORS: &[(Color, &str)] = &[
        (Color::Accent, "Accent"),
        (Color::Default, "Default"),
        (Color::Success, "Success"),
        (Color::Warning, "Warning"),
        (Color::Danger, "Danger"),
    ];
    rsx! {
        div { class: "w-full overflow-x-auto bg-gray-800 rounded-xl p-4",
            role: "region", aria_label: "Badge variant matrix",
            table { class: "w-full text-left border-collapse",
                caption { class: "sr-only", "Badge style matrix, variants x colors" }
                thead {
                    tr {
                        th { scope: "col", class: "p-3 text-gray-400 font-medium", "" }
                        for (_, name) in COLORS {
                            th { scope: "col", class: "p-3 text-gray-400 font-medium text-center", "{name}" }
                        }
                    }
                }
                tbody {
                    for (variant, vlabel) in VARIANTS {
                        tr {
                            th { scope: "row", class: "p-3 text-gray-400 capitalize", "{vlabel}" }
                            for (color, _) in COLORS {
                                td { class: "p-3 text-center border-t border-white/10",
                                    Anchor {
                                        span { style: "{AVATAR}", "FP" }
                                        Badge { shape: Shape::Circle, color: *color, size: Size::Sm, variant: *variant, aria_label: "5", "5" }
                                    }
                                }
                            }
                        }
                    }
                    tr {
                        th { scope: "row", class: "p-3 text-gray-400", "dot" }
                        for (color, clabel) in COLORS {
                            td { class: "p-3 text-center border-t border-white/10",
                                Anchor {
                                    span { style: "{AVATAR}", "FP" }
                                    Badge { shape: Shape::Circle, color: *color, size: Size::Sm, placement: Placement::BottomRight, aria_label: *clabel }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Example14() -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;gap:2rem;flex-wrap:wrap;justify-content:center;",
            Anchor {
                span {
                    style: "display:inline-flex;width:40px;height:40px;border-radius:50%;background:#1e293b;border:2px solid #7c3aed;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;",
                    aria_label: "Kate Wilson, 5 notifications",
                    "KW"
                }
                Badge { shape: Shape::Circle, class: "font-semibold", color: Color::Accent, size: Size::Sm,
                    variant: Variant::Soft, aria_label: "5 notifications", "5" }
            }
            Anchor {
                span {
                    style: "display:inline-flex;width:40px;height:40px;border-radius:8px;background:#1e293b;border:1px solid #334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;",
                    aria_label: "TS user, 2 files",
                    "TS"
                }
                Badge { shape: Shape::Circle, color: Color::Success, size: Size::Md, variant: Variant::Secondary,
                    aria_label: "2 files", "2" }
            }
        }
    }
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div {
            class: "min-h-screen flex flex-col items-center justify-center",
            style: "color:#5e5c7f;background-color:#303030;font-family:'Rubik',sans-serif;overflow-x:hidden;",

            h1 { class: "text-3xl font-bold mb-8 text-white", "Badges RS Dioxus Examples" }

            section { aria_labelledby: "basic-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "basic-heading", class: "text-xl font-semibold text-white mb-6", "Basic" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Basic Badge" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Placement, Size}};
use dioxus::prelude::*;

#[component]
fn BasicBadge() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:1.5rem;",
            Anchor {{
                span {{ aria_label: "FP, 5 notifications", "FP" }}
                Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Sm,
                    aria_label: "5 notifications", "5" }}
            }}
            Anchor {{
                span {{ aria_label: "AB, new", "AB" }}
                Badge {{ shape: Shape::Circle, color: Color::Accent, size: Size::Sm,
                    aria_label: "New", "New" }}
            }}
            Anchor {{
                span {{ aria_label: "CD, online", "CD" }}
                Badge {{ shape: Shape::Circle, color: Color::Success,
                    placement: Placement::BottomRight,
                    size: Size::Sm, aria_label: "Online" }}
            }}
        }}
    }}
}}"#
                        }
                        Example1 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Visibility Toggle" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size}};
use dioxus::prelude::*;

#[component]
fn VisibilityToggle() -> Element {{
    let mut visible = use_signal(|| true);
    rsx! {{
        div {{ style: "display:flex;flex-direction:column;
            align-items:center;gap:1.5rem;",
            Anchor {{
                span {{
                    aria_label: if visible() {{
                        "User, 4 unread messages"
                    }} else {{ "User inbox" }},
                    "FP"
                }}
                if visible() {{
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Sm,
                        aria_label: "4 unread messages", "4" }}
                }}
            }}
            button {{
                onclick: move |_| visible.set(!visible()),
                aria_pressed: if visible() {{
                    "true" }} else {{ "false" }},
                if visible() {{
                    "Hide Badge"
                }} else {{ "Show Badge" }}
            }}
        }}
    }}
}}"#
                        }
                        Example2 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Maximum Value" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size}};
use dioxus::prelude::*;

// Display 99 exactly, or truncate with 99+ / 999+.
#[component]
fn MaxValue() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:1.5rem;",
            div {{
                Anchor {{
                    span {{ aria_label: "show 99 unread", "✉" }}
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Sm,
                        aria_label: "99", "99" }}
                }}
                span {{ "99" }}
            }}
            div {{
                Anchor {{
                    span {{ aria_label: "show more than 99", "✉" }}
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Sm,
                        aria_label: "99+", "99+" }}
                }}
                span {{ "100" }}
            }}
            div {{
                Anchor {{
                    span {{ aria_label: "show more than 999", "✉" }}
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Sm,
                        aria_label: "999+", "999+" }}
                }}
                span {{ "1000" }}
            }}
        }}
    }}
}}"#
                        }
                        Example3 {}
                    }
                }
            }

            section { aria_labelledby: "variants-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "variants-heading", class: "text-xl font-semibold text-white mb-6", "Variants" }
                div { class: "grid grid-cols-1 gap-8",
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Variant x Color Matrix" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size, Variant}};
use dioxus::prelude::*;

const VARIANTS: &[(Variant, &str)] = &[
    (Variant::Primary,   "primary"),
    (Variant::Secondary, "secondary"),
    (Variant::Soft,      "soft"),
];
const COLORS: &[(Color, &str)] = &[
    (Color::Accent, "Accent"), (Color::Default, "Default"),
    (Color::Success, "Success"), (Color::Warning, "Warning"),
    (Color::Danger, "Danger"),
];

#[component]
fn VariantMatrix() -> Element {{
    rsx! {{
        div {{ style: "display:flex;flex-direction:column;gap:1.5rem;",
            for (variant, vlabel) in VARIANTS {{
                div {{
                    p {{ "{{vlabel}}" }}
                    div {{ style: "display:flex;align-items:center;gap:1.25rem;",
                        for (color, _) in COLORS {{
                            Anchor {{
                                span {{ "FP" }}
                                Badge {{ shape: Shape::Circle, color: *color, size: Size::Sm,
                                    variant: *variant, aria_label: "5", "5" }}
                            }}
                        }}
                    }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example4 {}
                    }
                }
            }

            section { aria_labelledby: "sizes-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "sizes-heading", class: "text-xl font-semibold text-white mb-6", "Sizes" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-8",
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Sizes" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size}};
use dioxus::prelude::*;

// Size::Sm → 16x16 px, 10 px font
// Size::Md → 20x20 px, 11 px font (default)
// Size::Lg → 24x24 px, 12 px font
#[component]
fn Sizes() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:flex-end;gap:2rem;",
            div {{
                Anchor {{
                    span {{ aria_label: "Small", "FP" }}
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Sm,
                        aria_label: "5", "5" }}
                }}
                span {{ "Sm" }}
            }}
            div {{
                Anchor {{
                    span {{ aria_label: "Medium", "FP" }}
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Md,
                        aria_label: "5", "5" }}
                }}
                span {{ "Md" }}
            }}
            div {{
                Anchor {{
                    span {{ aria_label: "Large", "FP" }}
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Lg,
                        aria_label: "5", "5" }}
                }}
                span {{ "Lg" }}
            }}
        }}
    }}
}}"#
                        }
                        Example5 {}
                    }
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Overlap Shapes" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size}};
use dioxus::prelude::*;

// Badge anchors to any shape: square, circle, icon button.
#[component]
fn Overlap() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:2rem;",
            div {{
                Anchor {{
                    span {{ style: "width:32px;height:32px;
                        background:#3b82f6;display:inline-block;",
                        aria_label: "Rectangle" }}
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Sm,
                        aria_label: "1", "1" }}
                }}
                span {{ "rect" }}
            }}
            div {{
                Anchor {{
                    span {{ style: "border-radius:50%;width:32px;
                        height:32px;background:#3b82f6;
                        display:inline-block;",
                        aria_label: "Circle" }}
                    Badge {{ shape: Shape::Circle, color: Color::Danger, size: Size::Sm,
                        aria_label: "1", "1" }}
                }}
                span {{ "circle" }}
            }}
        }}
    }}
}}"#
                        }
                        Example6 {}
                    }
                }
            }

            section { aria_labelledby: "colors-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "colors-heading", class: "text-xl font-semibold text-white mb-6", "Colors" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-8",
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Color - Dot Mode" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size}};
use dioxus::prelude::*;

const COLORS: &[(Color, &str)] = &[
    (Color::Accent, "Accent"),
    (Color::Default, "Default"),
    (Color::Success, "Success"),
    (Color::Warning, "Warning"),
    (Color::Danger, "Danger"),
];

// Omit children for a compact dot indicator.
#[component]
fn ColorDots() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:1.5rem;",
            for (color, label) in COLORS {{
                Anchor {{
                    span {{ aria_label: "User status", "FP" }}
                    Badge {{ shape: Shape::Circle, color: *color, size: Size::Sm,
                        aria_label: *label }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example7 {}
                    }
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Status Indicators" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Placement, Size}};
use dioxus::prelude::*;

const STATUSES: &[(&str, Color, &str)] = &[
    ("Online",  Color::Success, "Online"),
    ("Away",    Color::Warning, "Away"),
    ("Busy",    Color::Danger,  "Busy"),
    ("Offline", Color::Default, "Offline"),
];

// Bottom-right placement for presence / status dots.
#[component]
fn StatusDots() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:1.5rem;",
            for (label, color, aria) in STATUSES {{
                div {{
                    Anchor {{
                        span {{ aria_label: "User, {{label}}", "FP" }}
                        Badge {{ shape: Shape::Circle, color: *color,
                            placement: Placement::BottomRight,
                            size: Size::Sm, aria_label: *aria }}
                    }}
                    span {{ "{{label}}" }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example8 {}
                    }
                }
            }

            section { aria_labelledby: "placement-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "placement-heading", class: "text-xl font-semibold text-white mb-6", "Placements" }
                div { class: "grid grid-cols-1 gap-8",
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "All Four Corners" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Placement, Size}};
use dioxus::prelude::*;

const PLACEMENTS: &[(Placement, &str)] = &[
    (Placement::TopRight,    "top-right"),
    (Placement::TopLeft,     "top-left"),
    (Placement::BottomRight, "bottom-right"),
    (Placement::BottomLeft,  "bottom-left"),
];

#[component]
fn Placements() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:2rem;",
            for (placement, label) in PLACEMENTS {{
                div {{
                    Anchor {{
                        span {{ "FP" }}
                        Badge {{ shape: Shape::Circle, color: Color::Accent,
                            placement: *placement,
                            size: Size::Sm,
                            aria_label: *label }}
                    }}
                    span {{ "{{label}}" }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example9 {}
                    }
                }
            }

            section { aria_labelledby: "content-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "content-heading", class: "text-xl font-semibold text-white mb-6", "With Content" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-8",
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Text, Numbers, Icons" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size}};
use dioxus::prelude::*;

// Badge accepts any children: text, numbers,
// "99+", or SVG icons. Omit children for a dot.
#[component]
fn WithContent() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:1.5rem;",
            Anchor {{
                span {{ aria_label: "User, 5 unread", "FP" }}
                Badge {{ color: Color::Danger, size: Size::Sm,
                    aria_label: "5 unread messages", "5" }}
            }}
            Anchor {{
                span {{ aria_label: "User, new", "FP" }}
                Badge {{ color: Color::Danger, size: Size::Sm,
                    aria_label: "New", "New" }}
            }}
            Anchor {{
                span {{ aria_label: "User, 99+", "FP" }}
                Badge {{ color: Color::Danger, size: Size::Sm,
                    aria_label: "99+ notifications", "99+" }}
            }}
        }}
    }}
}}"#
                        }
                        Example10 {}
                    }
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "On Icon Buttons" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size}};
use dioxus::prelude::*;

// Badge is not limited to avatars.
// It anchors to any interactive element.
#[component]
fn IconBadges() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:1.5rem;",
            Anchor {{
                span {{ aria_label: "show 4 unread messages", "✉" }}
                Badge {{ color: Color::Danger, size: Size::Sm,
                    aria_label: "4 unread", "4" }}
            }}
            Anchor {{
                span {{ aria_label: "show 8 shared files", "📁" }}
                Badge {{ color: Color::Default, size: Size::Sm,
                    aria_label: "8 files", "8" }}
            }}
            Anchor {{
                span {{ aria_label: "show 2 confirmed events", "📅" }}
                Badge {{ color: Color::Success, size: Size::Sm,
                    aria_label: "2 events", "2" }}
            }}
            Anchor {{
                span {{ aria_label: "show 1 critical alert", "⚠" }}
                Badge {{ color: Color::Warning, size: Size::Sm,
                    aria_label: "1 alert", "1" }}
            }}
        }}
    }}
}}"#
                        }
                        Example11 {}
                    }
                }
            }

            section { aria_labelledby: "nav-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "nav-heading", class: "text-xl font-semibold text-white mb-6", "Navigation Items" }
                div { class: "grid grid-cols-1 gap-8",
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Badge on Nav List" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size, Variant}};
use dioxus::prelude::*;

// Include the count in aria-label so screen readers
// announce it with the folder name, WCAG 1.3.1.
#[component]
fn NavBadges() -> Element {{
    rsx! {{
        nav {{ aria_label: "Mail folders",
            ul {{
                li {{ aria_label: "Inbox, 4 unread messages",
                    aria_current: "page",
                    span {{ "Inbox" }}
                    Badge {{ color: Color::Accent,
                        variant: Variant::Soft, size: Size::Sm,
                        style: "position:static;transform:none;",
                        aria_label: "4 unread", "4" }}
                }}
                li {{ aria_label: "Sent", span {{ "Sent" }} }}
                li {{ aria_label: "Drafts, 12 unread messages",
                    span {{ "Drafts" }}
                    Badge {{ color: Color::Default,
                        variant: Variant::Soft, size: Size::Sm,
                        style: "position:static;transform:none;",
                        aria_label: "12 unread", "12" }}
                }}
                li {{ aria_label: "Spam, more than 99 unread messages",
                    span {{ "Spam" }}
                    Badge {{ color: Color::Default,
                        variant: Variant::Soft, size: Size::Sm,
                        style: "position:static;transform:none;",
                        aria_label: "99+ unread", "99+" }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example12 {}
                    }
                }
            }

            section { aria_labelledby: "matrix-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "matrix-heading", class: "text-xl font-semibold text-white mb-6", "Full Matrix" }
                div { class: "grid grid-cols-1 gap-8",
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "All Variants x Colors" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Placement, Size, Variant}};
use dioxus::prelude::*;

const VARIANTS: &[(Variant, &str)] = &[
    (Variant::Primary, "primary"),
    (Variant::Secondary, "secondary"),
    (Variant::Soft, "soft"),
];
const COLORS: &[(Color, &str)] = &[
    (Color::Accent, "Accent"), (Color::Default, "Default"),
    (Color::Success, "Success"), (Color::Warning, "Warning"),
    (Color::Danger, "Danger"),
];

// 3 variants x 5 colors + dot row.
// Useful for visual regression and style guides.
#[component]
fn MatrixTable() -> Element {{
    rsx! {{
        table {{
            thead {{
                tr {{
                    th {{ "" }}
                    for (_, name) in COLORS {{
                        th {{ "{{name}}" }}
                    }}
                }}
            }}
            tbody {{
                for (variant, vlabel) in VARIANTS {{
                    tr {{
                        th {{ "{{vlabel}}" }}
                        for (color, _) in COLORS {{
                            td {{
                                Anchor {{
                                    span {{ "FP" }}
                                    Badge {{ color: *color, size: Size::Sm,
                                        variant: *variant, aria_label: "5", "5" }}
                                }}
                            }}
                        }}
                    }}
                }}
                tr {{
                    th {{ "dot" }}
                    for (color, clabel) in COLORS {{
                        td {{
                            Anchor {{
                                span {{ "FP" }}
                                Badge {{ color: *color, size: Size::Sm,
                                    placement: Placement::BottomRight,
                                    aria_label: *clabel }}
                            }}
                        }}
                    }}
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example13 {}
                    }
                }
            }

            section { aria_labelledby: "custom-heading", class: "w-full max-w-6xl mb-12",
                h2 { id: "custom-heading", class: "text-xl font-semibold text-white mb-6", "Customization" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-8",
                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Custom CSS Classes" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use badges_rs::dioxus::{{Badge, Anchor}};
use badges_rs::{{Color, Size, Variant}};
use dioxus::prelude::*;

// Use `class` to layer in your own Tailwind or
// CSS classes on top of the component defaults.
#[component]
fn CustomStyle() -> Element {{
    rsx! {{
        div {{ style: "display:flex;align-items:center;gap:2rem;",
            Anchor {{
                span {{
                    style: "border:2px solid #7c3aed;...",
                    aria_label: "KW, 5 notifications",
                    "KW"
                }}
                Badge {{
                    class: "font-semibold",
                    color: Color::Accent,
                    size: Size::Sm,
                    variant: Variant::Soft,
                    aria_label: "5 notifications",
                    "5"
                }}
            }}
            Anchor {{
                span {{
                    style: "border-radius:8px;...",
                    aria_label: "TS, 2 files",
                    "TS"
                }}
                Badge {{
                    color: Color::Success,
                    size: Size::Md,
                    variant: Variant::Secondary,
                    aria_label: "2 files",
                    "2"
                }}
            }}
        }}
    }}
}}"#
                        }
                        Example14 {}
                    }

                    article { class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "BEM CSS Classes Reference" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre",
r#"/* Global CSS customization via BEM classes:    */
/* .badge         - base container element      */
/* .badge__label  - inner text / icon wrapper   */
/* .badge-anchor  - position:relative wrapper   */
/*                                              */
/* Color modifiers:                             */
/*   .badge--accent  .badge--default            */
/*   .badge--success .badge--warning            */
/*   .badge--danger                             */
/*                                              */
/* Variant modifiers:                           */
/*   .badge--primary   (filled, default)        */
/*   .badge--secondary (outlined)               */
/*   .badge--soft      (tinted background)      */
/*                                              */
/* Size modifiers:                              */
/*   .badge--sm  .badge--md  .badge--lg         */
/*                                              */
/* Placement modifiers:                         */
/*   .badge--top-right   .badge--top-left       */
/*   .badge--bottom-right .badge--bottom-left   */

@layer components {{
    .badge {{ @apply rounded-full font-semibold; }}
    .badge--accent {{ @apply shadow-sm; }}
}}"#
                        }
                        div { style: "display:flex;align-items:center;gap:1rem;flex-wrap:wrap;justify-content:center;",
                            Anchor {
                                span { style: "{AVATAR}", aria_label: "Accent badge", "FP" }
                                Badge { color: Color::Accent, size: Size::Sm, aria_label: "5", "5" }
                            }
                            Anchor {
                                span { style: "{AVATAR}", aria_label: "Success soft badge", "AB" }
                                Badge { color: Color::Success, size: Size::Sm, variant: Variant::Soft, aria_label: "3", "3" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Stylesheet { href: asset!("assets/main.css") }
        LandingPage {}
    }
}
