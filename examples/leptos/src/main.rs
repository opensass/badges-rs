// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use badges_rs::leptos::{Anchor, Badge};
use badges_rs::{Color, Placement, Shape, Size, Variant};
use leptos::prelude::*;

static AVATAR: &str = "display:inline-flex;width:40px;height:40px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static AVATAR_SM: &str = "display:inline-flex;width:32px;height:32px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:12px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static AVATAR_LG: &str = "display:inline-flex;width:48px;height:48px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static ICON_BTN: &str = "display:inline-flex;width:44px;height:44px;border-radius:0.5rem;background:#1e293b;border:1px solid #334155;align-items:center;justify-content:center;cursor:default;flex-shrink:0;";
static SQUARE: &str =
    "display:inline-block;width:32px;height:32px;background:#3b82f6;flex-shrink:0;";
static CIRCLE: &str = "display:inline-block;width:32px;height:32px;border-radius:50%;background:#3b82f6;flex-shrink:0;";

#[component]
fn Example1() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;">
            <Anchor>
                <span style=AVATAR aria-label="FP user, 5 notifications">"FP"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="5 notifications">"5"</Badge>
            </Anchor>
            <Anchor>
                <span style=AVATAR aria-label="AB user, new">"AB"</span>
                <Badge shape=Shape::Circle color=Color::Accent size=Size::Sm aria_label="New">"New"</Badge>
            </Anchor>
            <Anchor>
                <span style=AVATAR aria-label="CD user, online">"CD"</span>
                <Badge shape=Shape::Circle color=Color::Success placement=Placement::BottomRight size=Size::Sm aria_label="Online" />
            </Anchor>
        </div>
    }
}

#[component]
fn Example2() -> impl IntoView {
    let visible = RwSignal::new(true);
    view! {
        <div style="display:flex;flex-direction:column;align-items:center;gap:1.5rem;">
            <Anchor>
                <span style=AVATAR
                    aria-label=move || if visible.get() { "User, 4 unread messages" } else { "User inbox" }>
                    "FP"
                </span>
                {move || visible.get().then(|| view! {
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="4 unread messages">"4"</Badge>
                })}
            </Anchor>
            <button
                on:click=move |_| visible.update(|v| *v = !*v)
                aria-pressed=move || visible.get().to_string()
                style="padding:0.4rem 1rem;border:1px solid #334155;border-radius:9999px;background:#1e293b;color:#e2e8f0;cursor:pointer;font-size:0.875rem;"
            >
                {move || if visible.get() { "Hide Badge" } else { "Show Badge" }}
            </button>
        </div>
    }
}

#[component]
fn Example3() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;">
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=ICON_BTN aria-label="show 99 unread messages">"✉"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="99">"99"</Badge>
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"99"</span>
            </div>
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=ICON_BTN aria-label="show more than 99 unread messages">"✉"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="99+">"99+"</Badge>
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"100"</span>
            </div>
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=ICON_BTN aria-label="show more than 999 unread messages">"✉"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="999+">"999+"</Badge>
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"1000"</span>
            </div>
        </div>
    }
}

#[component]
fn Example4() -> impl IntoView {
    const VARIANTS: [(Variant, &str); 3] = [
        (Variant::Primary, "primary"),
        (Variant::Secondary, "secondary"),
        (Variant::Soft, "soft"),
    ];
    const COLORS: [(Color, &str); 5] = [
        (Color::Accent, "Accent"),
        (Color::Default, "Default"),
        (Color::Success, "Success"),
        (Color::Warning, "Warning"),
        (Color::Danger, "Danger"),
    ];
    view! {
        <div style="display:flex;flex-direction:column;gap:1.5rem;">
            {VARIANTS.into_iter().map(|(variant, vlabel)| view! {
                <div>
                    <p style="font-size:0.75rem;font-weight:600;color:#6b7280;margin-bottom:0.5rem;">{vlabel}</p>
                    <div style="display:flex;align-items:center;gap:1.25rem;flex-wrap:wrap;">
                        {COLORS.into_iter().map(|(color, _)| view! {
                            <Anchor>
                                <span style=AVATAR>"FP"</span>
                                <Badge shape=Shape::Circle color=color size=Size::Sm variant=variant aria_label="5">"5"</Badge>
                            </Anchor>
                        }).collect_view()}
                    </div>
                </div>
            }).collect_view()}
        </div>
    }
}

