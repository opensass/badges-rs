// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use badges_rs::yew::{Anchor, Badge};
use badges_rs::{Color, Placement, Shape, Size, Variant};
use yew::prelude::*;

static AVATAR: &str = "display:inline-flex;width:40px;height:40px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static AVATAR_SM: &str = "display:inline-flex;width:32px;height:32px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:12px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static AVATAR_LG: &str = "display:inline-flex;width:48px;height:48px;border-radius:50%;background:#334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;flex-shrink:0;";
static ICON_BTN: &str = "display:inline-flex;width:44px;height:44px;border-radius:0.5rem;background:#1e293b;border:1px solid #334155;align-items:center;justify-content:center;cursor:default;flex-shrink:0;";
static SQUARE: &str =
    "display:inline-block;width:32px;height:32px;background:#3b82f6;flex-shrink:0;";
static CIRCLE: &str = "display:inline-block;width:32px;height:32px;border-radius:50%;background:#3b82f6;flex-shrink:0;";

fn card(title: &'static str, code: &'static str, children: Html) -> Html {
    html! {
        <article
            class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md text-black"
        >
            <h3 class="text-xl font-bold mb-2 self-start">{ title }</h3>
            <pre
                class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto whitespace-pre"
            >
                { code }
            </pre>
            <div class="flex justify-center w-full">{ children }</div>
        </article>
    }
}

#[function_component(Example1)]
fn example1() -> Html {
    html! {
        <div class="flex items-center gap-6 flex-wrap justify-center">
            <Anchor>
                <span style={AVATAR} aria-label="FP user, 5 notifications">{ "FP" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Danger}
                    size={Size::Sm}
                    aria_label="5 notifications"
                >
                    { "5" }
                </Badge>
            </Anchor>
            <Anchor>
                <span style={AVATAR} aria-label="AB user, new">{ "AB" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Accent}
                    size={Size::Sm}
                    aria_label="New"
                >
                    { "New" }
                </Badge>
            </Anchor>
            <Anchor>
                <span style={AVATAR} aria-label="CD user, online">{ "CD" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Success}
                    placement={Placement::BottomRight}
                    size={Size::Sm}
                    aria_label="Online"
                />
            </Anchor>
        </div>
    }
}

#[function_component(Example2)]
fn example2() -> Html {
    let visible = use_state(|| true);
    let toggle = {
        let visible = visible.clone();
        Callback::from(move |_: MouseEvent| visible.set(!*visible))
    };
    html! {
        <div class="flex flex-col items-center gap-6">
            <Anchor>
                <span
                    style={AVATAR}
                    aria-label={if *visible { "User, 4 unread messages" } else { "User inbox" }}
                >
                    { "FP" }
                </span>
                if *visible {
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="4 unread messages"
                    >
                        { "4" }
                    </Badge>
                }
            </Anchor>
            <button
                onclick={toggle}
                style="padding:0.4rem 1rem;border:1px solid #334155;border-radius:9999px;background:#1e293b;color:#e2e8f0;cursor:pointer;font-size:0.875rem;"
                aria-pressed={if *visible { "true" } else { "false" }}
            >
                { if *visible { "Hide Badge" } else { "Show Badge" } }
            </button>
        </div>
    }
}

#[function_component(Example3)]
fn example3() -> Html {
    html! {
        <div class="flex items-center gap-6 flex-wrap justify-center">
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={ICON_BTN} aria-label="show 99 unread messages">{ "✉" }</span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="99"
                    >
                        { "99" }
                    </Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{ "99" }</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={ICON_BTN} aria-label="show more than 99 unread messages">
                        { "✉" }
                    </span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="99+"
                    >
                        { "99+" }
                    </Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{ "100" }</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={ICON_BTN} aria-label="show more than 999 unread messages">
                        { "✉" }
                    </span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="999+"
                    >
                        { "999+" }
                    </Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{ "1000" }</span>
            </div>
        </div>
    }
}

