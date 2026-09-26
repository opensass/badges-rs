# Y Badges RS Yew Usage

Adding Badges RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Badges RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add badges-rs --features=yew
   ```

1. Import the `Badge`, `Anchor`, and `Label` components into your Yew component and start using them in your app.

## 🛠️ Usage

### Basic Badge with Count

```rust
use badges_rs::yew::{Badge, Anchor};
use badges_rs::Color;
use yew::prelude::*;

#[function_component(MyBadge)]
pub fn my_badge() -> Html {
    html! {
        <Anchor>
            <span aria-label="Inbox, 5 unread messages">{"📬"}</span>
            <Badge color={Color::Danger} aria_label="5 unread messages">{"5"}</Badge>
        </Anchor>
    }
}
```

### Status Dot (No Children)

```rust
use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Placement};
use yew::prelude::*;

#[function_component(OnlineDot)]
pub fn online_dot() -> Html {
    html! {
        <Anchor>
            <span>{"👤"}</span>
            <Badge color={Color::Success} placement={Placement::BottomRight} aria_label="Online" />
        </Anchor>
    }
}
```

### Variants

```rust
use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Variant};
use yew::prelude::*;

#[function_component(BadgeVariants)]
pub fn badge_variants() -> Html {
    html! {
        <>
            <Anchor>
                <span>{"A"}</span>
                <Badge color={Color::Accent} variant={Variant::Primary}>{"5"}</Badge>
            </Anchor>
            <Anchor>
                <span>{"B"}</span>
                <Badge color={Color::Accent} variant={Variant::Secondary}>{"5"}</Badge>
            </Anchor>
            <Anchor>
                <span>{"C"}</span>
                <Badge color={Color::Accent} variant={Variant::Soft}>{"5"}</Badge>
            </Anchor>
        </>
    }
}
```

### Sizes

```rust
use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Size};
use yew::prelude::*;

#[function_component(BadgeSizes)]
pub fn badge_sizes() -> Html {
    html! {
        <>
            <Anchor>
                <span>{"S"}</span>
                <Badge color={Color::Danger} size={Size::Sm}>{"5"}</Badge>
            </Anchor>
            <Anchor>
                <span>{"M"}</span>
                <Badge color={Color::Danger} size={Size::Md}>{"5"}</Badge>
            </Anchor>
            <Anchor>
                <span>{"L"}</span>
                <Badge color={Color::Danger} size={Size::Lg}>{"5"}</Badge>
            </Anchor>
        </>
    }
}
```

### Placements

```rust
use badges_rs::yew::{Badge, Anchor};
use badges_rs::{Color, Placement};
use yew::prelude::*;

#[function_component(BadgePlacements)]
pub fn badge_placements() -> Html {
    html! {
        <>
            <Anchor>
                <span>{"TR"}</span>
                <Badge color={Color::Accent} placement={Placement::TopRight} aria_label="top-right" />
            </Anchor>
            <Anchor>
                <span>{"TL"}</span>
                <Badge color={Color::Accent} placement={Placement::TopLeft} aria_label="top-left" />
            </Anchor>
            <Anchor>
                <span>{"BR"}</span>
                <Badge color={Color::Accent} placement={Placement::BottomRight} aria_label="bottom-right" />
            </Anchor>
            <Anchor>
                <span>{"BL"}</span>
                <Badge color={Color::Accent} placement={Placement::BottomLeft} aria_label="bottom-left" />
            </Anchor>
        </>
    }
}
```

## 🔧 Props

### `Badge`

| Property      | Type           | Description                                  | Default               |
| ------------- | -------------- | -------------------------------------------- | --------------------- |
| `children`    | `Children`     | Content inside the badge. Omit for dot mode. | `""`                  |
| `color`       | `Color`        | Color theme.                                 | `Color::Default`      |
| `variant`     | `Variant`      | Visual style.                                | `Variant::Primary`    |
| `size`        | `Size`         | Badge size.                                  | `Size::Md`            |
| `placement`   | `Placement`    | Corner placement relative to anchor.         | `Placement::TopRight` |
| `class`       | `&'static str` | Extra CSS classes on root element.           | `""`                  |
| `style`       | `&'static str` | Extra inline CSS on root element.            | `""`                  |
| `id`          | `&'static str` | `id` attribute on root element.              | `""`                  |
| `aria_label`  | `&'static str` | Accessible name for screen readers.          | `"Badge"`             |
| `data_testid` | `&'static str` | Testing attribute.                           | `""`                  |

### `Anchor`

| Property   | Type           | Description                       | Default |
| ---------- | -------------- | --------------------------------- | ------- |
| `children` | `Children`     | Target element + `Badge`.         | -       |
| `class`    | `&'static str` | Extra CSS classes on the wrapper. | `""`    |
| `style`    | `&'static str` | Extra inline CSS on the wrapper.  | `""`    |
| `id`       | `&'static str` | `id` attribute.                   | `""`    |

### `Label`

| Property   | Type           | Description        | Default |
| ---------- | -------------- | ------------------ | ------- |
| `children` | `Children`     | Label content.     | -       |
| `class`    | `&'static str` | Extra CSS classes. | `""`    |
| `style`    | `&'static str` | Extra inline CSS.  | `""`    |
| `id`       | `&'static str` | `id` attribute.    | `""`    |

## 💡 Notes

- `Badge` must be placed inside a `Anchor` for correct absolute positioning.
- When `children` is empty the badge renders as a dot status indicator - no text, no padding.
- Use `Color` and `Variant` props to match your design system.