#[component]
fn Example5() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:flex-end;gap:2rem;flex-wrap:wrap;justify-content:center;">
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=AVATAR_SM aria-label="Small user">"FP"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="5">"5"</Badge>
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"Sm"</span>
            </div>
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=AVATAR aria-label="Medium user">"FP"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Md aria_label="5">"5"</Badge>
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"Md"</span>
            </div>
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=AVATAR_LG aria-label="Large user">"FP"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Lg aria_label="5">"5"</Badge>
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"Lg"</span>
            </div>
        </div>
    }
}

#[component]
fn Example6() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:2rem;flex-wrap:wrap;justify-content:center;">
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=SQUARE aria-label="Rectangle with count" />
                    <Badge shape=Shape::Rectangle color=Color::Danger size=Size::Sm aria_label="1">"1"</Badge>
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"rect + count"</span>
            </div>
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=SQUARE aria-label="Rectangle dot" />
                    <Badge shape=Shape::Rectangle color=Color::Danger size=Size::Sm aria_label="Active" />
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"rect + dot"</span>
            </div>
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=CIRCLE aria-label="Circle with count" />
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="1">"1"</Badge>
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"circle + count"</span>
            </div>
            <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                <Anchor>
                    <span style=CIRCLE aria-label="Circle dot" />
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="Active" />
                </Anchor>
                <span style="font-size:0.75rem;color:#6b7280;">"circle + dot"</span>
            </div>
        </div>
    }
}

#[component]
fn Example7() -> impl IntoView {
    const COLORS: [(Color, &str); 5] = [
        (Color::Accent, "Accent"),
        (Color::Default, "Default"),
        (Color::Success, "Success"),
        (Color::Warning, "Warning"),
        (Color::Danger, "Danger"),
    ];
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;">
            {COLORS.into_iter().map(|(color, label)| view! {
                <Anchor>
                    <span style=AVATAR aria-label=format!("User {label} status")>"FP"</span>
                    <Badge shape=Shape::Circle color=color size=Size::Sm aria_label=label />
                </Anchor>
            }).collect_view()}
        </div>
    }
}

#[component]
fn Example8() -> impl IntoView {
    const STATUSES: [(&str, Color, &str); 4] = [
        ("Online", Color::Success, "Online"),
        ("Away", Color::Warning, "Away"),
        ("Busy", Color::Danger, "Busy"),
        ("Offline", Color::Default, "Offline"),
    ];
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;">
            {STATUSES.into_iter().map(|(label, color, aria)| view! {
                <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                    <Anchor>
                        <span style=AVATAR aria-label=format!("User, {label}")>"FP"</span>
                        <Badge shape=Shape::Circle color=color placement=Placement::BottomRight size=Size::Sm aria_label=aria />
                    </Anchor>
                    <span style="font-size:0.75rem;color:#6b7280;">{label}</span>
                </div>
            }).collect_view()}
        </div>
    }
}

#[component]
fn Example9() -> impl IntoView {
    const PLACEMENTS: [(Placement, &str); 4] = [
        (Placement::TopRight, "top-right"),
        (Placement::TopLeft, "top-left"),
        (Placement::BottomRight, "bottom-right"),
        (Placement::BottomLeft, "bottom-left"),
    ];
    view! {
        <div style="display:flex;align-items:center;gap:2rem;flex-wrap:wrap;justify-content:center;">
            {PLACEMENTS.into_iter().map(|(placement, label)| view! {
                <div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;">
                    <Anchor>
                        <span style=AVATAR>"FP"</span>
                        <Badge shape=Shape::Circle color=Color::Accent placement=placement size=Size::Sm aria_label=label />
                    </Anchor>
                    <span style="font-size:0.75rem;color:#6b7280;">{label}</span>
                </div>
            }).collect_view()}
        </div>
    }
}