#[function_component(Example4)]
fn example4() -> Html {
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
    html! {
        <div class="flex flex-col gap-6 w-full">
            { for VARIANTS.iter().map(|(variant, vlabel)| {
                let v = *variant;
                html! {
                    <div>
                        <p class="text-xs font-semibold text-gray-500 mb-3 capitalize">{*vlabel}</p>
                        <div class="flex items-center gap-5 flex-wrap">
                            { for COLORS.iter().map(|(color, _)| {
                                let c = *color;
                                html! {
                                    <Anchor>
                                        <span style={AVATAR}>{"FP"}</span>
                                        <Badge shape={Shape::Circle} color={c} size={Size::Sm} variant={v} aria_label="5">{"5"}</Badge>
                                    </Anchor>
                                }
                            }) }
                        </div>
                    </div>
                }
            }) }
        </div>
    }
}

#[function_component(Example5)]
fn example5() -> Html {
    html! {
        <div class="flex items-end gap-8 flex-wrap justify-center">
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={AVATAR_SM} aria-label="Small user">{ "FP" }</span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="5"
                    >
                        { "5" }
                    </Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{ "Sm" }</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={AVATAR} aria-label="Medium user">{ "FP" }</span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Md}
                        aria_label="5"
                    >
                        { "5" }
                    </Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{ "Md" }</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={AVATAR_LG} aria-label="Large user">{ "FP" }</span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Lg}
                        aria_label="5"
                    >
                        { "5" }
                    </Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{ "Lg" }</span>
            </div>
        </div>
    }
}

#[function_component(Example6)]
fn example6() -> Html {
    html! {
        <div class="flex items-center gap-8 flex-wrap justify-center">
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={SQUARE} aria-label="Rectangle with count" />
                    <Badge
                        shape={Shape::Rectangle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="1"
                    >
                        { "1" }
                    </Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{ "rect + count" }</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={SQUARE} aria-label="Rectangle dot" />
                    <Badge
                        shape={Shape::Rectangle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="Active"
                    />
                </Anchor>
                <span class="text-xs text-gray-500">{ "rect + dot" }</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={CIRCLE} aria-label="Circle with count" />
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="1"
                    >
                        { "1" }
                    </Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{ "circle + count" }</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span style={CIRCLE} aria-label="Circle dot" />
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Danger}
                        size={Size::Sm}
                        aria_label="Active"
                    />
                </Anchor>
                <span class="text-xs text-gray-500">{ "circle + dot" }</span>
            </div>
        </div>
    }
}

#[function_component(Example7)]
fn example7() -> Html {
    const COLORS: &[(Color, &str)] = &[
        (Color::Accent, "Accent"),
        (Color::Default, "Default"),
        (Color::Success, "Success"),
        (Color::Warning, "Warning"),
        (Color::Danger, "Danger"),
    ];
    html! {
        <div class="flex items-center gap-6 flex-wrap justify-center">
            { for COLORS.iter().map(|(color, label)| {
                let c = *color;
                html! {
                    <Anchor>
                        <span style={AVATAR}>{"FP"}</span>
                        <Badge shape={Shape::Circle} color={c} size={Size::Sm} aria_label={*label} />
                    </Anchor>
                }
            }) }
        </div>
    }
}

#[function_component(Example8)]
fn example8() -> Html {
    const STATUSES: &[(&str, Color, &str)] = &[
        ("Online", Color::Success, "Online"),
        ("Away", Color::Warning, "Away"),
        ("Busy", Color::Danger, "Busy"),
        ("Offline", Color::Default, "Offline"),
    ];
    html! {
        <div class="flex items-center gap-6 flex-wrap justify-center">
            { for STATUSES.iter().map(|(label, color, aria)| {
                let c = *color;
                html! {
                    <div class="flex flex-col items-center gap-2">
                        <Anchor>
                            <span style={AVATAR}>{"FP"}</span>
                            <Badge shape={Shape::Circle} color={c} placement={Placement::BottomRight} size={Size::Sm} aria_label={*aria} />
                        </Anchor>
                        <span class="text-xs text-gray-500">{*label}</span>
                    </div>
                }
            }) }
        </div>
    }
}

#[function_component(Example9)]
fn example9() -> Html {
    const PLACEMENTS: &[(Placement, &str)] = &[
        (Placement::TopRight, "top-right"),
        (Placement::TopLeft, "top-left"),
        (Placement::BottomRight, "bottom-right"),
        (Placement::BottomLeft, "bottom-left"),
    ];
    html! {
        <div class="flex items-center gap-8 flex-wrap justify-center">
            { for PLACEMENTS.iter().map(|(placement, label)| {
                let p = *placement;
                html! {
                    <div class="flex flex-col items-center gap-2">
                        <Anchor>
                            <span style={AVATAR}>{"FP"}</span>
                            <Badge shape={Shape::Circle} color={Color::Accent} placement={p} size={Size::Sm} aria_label={*label} />
                        </Anchor>
                        <span class="text-xs text-gray-500">{*label}</span>
                    </div>
                }
            }) }
        </div>
    }
}

#[function_component(Example10)]
fn example10() -> Html {
    html! {
        <div class="flex items-center gap-6 flex-wrap justify-center">
            <Anchor>
                <span style={AVATAR} aria-label="User, 5 unread">{ "FP" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Danger}
                    size={Size::Sm}
                    aria_label="5 unread messages"
                >
                    { "5" }
                </Badge>
            </Anchor>
            <Anchor>
                <span style={AVATAR} aria-label="User, new">{ "FP" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Danger}
                    size={Size::Sm}
                    aria_label="New"
                >
                    { "New" }
                </Badge>
            </Anchor>
            <Anchor>
                <span style={AVATAR} aria-label="User, 99+ notifications">{ "FP" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Danger}
                    size={Size::Sm}
                    aria_label="99+ notifications"
                >
                    { "99+" }
                </Badge>
            </Anchor>
        </div>
    }
}

#[function_component(Example11)]
fn example11() -> Html {
    html! {
        <div class="flex items-center gap-6 flex-wrap justify-center">
            <Anchor>
                <span style={ICON_BTN} aria-label="show 4 unread messages">{ "✉" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Danger}
                    size={Size::Sm}
                    aria_label="4 unread"
                >
                    { "4" }
                </Badge>
            </Anchor>
            <Anchor>
                <span style={ICON_BTN} aria-label="show 8 shared files">{ "📁" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Default}
                    size={Size::Sm}
                    aria_label="8 files"
                >
                    { "8" }
                </Badge>
            </Anchor>
            <Anchor>
                <span style={ICON_BTN} aria-label="show 2 confirmed events">{ "📅" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Success}
                    size={Size::Sm}
                    aria_label="2 events"
                >
                    { "2" }
                </Badge>
            </Anchor>
            <Anchor>
                <span style={ICON_BTN} aria-label="show 1 critical alert">{ "⚠" }</span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Warning}
                    size={Size::Sm}
                    aria_label="1 alert"
                >
                    { "1" }
                </Badge>
            </Anchor>
        </div>
    }
}

