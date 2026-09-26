# 🧬 Badges RS Dioxus Usage

Adding Badges RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Follow their [Getting Started Guide](https://dioxuslabs.com/learn/0.7/getting_started) for setup instructions.

1. Add the Badges RS component to your dependencies:

   ```sh
   cargo add badges-rs --features=dio
   ```

1. Import and use the `Badge`, `Anchor`, and `Label` components.

## 🛠️ Usage

### Basic Badge with Count

```rust
use badges_rs::dioxus::{Badge, Anchor};
use badges_rs::Color;
use dioxus::prelude::*;

fn MyBadge() -> Element {
    rsx! {
        Anchor {
            span { aria_label: "Inbox, 5 unread messages", "📬" }
            Badge { color: Color::Danger, aria_label: "5 unread messages", "5" }
        }
    }
}
```

### Status Dot (No Children)

```rust
use badges_rs::dioxus::{Badge, Anchor};
use badges_rs::{Color, Placement};
use dioxus::prelude::*;

fn OnlineDot() -> Element {
    rsx! {
        Anchor {
            span { "👤" }
            Badge { color: Color::Success, placement: Placement::BottomRight, aria_label: "Online" }
        }
    }
}
```

### Variants

```rust
use badges_rs::dioxus::{Badge, Anchor};
use badges_rs::{Color, Variant};
use dioxus::prelude::*;

fn BadgeVariants() -> Element {
    rsx! {
        Anchor { span { "A" } Badge { color: Color::Accent, variant: Variant::Primary, "5" } }
        Anchor { span { "B" } Badge { color: Color::Accent, variant: Variant::Secondary, "5" } }
        Anchor { span { "C" } Badge { color: Color::Accent, variant: Variant::Soft, "5" } }
    }
}
```

### Sizes

```rust
use badges_rs::dioxus::{Badge, Anchor};
use badges_rs::{Color, Size};
use dioxus::prelude::*;

fn BadgeSizes() -> Element {
    rsx! {
        Anchor { span { "S" } Badge { color: Color::Danger, size: Size::Sm, "5" } }
        Anchor { span { "M" } Badge { color: Color::Danger, size: Size::Md, "5" } }
        Anchor { span { "L" } Badge { color: Color::Danger, size: Size::Lg, "5" } }
    }
}
```

### Placements

```rust
use badges_rs::dioxus::{Badge, Anchor};
use badges_rs::{Color, Placement};
use dioxus::prelude::*;

fn BadgePlacements() -> Element {
    rsx! {
        Anchor { span { "TR" } Badge { color: Color::Accent, placement: Placement::TopRight, aria_label: "top-right" } }
        Anchor { span { "TL" } Badge { color: Color::Accent, placement: Placement::TopLeft, aria_label: "top-left" } }
        Anchor { span { "BR" } Badge { color: Color::Accent, placement: Placement::BottomRight, aria_label: "bottom-right" } }
        Anchor { span { "BL" } Badge { color: Color::Accent, placement: Placement::BottomLeft, aria_label: "bottom-left" } }
    }
}
```

## 🔧 Props

### `Badge`

| Property      | Type              | Description                                    | Default               |
| ------------- | ----------------- | ---------------------------------------------- | --------------------- |
| `children`    | `Option<Element>` | Content inside the badge. `None` for dot mode. | `None`                |
| `color`       | `Color`           | Color theme.                                   | `Color::Default`      |
| `variant`     | `Variant`         | Visual style.                                  | `Variant::Primary`    |
| `size`        | `Size`            | Badge size.                                    | `Size::Md`            |
| `placement`   | `Placement`       | Corner placement relative to anchor.           | `Placement::TopRight` |
| `class`       | `&'static str`    | Extra CSS classes.                             | `""`                  |
| `style`       | `&'static str`    | Extra inline CSS.                              | `""`                  |
| `id`          | `&'static str`    | `id` attribute.                                | `""`                  |
| `aria_label`  | `&'static str`    | Accessible name for screen readers.            | `"Badge"`             |
| `data_testid` | `&'static str`    | Testing attribute.                             | `""`                  |

### `Anchor`

| Property   | Type           | Description               | Default |
| ---------- | -------------- | ------------------------- | ------- |
| `children` | `Element`      | Target element + `Badge`. | -       |
| `class`    | `&'static str` | Extra CSS classes.        | `""`    |
| `style`    | `&'static str` | Extra inline CSS.         | `""`    |
| `id`       | `&'static str` | `id` attribute.           | `""`    |

### `Label`

| Property   | Type           | Description        | Default |
| ---------- | -------------- | ------------------ | ------- |
| `children` | `Element`      | Label content.     | -       |
| `class`    | `&'static str` | Extra CSS classes. | `""`    |
| `style`    | `&'static str` | Extra inline CSS.  | `""`    |
| `id`       | `&'static str` | `id` attribute.    | `""`    |

## 💡 Notes

- `Badge` must be placed inside a `Anchor` for correct absolute positioning.
- Pass `children: None` (or omit children) to render a dot indicator.
- Use `Color` and `Variant` props to match your design system.
