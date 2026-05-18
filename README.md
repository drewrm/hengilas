# hengilás

A GTK4 lockscreen for Wayland compositors using the `ext-session-lock` protocol.

![screenshot](screenshot.png)

## Features

- **PAM authentication** — uses the system PAM stack with the `login` service
- **Customizable** — configure fonts, colors, overlay, background image via TOML
- **Wayland native** — uses `gtk4-layer-shell` and `gtk4-session-lock`
- **Clock** — displays live date and time with configurable header/subtitle text

## Dependencies

- Rust edition 2024
- GTK4, libadwaita or adw-gtk3 theme
- A Wayland compositor with `ext-session-lock` support

## Build

```sh
cargo build --release
```

## Configuration

Place a config file at `~/.config/hengilas/config.toml`. See `sample.config.toml` for all options:

```toml
font_family = "Sans"
font_size = 14

[header]
font_family = "Sans"
font_size = 64
text = "Unlock Computer"

[subtitle]
font_family = "Sans"
font_size = 32
text = "Enter your password to unlock"

[focus]
color = "#4a90d9"
width = 2

[button]
background = "#4a90d9"
foreground = "#ffffff"

[overlay]
color = "#000000"
opacity = 0.5

[background]
image = "/usr/share/backgrounds/default.png"
```

All sections and fields are optional — missing values fall back to sensible defaults.

## Styling

The base CSS is loaded from `resources/styles/main.css`. Config values like overlay color, focus ring, and button colors override the base at runtime via an injected CSS provider.

## License

GPL-2.0-only