#[function_component(Example12)]
fn example12() -> Html {
    html! {
        <nav
            aria-label="Mail folders"
            style="background:#1e293b;border-radius:0.5rem;border:1px solid #334155;width:100%;max-width:280px;"
        >
            <ul style="list-style:none;margin:0;padding:0;" role="list">
                <li
                    aria-label="Inbox, 4 unread messages"
                    aria-current="page"
                    style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;background:#334155;border-left:3px solid #7c3aed;border-bottom:1px solid #334155;"
                >
                    <span style="color:#e2e8f0;font-size:0.875rem;">{ "Inbox" }</span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Accent}
                        variant={Variant::Soft}
                        size={Size::Sm}
                        style="position:static;transform:none;"
                        aria_label="4 unread"
                    >
                        { "4" }
                    </Badge>
                </li>
                <li
                    aria-label="Sent"
                    style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;border-bottom:1px solid #334155;"
                >
                    <span style="color:#e2e8f0;font-size:0.875rem;">{ "Sent" }</span>
                </li>
                <li
                    aria-label="Drafts, 12 unread messages"
                    style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;border-bottom:1px solid #334155;"
                >
                    <span style="color:#e2e8f0;font-size:0.875rem;">{ "Drafts" }</span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Default}
                        variant={Variant::Soft}
                        size={Size::Sm}
                        style="position:static;transform:none;"
                        aria_label="12 unread"
                    >
                        { "12" }
                    </Badge>
                </li>
                <li
                    aria-label="Spam, more than 99 unread messages"
                    style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;"
                >
                    <span style="color:#e2e8f0;font-size:0.875rem;">{ "Spam" }</span>
                    <Badge
                        shape={Shape::Circle}
                        color={Color::Default}
                        variant={Variant::Soft}
                        size={Size::Sm}
                        style="position:static;transform:none;"
                        aria_label="99+ unread"
                    >
                        { "99+" }
                    </Badge>
                </li>
            </ul>
        </nav>
    }
}