#[component]
fn Example10() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;">
            <Anchor>
                <span style=AVATAR aria-label="User, 5 unread">"FP"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="5 unread messages">"5"</Badge>
            </Anchor>
            <Anchor>
                <span style=AVATAR aria-label="User, new">"FP"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="New">"New"</Badge>
            </Anchor>
            <Anchor>
                <span style=AVATAR aria-label="User, 99+ notifications">"FP"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="99+ notifications">"99+"</Badge>
            </Anchor>
        </div>
    }
}

#[component]
fn Example11() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;flex-wrap:wrap;justify-content:center;">
            <Anchor>
                <span style=ICON_BTN aria-label="show 4 unread messages">"✉"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm aria_label="4 unread">"4"</Badge>
            </Anchor>
            <Anchor>
                <span style=ICON_BTN aria-label="show 8 shared files">"📁"</span>
                <Badge shape=Shape::Circle color=Color::Default size=Size::Sm aria_label="8 files">"8"</Badge>
            </Anchor>
            <Anchor>
                <span style=ICON_BTN aria-label="show 2 confirmed events">"📅"</span>
                <Badge shape=Shape::Circle color=Color::Success size=Size::Sm aria_label="2 events">"2"</Badge>
            </Anchor>
            <Anchor>
                <span style=ICON_BTN aria-label="show 1 critical alert">"⚠"</span>
                <Badge shape=Shape::Circle color=Color::Warning size=Size::Sm aria_label="1 alert">"1"</Badge>
            </Anchor>
        </div>
    }
}

