use crate::config::Config;
use gtk4 as gtk;
use gtk::gdk;
use gtk::prelude::*;
use gtk4_layer_shell::LayerShell;
use gtk4_session_lock::Instance as SessionLock;
use std::sync::mpsc;

pub struct LockWindow {
    window: gtk::ApplicationWindow,
    username_entry: gtk::Entry,
    password_entry: gtk::Entry,
    title_label: gtk::Label,
    time_label: gtk::Label,
    unlock_button: gtk::Button,
    error_label: gtk::Label,
    background_picture: gtk::Picture,
}

impl LockWindow {
    pub fn from_builder(builder: &gtk::Builder) -> Self {
        Self {
            window: builder.object("lock_window").expect("lock_window"),
            username_entry: builder.object("username_entry").expect("username_entry"),
            password_entry: builder.object("password_entry").expect("password_entry"),
            title_label: builder.object("title_label").expect("title_label"),
            time_label: builder.object("time_label").expect("time_label"),
            unlock_button: builder.object("unlock_button").expect("unlock_button"),
            error_label: builder.object("error_label").expect("error_label"),
            background_picture: builder.object("background_picture").expect("background_picture"),
        }
    }

    pub fn apply_config(&self, config: &Config) {
        if let Some(ref image) = config.background.image {
            self.background_picture.set_filename(Some(image));
        }

        let font_family = config.font_family.as_deref().unwrap_or("Sans");
        let font_size = config.font_size.unwrap_or(14);
        apply_font(
            &self.title_label,
            config.header.font_family.as_deref().unwrap_or(font_family),
            config.header.font_size.unwrap_or(font_size),
        );
        apply_font(
            &self.time_label,
            config.subtitle.font_family.as_deref().unwrap_or(font_family),
            config.subtitle.font_size.unwrap_or(font_size),
        );

        if let Some(ref text) = config.header.text {
            self.title_label.set_text(text);
        }
        if let Some(ref text) = config.subtitle.text {
            self.time_label.set_text(text);
        }
    }

    pub fn init_layer_shell(&self) {
        LayerShell::init_layer_shell(&self.window);
        self.window.set_layer(gtk4_layer_shell::Layer::Overlay);
        self.window.set_anchor(gtk4_layer_shell::Edge::Top, true);
        self.window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
        self.window.set_anchor(gtk4_layer_shell::Edge::Left, true);
        self.window.set_anchor(gtk4_layer_shell::Edge::Right, true);
    }

    pub fn start_clock(&self, config: &Config) {
        let title_label = self.title_label.clone();
        let time_label = self.time_label.clone();
        let has_custom_title = config.header.text.is_some();
        let has_custom_subtitle = config.subtitle.text.is_some();

        let update = move || {
            let now = glib::DateTime::now_local().unwrap();
            if !has_custom_title {
                title_label.set_text(&now.format("%A, %B %e").unwrap());
            }
            if !has_custom_subtitle {
                time_label.set_text(&now.format("%H:%M:%S").unwrap());
            }
            glib::ControlFlow::Continue
        };
        update();
        glib::timeout_add_seconds_local(1, update);
    }

    pub fn setup_auth(&self, lock: SessionLock) {
        let username = std::env::var("USER").unwrap_or_default();
        self.username_entry.set_text(&format!("\u{f007}  {username}"));
        self.username_entry.set_focusable(false);
        self.password_entry.grab_focus();

        let (tx, rx) = mpsc::channel::<bool>();

        let poll_entry = self.password_entry.clone();
        let poll_lock = lock.clone();
        let poll_button = self.unlock_button.clone();
        let poll_error = self.error_label.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
            while let Ok(success) = rx.try_recv() {
                if success {
                    log::info!("Authentication successful, unlocking");
                    poll_lock.unlock();
                } else {
                    log::error!("Authentication failed");
                    poll_entry.set_text("");
                    poll_entry.set_sensitive(true);
                    poll_button.set_sensitive(true);
                    poll_error.set_label("Invalid password");
                }
            }
            glib::ControlFlow::Continue
        });

        let entry_c = self.password_entry.clone();
        let user_c = username.clone();
        let button_c = self.unlock_button.clone();
        let error_c = self.error_label.clone();
        let tx_c = tx.clone();
        self.unlock_button.connect_clicked(move |_| {
            authenticate(&entry_c, &button_c, &error_c, &user_c, &tx_c);
        });

        let entry_a = self.password_entry.clone();
        let user_a = username;
        let button_a = self.unlock_button.clone();
        let error_a = self.error_label.clone();
        let tx_a = tx;
        entry_a.clone().connect_activate(move |_| {
            authenticate(&entry_a, &button_a, &error_a, &user_a, &tx_a);
        });
    }

    pub fn show(&self, lock: &SessionLock, app: &gtk::Application, monitor: &gdk::Monitor) {
        lock.assign_window_to_monitor(&self.window, monitor);
        self.window.set_application(Some(app));
        self.window.set_visible(true);
    }
}

fn authenticate(
    entry: &gtk::Entry,
    button: &gtk::Button,
    error: &gtk::Label,
    user: &str,
    tx: &mpsc::Sender<bool>,
) {
    button.set_sensitive(false);
    entry.set_sensitive(false);
    error.set_label("");
    let password = entry.text().to_string();
    let user = user.to_string();
    let tx = tx.clone();
    std::thread::spawn(move || {
        tx.send(crate::auth::verify_credentials(&user, &password).is_ok()).ok();
    });
}

fn apply_font(label: &gtk::Label, family: &str, size: u8) {
    let mut desc = pango::FontDescription::new();
    desc.set_family(family);
    desc.set_absolute_size((size as f64) * pango::SCALE as f64);
    let attrs = pango::AttrList::new();
    attrs.insert(pango::AttrFontDesc::new(&desc));
    label.set_attributes(Some(&attrs));
}
