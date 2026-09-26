# 🌱 Badges RS Leptos Usage

Adding Badges RS to your project is simple:

1. Make sure your project is set up with **Leptos**. Follow their [Getting Started Guide](https://book.leptos.dev/getting_started/index.html) for setup instructions.

1. Add the Badges RS component to your dependencies:

   ```sh
   cargo add badges-rs --features=lep
   ```

1. Import and use the `Badge`, `Anchor`, and `Label` components.

## 🛠️ Usage

### Basic Badge with Count

```rust
use badges_rs::leptos::{Badge, Anchor};
use badges_rs::Color;
use leptos::prelude::*;

#[component]
pub fn MyBadge() -> impl IntoView {
    view! {
        <Anchor>
            <span aria-label="Inbox, 5 unread messages">"📬"</span>
            <Badge color=Color::Danger aria_label="5 unread messages">"5"</Badge>
        </Anchor>
    }
}
```

### Status Dot (No Children)

```rust
use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Placement};
use leptos::prelude::*;

#[component]
pub fn OnlineDot() -> impl IntoView {
    view! {
        <Anchor>
            <span>"👤"</span>
            <Badge color=Color::Success placement=Placement::BottomRight aria_label="Online" />
        </Anchor>
    }
}
```

### Variants

```rust
use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Variant};
use leptos::prelude::*;

#[component]
pub fn BadgeVariants() -> impl IntoView {
    view! {
        <Anchor><span>"A"</span><Badge color=Color::Accent variant=Variant::Primary>"5"</Badge></Anchor>
        <Anchor><span>"B"</span><Badge color=Color::Accent variant=Variant::Secondary>"5"</Badge></Anchor>
        <Anchor><span>"C"</span><Badge color=Color::Accent variant=Variant::Soft>"5"</Badge></Anchor>
    }
}
```

### Sizes

```rust
use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Size};
use leptos::prelude::*;

#[component]
pub fn BadgeSizes() -> impl IntoView {
    view! {
        <Anchor><span>"S"</span><Badge color=Color::Danger size=Size::Sm>"5"</Badge></Anchor>
        <Anchor><span>"M"</span><Badge color=Color::Danger size=Size::Md>"5"</Badge></Anchor>
        <Anchor><span>"L"</span><Badge color=Color::Danger size=Size::Lg>"5"</Badge></Anchor>
    }
}
```

### Placements

```rust
use badges_rs::leptos::{Badge, Anchor};
use badges_rs::{Color, Placement};
use leptos::prelude::*;

#[component]
pub fn BadgePlacements() -> impl IntoView {
    view! {
        <Anchor><span>"TR"</span><Badge color=Color::Accent placement=Placement::TopRight aria_label="top-right" /></Anchor>
        <Anchor><span>"TL"</span><Badge color=Color::Accent placement=Placement::TopLeft aria_label="top-left" /></Anchor>
        <Anchor><span>"BR"</span><Badge color=Color::Accent placement=Placement::BottomRight aria_label="bottom-right" /></Anchor>
        <Anchor><span>"BL"</span><Badge color=Color::Accent placement=Placement::BottomLeft aria_label="bottom-left" /></Anchor>
    }
}
```

## 🔧 Props

### `Badge`

| Property      | Type                 | Description                                    | Default               |
| ------------- | -------------------- | ---------------------------------------------- | --------------------- |
| `children`    | `Option<ChildrenFn>` | Content inside the badge. `None` for dot mode. | `None`                |
| `color`       | `Color`              | Color theme.                                   | `Color::Default`      |
| `variant`     | `Variant`            | Visual style.                                  | `Variant::Primary`    |
| `size`        | `Size`               | Badge size.                                    | `Size::Md`            |
| `placement`   | `Placement`          | Corner placement relative to anchor.           | `Placement::TopRight` |
| `class`       | `&'static str`       | Extra CSS classes.                             | `""`                  |
| `style`       | `&'static str`       | Extra inline CSS.                              | `""`                  |
| `id`          | `&'static str`       | `id` attribute.                                | `""`                  |
| `aria_label`  | `&'static str`       | Accessible name for screen readers.            | `"Badge"`             |
| `data_testid` | `&'static str`       | Testing attribute.                             | `""`                  |

### `Anchor`

| Property   | Type           | Description               | Default |
| ---------- | -------------- | ------------------------- | ------- |
| `children` | `Children`     | Target element + `Badge`. | -       |
| `class`    | `&'static str` | Extra CSS classes.        | `""`    |
| `style`    | `&'static str` | Extra inline CSS.         | `""`    |
| `id`       | `&'static str` | `id` attribute.           | `""`    |

### `Label`

| Property   | Type           | Description        | Default |
| ---------- | -------------- | ------------------ | ------- |
| `children` | `Children`     | Label content.     | -       |
| `class`    | `&'static str` | Extra CSS classes. | `""`    |
| `style`    | `&'static str` | Extra inline CSS.  | `""`    |
| `id`       | `&'static str` | `id` attribute.    | `""`    |

## 💡 Notes

- `Badge` must be placed inside a `Anchor` for correct absolute positioning.
- Omit `children` (or pass `None`) to render as a compact dot status indicator.
- Use `Color` and `Variant` props to match your design system.
