use crate::config::Config;
use gtk4 as gtk;
use gtk::gdk;

pub fn setup(config: &Config, display: &gdk::Display) {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/com/example/locker/styles/main.css");
    gtk::style_context_add_provider_for_display(display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    if let Some(ref color) = config.overlay.color {
        let hex = color.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let opacity = config.overlay.opacity.unwrap_or(0.65);
            let overlay_css = format!(".lock-screen-overlay {{ background-color: rgba({r}, {g}, {b}, {opacity}); }}");
            let overlay_provider = gtk::CssProvider::new();
            overlay_provider.load_from_string(&overlay_css);
            gtk::style_context_add_provider_for_display(display, &overlay_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1);
        }
    }

    let mut extra_css = String::new();

    if let Some(ref color) = config.focus.color {
        let width = config.focus.width.map(|w| format!("{w}px")).unwrap_or("2px".into());
        extra_css.push_str(&format!(
            "entry:focus-within, entry:focus {{ outline-color: {color}; outline-width: {width}; }}\n"
        ));
    }

    if let Some(ref bg) = config.button.background {
        extra_css.push_str(&format!("button.suggested-action {{ background: {bg}; }}\n"));
    }
    if let Some(ref fg) = config.button.foreground {
        extra_css.push_str(&format!("button.suggested-action {{ color: {fg}; }}\n"));
    }

    if !extra_css.is_empty() {
        let extra_provider = gtk::CssProvider::new();
        extra_provider.load_from_string(&extra_css);
        gtk::style_context_add_provider_for_display(display, &extra_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1);
    }
}