#[component]
fn Example12() -> impl IntoView {
    view! {
        <nav aria-label="Mail folders"
            style="background:#1e293b;border-radius:0.5rem;border:1px solid #334155;width:100%;max-width:280px;">
            <ul style="list-style:none;margin:0;padding:0;">
                <li aria-label="Inbox, 4 unread messages" aria-current="page"
                    style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;background:#334155;border-left:3px solid #7c3aed;border-bottom:1px solid #334155;">
                    <span style="color:#e2e8f0;font-size:0.875rem;">"Inbox"</span>
                    <Badge shape=Shape::Circle color=Color::Accent variant=Variant::Soft size=Size::Sm
                        style="position:static;transform:none;" aria_label="4 unread">"4"</Badge>
                </li>
                <li aria-label="Sent"
                    style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;border-bottom:1px solid #334155;">
                    <span style="color:#e2e8f0;font-size:0.875rem;">"Sent"</span>
                </li>
                <li aria-label="Drafts, 12 unread messages"
                    style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;border-bottom:1px solid #334155;">
                    <span style="color:#e2e8f0;font-size:0.875rem;">"Drafts"</span>
                    <Badge shape=Shape::Circle color=Color::Default variant=Variant::Soft size=Size::Sm
                        style="position:static;transform:none;" aria_label="12 unread">"12"</Badge>
                </li>
                <li aria-label="Spam, more than 99 unread messages"
                    style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;">
                    <span style="color:#e2e8f0;font-size:0.875rem;">"Spam"</span>
                    <Badge shape=Shape::Circle color=Color::Default variant=Variant::Soft size=Size::Sm
                        style="position:static;transform:none;" aria_label="99+ unread">"99+"</Badge>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn Example13() -> impl IntoView {
    const VARIANTS: [(Variant, &str); 3] = [
        (Variant::Primary, "primary"),
        (Variant::Secondary, "secondary"),
        (Variant::Soft, "soft"),
    ];
    const COLORS: [(Color, &str); 5] = [
        (Color::Accent, "Accent"),
        (Color::Default, "Default"),
        (Color::Success, "Success"),
        (Color::Warning, "Warning"),
        (Color::Danger, "Danger"),
    ];
    view! {
        <div class="w-full overflow-x-auto bg-gray-800 rounded-xl p-4" role="region" aria-label="Badge variant matrix">
            <table class="w-full text-left border-collapse">
                <caption class="sr-only">"Badge style matrix - variants x colors"</caption>
                <thead>
                    <tr>
                        <th scope="col" class="p-3 text-gray-400 font-medium">""</th>
                        {COLORS.into_iter().map(|(_, name)| view! {
                            <th scope="col" class="p-3 text-gray-400 font-medium text-center">{name}</th>
                        }).collect_view()}
                    </tr>
                </thead>
                <tbody>
                    {VARIANTS.into_iter().map(|(variant, vlabel)| view! {
                        <tr>
                            <th scope="row" class="p-3 text-gray-400 capitalize">{vlabel}</th>
                            {COLORS.into_iter().map(|(color, _)| view! {
                                <td class="p-3 text-center border-t border-white/10">
                                    <Anchor>
                                        <span style=AVATAR>"FP"</span>
                                        <Badge shape=Shape::Circle color=color size=Size::Sm variant=variant aria_label="5">"5"</Badge>
                                    </Anchor>
                                </td>
                            }).collect_view()}
                        </tr>
                    }).collect_view()}
                    <tr>
                        <th scope="row" class="p-3 text-gray-400">"dot"</th>
                        {COLORS.into_iter().map(|(color, clabel)| view! {
                            <td class="p-3 text-center border-t border-white/10">
                                <Anchor>
                                    <span style=AVATAR>"FP"</span>
                                    <Badge shape=Shape::Circle color=color size=Size::Sm placement=Placement::BottomRight aria_label=clabel />
                                </Anchor>
                            </td>
                        }).collect_view()}
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn Example14() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:2rem;flex-wrap:wrap;justify-content:center;">
            <Anchor>
                <span
                    style="display:inline-flex;width:40px;height:40px;border-radius:50%;background:#1e293b;border:2px solid #7c3aed;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;"
                    aria-label="Kate Wilson, 5 notifications">
                    "KW"
                </span>
                <Badge shape=Shape::Circle class="font-semibold" color=Color::Accent size=Size::Sm
                    variant=Variant::Soft aria_label="5 notifications">"5"</Badge>
            </Anchor>
            <Anchor>
                <span
                    style="display:inline-flex;width:40px;height:40px;border-radius:8px;background:#1e293b;border:1px solid #334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;"
                    aria-label="TS user, 2 files">
                    "TS"
                </span>
                <Badge shape=Shape::Circle color=Color::Success size=Size::Md variant=Variant::Secondary
                    aria_label="2 files">"2"</Badge>
            </Anchor>
        </div>
    }
}

fn card(title: &'static str, code: &'static str, children: impl IntoView) -> impl IntoView {
    view! {
        <article class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black">
            <h3 class="text-xl font-bold mb-2 self-start">{title}</h3>
            <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre">{code}</pre>
            <div class="flex justify-center w-full">{children}</div>
        </article>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
            <div
                class="min-h-screen flex flex-col items-center justify-center"
                style="color:#5e5c7f;background-color:#303030;font-family:'Rubik',sans-serif;overflow-x:hidden;"
            >
                <h1 class="text-3xl font-bold mb-8 text-white">"Badges RS Leptos Examples"</h1>

                <section aria-labelledby="basic-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="basic-heading" class="text-xl font-semibold text-white mb-6">"Basic"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                        {card("Basic Badge", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Placement, Size, Shape};
use leptos::prelude::*;

#[component]
pub fn BasicBadge() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            <Anchor>
                <span aria-label="FP, 5 notifications">"FP"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                    aria_label="5 notifications">"5"</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="AB, new">"AB"</span>
                <Badge shape=Shape::Circle color=Color::Accent size=Size::Sm
                    aria_label="New">"New"</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="CD, online">"CD"</span>
                <Badge shape=Shape::Circle color=Color::Success
                    placement=Placement::BottomRight
                    size=Size::Sm aria_label="Online" />
            </Anchor>
        </div>
    }
}"#, view! { <Example1 /> })}

                        {card("Visibility Toggle", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use leptos::prelude::*;

#[component]
pub fn VisibilityToggle() -> impl IntoView {
    let visible = RwSignal::new(true);
    view! {
        <div style="display:flex;flex-direction:column;
            align-items:center;gap:1.5rem;">
            <Anchor>
                <span aria-label=move || {
                    if visible.get() {
                        "User, 4 unread messages"
                    } else { "User inbox" }
                }>"FP"</span>
                {move || visible.get().then(|| view! {
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                        aria_label="4 unread messages">"4"</Badge>
                })}
            </Anchor>
            <button
                on:click=move |_| visible.update(|v| *v = !*v)
                aria-pressed=move || visible.get().to_string()>
                {move || if visible.get() {
                    "Hide Badge" } else { "Show Badge" }
                }
            </button>
        </div>
    }
}"#, view! { <Example2 /> })}

                        {card("Maximum Value", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use leptos::prelude::*;

// Display 99 exactly, or truncate with 99+ / 999+.
#[component]
pub fn MaxValue() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            <div>
                <Anchor>
                    <span aria-label="show 99 unread">"✉"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                        aria_label="99">"99"</Badge>
                </Anchor>
                <span>"99"</span>
            </div>
            <div>
                <Anchor>
                    <span aria-label="show more than 99">"✉"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                        aria_label="99+">"99+"</Badge>
                </Anchor>
                <span>"100"</span>
            </div>
            <div>
                <Anchor>
                    <span aria-label="show more than 999">"✉"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                        aria_label="999+">"999+"</Badge>
                </Anchor>
                <span>"1000"</span>
            </div>
        </div>
    }
}"#, view! { <Example3 /> })}
                    </div>
                </section>

                <section aria-labelledby="variants-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="variants-heading" class="text-xl font-semibold text-white mb-6">"Variants"</h2>
                    <div class="grid grid-cols-1 gap-8">
                        {card("Variant x Color Matrix", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Variant, Shape};
use leptos::prelude::*;

const VARIANTS: [(Variant, &str); 3] = [
    (Variant::Primary,   "primary"),
    (Variant::Secondary, "secondary"),
    (Variant::Soft,      "soft"),
];
const COLORS: [(Color, &str); 5] = [
    (Color::Accent, "Accent"), (Color::Default, "Default"),
    (Color::Success, "Success"), (Color::Warning, "Warning"),
    (Color::Danger, "Danger"),
];

#[component]
pub fn VariantMatrix() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:1.5rem;">
            {VARIANTS.into_iter().map(|(variant, vlabel)| view! {
                <div>
                    <p>{vlabel}</p>
                    <div style="display:flex;align-items:center;gap:1.25rem;">
                        {COLORS.into_iter().map(|(color, _)| view! {
                            <Anchor>
                                <span>"FP"</span>
                                <Badge shape=Shape::Circle color=color size=Size::Sm
                                    variant=variant aria_label="5">"5"</Badge>
                            </Anchor>
                        }).collect_view()}
                    </div>
                </div>
            }).collect_view()}
        </div>
    }
}"#, view! { <Example4 /> })}
                    </div>
                </section>

                <section aria-labelledby="sizes-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="sizes-heading" class="text-xl font-semibold text-white mb-6">"Sizes"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                        {card("Sizes", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use leptos::prelude::*;

// Size::Sm → 16x16 px, 10 px font
// Size::Md → 20x20 px, 11 px font (default)
// Size::Lg → 24x24 px, 12 px font
#[component]
pub fn Sizes() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:flex-end;gap:2rem;">
            <div>
                <Anchor>
                    <span aria-label="Small">"FP"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                        aria_label="5">"5"</Badge>
                </Anchor>
                <span>"Sm"</span>
            </div>
            <div>
                <Anchor>
                    <span aria-label="Medium">"FP"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Md
                        aria_label="5">"5"</Badge>
                </Anchor>
                <span>"Md"</span>
            </div>
            <div>
                <Anchor>
                    <span aria-label="Large">"FP"</span>
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Lg
                        aria_label="5">"5"</Badge>
                </Anchor>
                <span>"Lg"</span>
            </div>
        </div>
    }
}"#, view! { <Example5 /> })}
                        {card("Overlap Shapes", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use leptos::prelude::*;

// Badge anchors to any shape: square, circle, icon.
#[component]
pub fn Overlap() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:2rem;">
            <div>
                <Anchor>
                    <span style="width:32px;height:32px;
                        background:#3b82f6;display:inline-block;"
                        aria-label="Rectangle" />
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                        aria_label="1">"1"</Badge>
                </Anchor>
                <span>"rect"</span>
            </div>
            <div>
                <Anchor>
                    <span style="border-radius:50%;width:32px;
                        height:32px;background:#3b82f6;
                        display:inline-block;"
                        aria-label="Circle" />
                    <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                        aria_label="1">"1"</Badge>
                </Anchor>
                <span>"circle"</span>
            </div>
        </div>
    }
}"#, view! { <Example6 /> })}
                    </div>
                </section>

                <section aria-labelledby="colors-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="colors-heading" class="text-xl font-semibold text-white mb-6">"Colors"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                        {card("Color - Dot Mode", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use leptos::prelude::*;

const COLORS: [(Color, &str); 5] = [
    (Color::Accent, "Accent"),
    (Color::Default, "Default"),
    (Color::Success, "Success"),
    (Color::Warning, "Warning"),
    (Color::Danger, "Danger"),
];

// Omit children for a compact dot indicator.
#[component]
pub fn ColorDots() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            {COLORS.into_iter().map(|(color, label)| view! {
                <Anchor>
                    <span aria-label=format!("User {label}")>"FP"</span>
                    <Badge shape=Shape::Circle color=color size=Size::Sm aria_label=label />
                </Anchor>
            }).collect_view()}
        </div>
    }
}"#, view! { <Example7 /> })}
                        {card("Status Indicators", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Placement, Size, Shape};
use leptos::prelude::*;

const STATUSES: [(&str, Color, &str); 4] = [
    ("Online",  Color::Success, "Online"),
    ("Away",    Color::Warning, "Away"),
    ("Busy",    Color::Danger,  "Busy"),
    ("Offline", Color::Default, "Offline"),
];

// Bottom-right placement for presence / status dots.
#[component]
pub fn StatusDots() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            {STATUSES.into_iter().map(|(label, color, aria)| view! {
                <div style="display:flex;flex-direction:column;
                    align-items:center;gap:0.5rem;">
                    <Anchor>
                        <span aria-label=format!("User, {label}")>"FP"</span>
                        <Badge shape=Shape::Circle color=color
                            placement=Placement::BottomRight
                            size=Size::Sm aria_label=aria />
                    </Anchor>
                    <span>{label}</span>
                </div>
            }).collect_view()}
        </div>
    }
}"#, view! { <Example8 /> })}
                    </div>
                </section>

                <section aria-labelledby="placement-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="placement-heading" class="text-xl font-semibold text-white mb-6">"Placements"</h2>
                    <div class="grid grid-cols-1 gap-8">
                        {card("All Four Corners", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Placement, Size, Shape};
use leptos::prelude::*;

const PLACEMENTS: [(Placement, &str); 4] = [
    (Placement::TopRight,    "top-right"),
    (Placement::TopLeft,     "top-left"),
    (Placement::BottomRight, "bottom-right"),
    (Placement::BottomLeft,  "bottom-left"),
];

#[component]
pub fn Placements() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:2rem;">
            {PLACEMENTS.into_iter().map(|(placement, label)| view! {
                <div style="display:flex;flex-direction:column;
                    align-items:center;gap:0.5rem;">
                    <Anchor>
                        <span>"FP"</span>
                        <Badge shape=Shape::Circle color=Color::Accent placement=placement
                            size=Size::Sm aria_label=label />
                    </Anchor>
                    <span>{label}</span>
                </div>
            }).collect_view()}
        </div>
    }
}"#, view! { <Example9 /> })}
                    </div>
                </section>

                <section aria-labelledby="content-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="content-heading" class="text-xl font-semibold text-white mb-6">"With Content"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                        {card("Text, Numbers, Icons", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use leptos::prelude::*;