#[function_component(Example13)]
fn example13() -> Html {
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
    html! {
        <div
            class="w-full overflow-x-auto bg-gray-800 rounded-xl p-4"
            role="region"
            aria-label="Badge variant matrix"
        >
            <table class="w-full text-left border-collapse">
                <caption class="sr-only">{ "Badge style matrix, variants x colors" }</caption>
                <thead>
                    <tr>
                        <th scope="col" class="p-3 text-gray-400 font-medium">{ "" }</th>
                        { for COLORS.iter().map(|(_, name)| html! {
                            <th scope="col" class="p-3 text-gray-400 font-medium text-center">{*name}</th>
                        }) }
                    </tr>
                </thead>
                <tbody>
                    { for VARIANTS.iter().map(|(variant, vlabel)| {
                        let v = *variant;
                        html! {
                            <tr>
                                <th scope="row" class="p-3 text-gray-400 capitalize">{*vlabel}</th>
                                { for COLORS.iter().map(|(color, _)| {
                                    let c = *color;
                                    html! {
                                        <td class="p-3 text-center border-t border-white/10">
                                            <Anchor>
                                                <span style={AVATAR}>{"FP"}</span>
                                                <Badge shape={Shape::Circle} color={c} size={Size::Sm} variant={v} aria_label="5">{"5"}</Badge>
                                            </Anchor>
                                        </td>
                                    }
                                }) }
                            </tr>
                        }
                    }) }
                    <tr>
                        <th scope="row" class="p-3 text-gray-400">{ "dot" }</th>
                        { for COLORS.iter().map(|(color, clabel)| {
                            let c = *color;
                            html! {
                                <td class="p-3 text-center border-t border-white/10">
                                    <Anchor>
                                        <span style={AVATAR}>{"FP"}</span>
                                        <Badge shape={Shape::Circle} color={c} size={Size::Sm}
                                            placement={Placement::BottomRight} aria_label={*clabel} />
                                    </Anchor>
                                </td>
                            }
                        }) }
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[function_component(Example14)]
fn example14() -> Html {
    html! {
        <div class="flex items-center gap-8 flex-wrap justify-center">
            <Anchor>
                <span
                    style="display:inline-flex;width:40px;height:40px;border-radius:50%;background:#1e293b;border:2px solid #7c3aed;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;"
                    aria-label="Kate Wilson, 5 notifications"
                >
                    { "KW" }
                </span>
                <Badge
                    shape={Shape::Circle}
                    class="font-semibold"
                    color={Color::Accent}
                    size={Size::Sm}
                    variant={Variant::Soft}
                    aria_label="5 notifications"
                >
                    { "5" }
                </Badge>
            </Anchor>
            <Anchor>
                <span
                    style="display:inline-flex;width:40px;height:40px;border-radius:8px;background:#1e293b;border:1px solid #334155;align-items:center;justify-content:center;font-size:14px;font-weight:600;color:#e2e8f0;"
                    aria-label="TS user, 2 files"
                >
                    { "TS" }
                </span>
                <Badge
                    shape={Shape::Circle}
                    color={Color::Success}
                    size={Size::Md}
                    variant={Variant::Secondary}
                    aria_label="2 files"
                >
                    { "2" }
                </Badge>
            </Anchor>
        </div>
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-4 text-white">{ "Badges RS Yew Examples" }</h1>
            <section aria-labelledby="basic-heading" class="w-full max-w-6xl mb-12">
                <h2 id="basic-heading" class="text-xl font-semibold text-white mb-6">
                    { "Basic" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    { card("Basic Badge", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Placement, Size, Shape};
use yew::prelude::*;

#[function_component(BasicBadge)]
pub fn basic_badge() -> Html {
    html! {
        <div class="flex items-center gap-6">
            <Anchor>
                <span aria-label="FP, 5 notifications">{"FP"}</span>
                <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Sm}
                    aria_label="5 notifications">{"5"}</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="AB, new">{"AB"}</span>
                <Badge shape={Shape::Circle} color={Color::Accent} size={Size::Sm}
                    aria_label="New">{"New"}</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="CD, online">{"CD"}</span>
                <Badge shape={Shape::Circle} color={Color::Success}
                    placement={Placement::BottomRight}
                    size={Size::Sm} aria_label="Online" />
            </Anchor>
        </div>
    }
}"#, html! { <Example1 /> }) }
                    { card("Visibility Toggle", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use yew::prelude::*;

#[function_component(VisibilityToggle)]
pub fn visibility_toggle() -> Html {
    let visible = use_state(|| true);
    let toggle = {
        let visible = visible.clone();
        Callback::from(move |_: MouseEvent| {
            visible.set(!*visible)
        })
    };
    html! {
        <div class="flex flex-col items-center gap-4">
            <Anchor>
                <span aria-label={if *visible {
                    "User, 4 unread messages"
                } else { "User inbox" }}>
                    {"FP"}
                </span>
                if *visible {
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Sm}
                        aria_label="4 unread messages">{"4"}</Badge>
                }
            </Anchor>
            <button
                onclick={toggle}
                aria-pressed={if *visible { "true" } else { "false" }}>
                { if *visible { "Hide Badge" } else { "Show Badge" } }
            </button>
        </div>
    }
}"#, html! { <Example2 /> }) }
                    { card("Maximum Value", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use yew::prelude::*;

#[function_component(MaxValue)]
pub fn max_value() -> Html {
    html! {
        <div class="flex items-center gap-6">
            <div class="flex flex-col items-center gap-1">
                <Anchor>
                    <span aria-label="show 99">{"✉"}</span>
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Sm}
                        aria_label="99">{"99"}</Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{"99"}</span>
            </div>
            <div class="flex flex-col items-center gap-1">
                <Anchor>
                    <span aria-label="show more than 99">{"✉"}</span>
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Sm}
                        aria_label="99+">{"99+"}</Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{"100"}</span>
            </div>
            <div class="flex flex-col items-center gap-1">
                <Anchor>
                    <span aria-label="show more than 999">{"✉"}</span>
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Sm}
                        aria_label="999+">{"999+"}</Badge>
                </Anchor>
                <span class="text-xs text-gray-500">{"1000"}</span>
            </div>
        </div>
    }
}"#, html! { <Example3 /> }) }
                </div>
            </section>
            <section aria-labelledby="variants-heading" class="w-full max-w-6xl mb-12">
                <h2 id="variants-heading" class="text-xl font-semibold text-white mb-6">
                    { "Variants" }
                </h2>
                <div class="grid grid-cols-1 gap-8">
                    { card("Variant x Color Matrix", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size, Variant, Shape};
use yew::prelude::*;

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

#[function_component(VariantMatrix)]
pub fn variant_matrix() -> Html {
    html! {
        <div class="flex flex-col gap-4">
            { for VARIANTS.iter().map(|(variant, vlabel)| {
                let v = *variant;
                html! {
                    <div>
                        <p class="text-xs font-semibold capitalize">{*vlabel}</p>
                        <div class="flex items-center gap-4">
                            { for COLORS.iter().map(|(color, _)| {
                                let c = *color;
                                html! {
                                    <Anchor>
                                        <span>{"FP"}</span>
                                        <Badge shape={Shape::Circle} color={c} size={Size::Sm}
                                            variant={v} aria_label="5">{"5"}</Badge>
                                    </Anchor>
                                }
                            }) }
                        </div>
                    </div>
                }
            }) }
        </div>
    }
}"#, html! { <Example4 /> }) }
                </div>
            </section>
            <section aria-labelledby="sizes-heading" class="w-full max-w-6xl mb-12">
                <h2 id="sizes-heading" class="text-xl font-semibold text-white mb-6">
                    { "Sizes" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                    { card("Sizes", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use yew::prelude::*;

// Size::Sm → 16x16 px, 10 px font
// Size::Md → 20x20 px, 11 px font (default)
// Size::Lg → 24x24 px, 12 px font

#[function_component(Sizes)]
pub fn sizes() -> Html {
    html! {
        <div class="flex items-end gap-8">
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span aria-label="Small">{"FP"}</span>
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Sm}
                        aria_label="5">{"5"}</Badge>
                </Anchor>
                <span class="text-xs">{"Sm"}</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span aria-label="Medium">{"FP"}</span>
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Md}
                        aria_label="5">{"5"}</Badge>
                </Anchor>
                <span class="text-xs">{"Md"}</span>
            </div>
            <div class="flex flex-col items-center gap-2">
                <Anchor>
                    <span aria-label="Large">{"FP"}</span>
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Lg}
                        aria_label="5">{"5"}</Badge>
                </Anchor>
                <span class="text-xs">{"Lg"}</span>
            </div>
        </div>
    }
}"#, html! { <Example5 /> }) }
                    { card("Overlap Shapes", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use yew::prelude::*;

// Badge anchors to any shape: square, circle, icon.
#[function_component(Overlap)]
pub fn overlap() -> Html {
    html! {
        <div class="flex items-center gap-6">
            <div class="flex flex-col items-center gap-1">
                <Anchor>
                    <span style="width:32px;height:32px;
                        background:#3b82f6;display:inline-block;"
                        aria-label="Rectangle" />
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Sm}
                        aria_label="1">{"1"}</Badge>
                </Anchor>
                <span class="text-xs">{"rect"}</span>
            </div>
            <div class="flex flex-col items-center gap-1">
                <Anchor>
                    <span style="width:32px;height:32px;border-radius:50%;
                        background:#3b82f6;display:inline-block;"
                        aria-label="Circle" />
                    <Badge shape={Shape::Circle} color={Color::Danger} size={Size::Sm}
                        aria_label="1">{"1"}</Badge>
                </Anchor>
                <span class="text-xs">{"circle"}</span>
            </div>
        </div>
    }
}"#, html! { <Example6 /> }) }
                </div>
            </section>
            <section aria-labelledby="colors-heading" class="w-full max-w-6xl mb-12">
                <h2 id="colors-heading" class="text-xl font-semibold text-white mb-6">
                    { "Colors" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                    { card("Color - Dot Mode", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size, Shape};
use yew::prelude::*;

const COLORS: &[(Color, &str)] = &[
    (Color::Accent, "Accent"),
    (Color::Default, "Default"),
    (Color::Success, "Success"),
    (Color::Warning, "Warning"),
    (Color::Danger, "Danger"),
];

// Omit children for a compact dot indicator.
#[function_component(ColorDots)]
pub fn color_dots() -> Html {
    html! {
        <div class="flex items-center gap-4">
            { for COLORS.iter().map(|(color, label)| {
                let c = *color;
                html! {
                    <Anchor>
                        <span aria-label="User status">{"FP"}</span>
                        <Badge shape={Shape::Circle} color={c} size={Size::Sm}
                            aria_label={*label} />
                    </Anchor>
                }
            }) }
        </div>
    }
}"#, html! { <Example7 /> }) }
                    { card("Status Indicators", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Placement, Size, Shape};
use yew::prelude::*;

const STATUSES: &[(&str, Color, &str)] = &[
    ("Online",  Color::Success, "Online"),
    ("Away",    Color::Warning, "Away"),
    ("Busy",    Color::Danger,  "Busy"),
    ("Offline", Color::Default, "Offline"),
];

// Bottom-right placement for presence/status dots.
#[function_component(StatusDots)]
pub fn status_dots() -> Html {
    html! {
        <div class="flex items-center gap-4">
            { for STATUSES.iter().map(|(label, color, aria)| {
                let c = *color;
                html! {
                    <div class="flex flex-col items-center gap-1">
                        <Anchor>
                            <span aria-label="User">{"FP"}</span>
                            <Badge shape={Shape::Circle} color={c}
                                placement={Placement::BottomRight}
                                size={Size::Sm}
                                aria_label={*aria} />
                        </Anchor>
                        <span class="text-xs">{*label}</span>
                    </div>
                }
            }) }
        </div>
    }
}"#, html! { <Example8 /> }) }
                </div>
            </section>
            <section aria-labelledby="placement-heading" class="w-full max-w-6xl mb-12">
                <h2 id="placement-heading" class="text-xl font-semibold text-white mb-6">
                    { "Placements" }
                </h2>
                <div class="grid grid-cols-1 gap-8">
                    { card("All Four Corners", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Placement, Size, Shape};
use yew::prelude::*;

const PLACEMENTS: &[(Placement, &str)] = &[
    (Placement::TopRight,    "top-right"),
    (Placement::TopLeft,     "top-left"),
    (Placement::BottomRight, "bottom-right"),
    (Placement::BottomLeft,  "bottom-left"),
];

#[function_component(Placements)]
pub fn placements() -> Html {
    html! {
        <div class="flex items-center gap-6">
            { for PLACEMENTS.iter().map(|(placement, label)| {
                let p = *placement;
                html! {
                    <div class="flex flex-col items-center gap-1">
                        <Anchor>
                            <span aria-label="User">{"FP"}</span>
                            <Badge shape={Shape::Circle} color={Color::Accent}
                                placement={p} size={Size::Sm}
                                aria_label={*label} />
                        </Anchor>
                        <span class="text-xs">{*label}</span>
                    </div>
                }
            }) }
        </div>
    }
}"#, html! { <Example9 /> }) }
                </div>
            </section>
            <section aria-labelledby="content-heading" class="w-full max-w-6xl mb-12">
                <h2 id="content-heading" class="text-xl font-semibold text-white mb-6">
                    { "With Content" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                    { card("Text, Numbers, Icons", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size};
use yew::prelude::*;

// Badge accepts any children: text, numbers, "99+", SVG icons.
// When no children are given, it renders as a compact dot.
#[function_component(WithContent)]
pub fn with_content() -> Html {
    html! {
        <div class="flex items-center gap-4">
            <Anchor>
                <span aria-label="User, 5 unread">{"FP"}</span>
                <Badge color={Color::Danger} size={Size::Sm}
                    aria_label="5 unread messages">{"5"}</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="User, new">{"FP"}</span>
                <Badge color={Color::Danger} size={Size::Sm}
                    aria_label="New">{"New"}</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="User, 99+ notifications">{"FP"}</span>
                <Badge color={Color::Danger} size={Size::Sm}
                    aria_label="99+ notifications">{"99+"}</Badge>
            </Anchor>
        </div>
    }
}"#, html! { <Example10 /> }) }
                    { card("On Icon Buttons", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size};
use yew::prelude::*;

// Badge is not limited to avatars.
// It anchors to any element type.
#[function_component(IconBadges)]
pub fn icon_badges() -> Html {
    html! {
        <div class="flex items-center gap-6">
            <Anchor>
                <span aria-label="show 4 unread messages">{"✉"}</span>
                <Badge color={Color::Danger} size={Size::Sm}
                    aria_label="4 unread">{"4"}</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="show 8 shared files">{"📁"}</span>
                <Badge color={Color::Default} size={Size::Sm}
                    aria_label="8 files">{"8"}</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="show 2 confirmed events">{"📅"}</span>
                <Badge color={Color::Success} size={Size::Sm}
                    aria_label="2 events">{"2"}</Badge>
            </Anchor>
            <Anchor>
                <span aria-label="show 1 critical alert">{"⚠"}</span>
                <Badge color={Color::Warning} size={Size::Sm}
                    aria_label="1 alert">{"1"}</Badge>
            </Anchor>
        </div>
    }
}"#, html! { <Example11 /> }) }
                </div>
            </section>
            <section aria-labelledby="nav-heading" class="w-full max-w-6xl mb-12">
                <h2 id="nav-heading" class="text-xl font-semibold text-white mb-6">
                    { "Navigation Items" }
                </h2>
                <div class="grid grid-cols-1 gap-8">
                    { card("Badge on Nav List", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size, Variant};
use yew::prelude::*;

// Include the count in aria-label so screen readers
// announce it together with the folder name.
#[function_component(NavBadges)]
pub fn nav_badges() -> Html {
    html! {
        <nav aria-label="Mail folders">
            <ul style="list-style:none;padding:0;margin:0;">
                <li aria-label="Inbox, 4 unread messages"
                    aria-current="page">
                    <span>{"Inbox"}</span>
                    <Badge color={Color::Accent}
                        variant={Variant::Soft} size={Size::Sm}
                        style="position:static;transform:none;"
                        aria_label="4 unread">{"4"}</Badge>
                </li>
                <li aria-label="Sent"><span>{"Sent"}</span></li>
                <li aria-label="Drafts, 12 unread messages">
                    <span>{"Drafts"}</span>
                    <Badge color={Color::Default}
                        variant={Variant::Soft} size={Size::Sm}
                        style="position:static;transform:none;"
                        aria_label="12 unread">{"12"}</Badge>
                </li>
                <li aria-label="Spam, more than 99 unread messages">
                    <span>{"Spam"}</span>
                    <Badge color={Color::Default}
                        variant={Variant::Soft} size={Size::Sm}
                        style="position:static;transform:none;"
                        aria_label="99+ unread">{"99+"}</Badge>
                </li>
            </ul>
        </nav>
    }
}"#, html! { <Example12 /> }) }
                </div>
            </section>
            <section aria-labelledby="matrix-heading" class="w-full max-w-6xl mb-12">
                <h2 id="matrix-heading" class="text-xl font-semibold text-white mb-6">
                    { "Full Matrix" }
                </h2>
                <div class="grid grid-cols-1 gap-8">
                    { card("All Variants x Colors", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Placement, Size, Variant};
use yew::prelude::*;

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

#[function_component(MatrixTable)]
pub fn matrix_table() -> Html {
    html! {
        <table>
            <thead>
                <tr>
                    <th>{""}</th>
                    { for COLORS.iter().map(|(_, n)| html!{
                        <th>{*n}</th>
                    }) }
                </tr>
            </thead>
            <tbody>
                { for VARIANTS.iter().map(|(variant, vlabel)| {
                    let v = *variant;
                    html! {
                        <tr>
                            <th>{*vlabel}</th>
                            { for COLORS.iter().map(|(color, _)| {
                                let c = *color;
                                html! {
                                    <td>
                                        <Anchor>
                                            <span>{"FP"}</span>
                                            <Badge color={c} size={Size::Sm}
                                                variant={v} aria_label="5">{"5"}
                                            </Badge>
                                        </Anchor>
                                    </td>
                                }
                            }) }
                        </tr>
                    }
                }) }
                <tr>
                    <th>{"dot"}</th>
                    { for COLORS.iter().map(|(color, clabel)| {
                        let c = *color;
                        html! {
                            <td>
                                <Anchor>
                                    <span>{"FP"}</span>
                                    <Badge color={c} size={Size::Sm}
                                        placement={Placement::BottomRight}
                                        aria_label={*clabel} />
                                </Anchor>
                            </td>
                        }
                    }) }
                </tr>
            </tbody>
        </table>
    }
}"#, html! { <Example13 /> }) }
                </div>
            </section>
            <section aria-labelledby="custom-heading" class="w-full max-w-6xl mb-12">
                <h2 id="custom-heading" class="text-xl font-semibold text-white mb-6">
                    { "Customization" }
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-8">
                    { card("Custom CSS Classes", r#"use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size, Variant};
use yew::prelude::*;

// Use `class` to layer in your own Tailwind or
// CSS classes on top of the component defaults.
#[function_component(CustomStyle)]
pub fn custom_style() -> Html {
    html! {
        <div class="flex items-center gap-4">
            <Anchor>
                <span style="border:2px solid #7c3aed;"
                    aria-label="KW, 5 notifications">
                    {"KW"}
                </span>
                <Badge
                    class="font-semibold"
                    color={Color::Accent}
                    size={Size::Sm}
                    variant={Variant::Soft}
                    aria_label="5 notifications">
                    {"5"}
                </Badge>
            </Anchor>
            <Anchor>
                <span style="border-radius:8px;"
                    aria-label="TS, 2 files">
                    {"TS"}
                </span>
                <Badge
                    color={Color::Success}
                    size={Size::Md}
                    variant={Variant::Secondary}
                    aria_label="2 files">
                    {"2"}
                </Badge>
            </Anchor>
        </div>
    }
}"#, html! { <Example14 /> }) }
                    { card("BEM CSS Classes Reference", r#"/* Global CSS customization via BEM classes:    */
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
}"#, html! {
        <div class="flex items-center gap-4 flex-wrap justify-center">
            <Anchor>
                <span style={AVATAR} aria-label="Accent badge">{"FP"}</span>
                <Badge color={Color::Accent} size={Size::Sm} aria_label="5">{"5"}</Badge>
            </Anchor>
            <Anchor>
                <span style={AVATAR} aria-label="Soft success badge">{"AB"}</span>
                <Badge color={Color::Success} size={Size::Sm} variant={Variant::Soft} aria_label="3">{"3"}</Badge>
            </Anchor>
            <Anchor>
                <span style={AVATAR} aria-label="Secondary danger badge">{"CD"}</span>
                <Badge color={Color::Danger} size={Size::Sm} variant={Variant::Secondary} aria_label="9">{"9"}</Badge>
            </Anchor>
        </div>
    }) }
                </div>
            </section>
        </div>
    }
}