// Badge accepts any children: text, numbers,
// "99+", or SVG icons. Omit children for a dot.
#[component]
pub fn WithContent() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            <Anchor>
                <span aria-label="User, 5 unread">"FP"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                    aria_label="5 unread messages">"5"</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="User, new">"FP"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                    aria_label="New">"New"</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="User, 99+ notifications">"FP"</span>
                <Badge shape=Shape::Circle color=Color::Danger size=Size::Sm
                    aria_label="99+ notifications">"99+"</Badge>
            </Anchor>
        </div>
    }
}"#, view! { <Example10 /> })}
                        {card("On Icon Buttons", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use leptos::prelude::*;

// Badge is not restricted to avatars;
// it anchors to any interactive element.
#[component]
pub fn IconBadges() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            <Anchor>
                <span aria-label="show 4 unread messages">"✉"</span>
                <Badge color=Color::Danger size=Size::Sm
                    aria_label="4 unread">"4"</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="show 8 shared files">"📁"</span>
                <Badge color=Color::Default size=Size::Sm
                    aria_label="8 files">"8"</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="show 2 confirmed events">"📅"</span>
                <Badge color=Color::Success size=Size::Sm
                    aria_label="2 events">"2"</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="show 1 critical alert">"⚠"</span>
                <Badge color=Color::Warning size=Size::Sm
                    aria_label="1 alert">"1"</Badge>
            </Anchor>
        </div>
    }
}"#, view! { <Example11 /> })}
                    </div>
                </section>

                <section aria-labelledby="nav-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="nav-heading" class="text-xl font-semibold text-white mb-6">"Navigation Items"</h2>
                    <div class="grid grid-cols-1 gap-8">
                        {card("Badge on Nav List", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Variant};
use leptos::prelude::*;

// Include the count in aria-label so screen readers
// announce it with the folder name - WCAG 1.3.1.
#[component]
pub fn NavBadges() -> impl IntoView {
    view! {
        <nav aria-label="Mail folders">
            <ul style="list-style:none;padding:0;margin:0;">
                <li aria-label="Inbox, 4 unread messages"
                    aria-current="page">
                    <span>"Inbox"</span>
                    <Badge color=Color::Accent
                        variant=Variant::Soft size=Size::Sm
                        style="position:static;transform:none;"
                        aria_label="4 unread">"4"</Badge>
                </li>
                <li aria-label="Sent">
                    <span>"Sent"</span>
                </li>
                <li aria-label="Drafts, 12 unread messages">
                    <span>"Drafts"</span>
                    <Badge color=Color::Default
                        variant=Variant::Soft size=Size::Sm
                        style="position:static;transform:none;"
                        aria_label="12 unread">"12"</Badge>
                </li>
                <li aria-label="Spam, more than 99 unread messages">
                    <span>"Spam"</span>
                    <Badge color=Color::Default
                        variant=Variant::Soft size=Size::Sm
                        style="position:static;transform:none;"
                        aria_label="99+ unread">"99+"</Badge>
                </li>
            </ul>
        </nav>
    }
}"#, view! { <Example12 /> })}
                    </div>
                </section>

                <section aria-labelledby="matrix-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="matrix-heading" class="text-xl font-semibold text-white mb-6">"Full Matrix"</h2>
                    <div class="grid grid-cols-1 gap-8">
                        {card("All Variants x Colors", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Placement, Size, Variant};
use leptos::prelude::*;

const VARIANTS: [(Variant, &str); 3] = [
    (Variant::Primary, "primary"),
    (Variant::Secondary, "secondary"),
    (Variant::Soft, "soft"),
];
const COLORS: [(Color, &str); 5] = [
    (Color::Accent, "Accent"), (Color::Default, "Default"),
    (Color::Success, "Success"), (Color::Warning, "Warning"),
    (Color::Danger, "Danger"),
];

// 3 variants x 5 colors + dot row.
// Great for visual regression and style guides.
#[component]
pub fn MatrixTable() -> impl IntoView {
    view! {
        <table>
            <thead>
                <tr>
                    <th>{""}</th>
                    {COLORS.into_iter().map(|(_, n)| view!{
                        <th>{n}</th>
                    }).collect_view()}
                </tr>
            </thead>
            <tbody>
                {VARIANTS.into_iter().map(|(variant, vlabel)| view! {
                    <tr>
                        <th>{vlabel}</th>
                        {COLORS.into_iter().map(|(color, _)| view! {
                            <td>
                                <Anchor>
                                    <span>"FP"</span>
                                    <Badge color=color size=Size::Sm
                                        variant=variant aria_label="5">
                                        "5"
                                    </Badge>
                                </Anchor>
                            </td>
                        }).collect_view()}
                    </tr>
                }).collect_view()}
                <tr>
                    <th>"dot"</th>
                    {COLORS.into_iter().map(|(color, clabel)| view! {
                        <td>
                            <Anchor>
                                <span>"FP"</span>
                                <Badge color=color size=Size::Sm
                                    placement=Placement::BottomRight
                                    aria_label=clabel />
                            </Anchor>
                        </td>
                    }).collect_view()}
                </tr>
            </tbody>
        </table>
    }
}"#, view! { <Example13 /> })}
                    </div>
                </section>

                <section aria-labelledby="custom-heading" class="w-full max-w-6xl mb-12">
                    <h2 id="custom-heading" class="text-xl font-semibold text-white mb-6">"Customization"</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                        {card("Custom CSS Classes", r#"use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size, Variant};
use leptos::prelude::*;

// Use `class` to layer in your own Tailwind or
// CSS classes on top of the component defaults.
#[component]
pub fn CustomStyle() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:2rem;">
            <Anchor>
                <span style="border:2px solid #7c3aed;"
                    aria-label="KW, 5 notifications">
                    "KW"
                </span>
                <Badge
                    class="font-semibold"
                    color=Color::Accent
                    size=Size::Sm
                    variant=Variant::Soft
                    aria_label="5 notifications">
                    "5"
                </Badge>
            </Anchor>
            <Anchor>
                <span style="border-radius:8px;border:1px solid #334155;"
                    aria-label="TS, 2 files">
                    "TS"
                </span>
                <Badge
                    color=Color::Success
                    size=Size::Md
                    variant=Variant::Secondary
                    aria_label="2 files">
                    "2"
                </Badge>
            </Anchor>
        </div>
    }
}"#, view! { <Example14 /> })}
                        {card("BEM CSS Classes Reference", r#"/* Global CSS customization via BEM classes:    */
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

@layer components {
    .badge { @apply rounded-full font-semibold; }
    .badge--accent { @apply shadow-sm; }
}"#, view! {
        <div style="display:flex;align-items:center;gap:1rem;flex-wrap:wrap;justify-content:center;">
            <Anchor>
                <span style=AVATAR aria-label="Accent badge">"FP"</span>
                <Badge color=Color::Accent size=Size::Sm aria_label="5">"5"</Badge>
            </Anchor>
            <Anchor>
                <span style=AVATAR aria-label="Soft success badge">"AB"</span>
                <Badge color=Color::Success size=Size::Sm variant=Variant::Soft aria_label="3">"3"</Badge>
            </Anchor>
            <Anchor>
                <span style=AVATAR aria-label="Secondary danger badge">"CD"</span>
                <Badge color=Color::Danger size=Size::Sm variant=Variant::Secondary aria_label="9">"9"</Badge>
            </Anchor>
        </div>
    })}
                    </div>
                </section>
            </div>
        }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
