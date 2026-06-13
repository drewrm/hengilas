mod config;
mod auth;
mod css;
mod lock_window;

use gtk4 as gtk;
use gtk::gdk;
use gtk::prelude::*;
use gtk4_session_lock::Instance as SessionLock;
use std::sync::Arc;
use config::Config;

struct LockState {
    pub app: gtk4::Application,
    pub lock: SessionLock,
}

impl LockState {
    fn new(app: gtk4::Application) -> Self {
        let lock = SessionLock::new();
        Self { app, lock }
    }
}

fn on_monitor(state: &Arc<LockState>, monitor: &gdk::Monitor) {
    log::debug!("Monitor changed: {:?}", monitor);
    let config = Config::load();
    let display = gdk::Display::default().expect("Failed to get default display");
    css::setup(&config, &display);
    let builder = gtk::Builder::from_resource("/com/example/locker/ui/lock_window.ui");
    let lock_ui = lock_window::LockWindow::from_builder(&builder);
    lock_ui.apply_config(&config);
    lock_ui.init_layer_shell();
    lock_ui.start_clock(&config);
    lock_ui.setup_auth(state.lock.clone());
    lock_ui.show(&state.lock, &state.app, monitor);
}

fn activate(app: &gtk::Application) {
    let state = Arc::new(LockState::new(app.clone()));

    let state_failed = Arc::clone(&state);
    let state_unlocked = Arc::clone(&state);
    let state_monitor = Arc::clone(&state);

    state.lock.connect_locked(move |_| {
        log::info!("Locked");
    });

    state.lock.connect_failed(move |_| {
        log::error!("Failed");
        state_failed.app.quit();
    });

    state.lock.connect_unlocked(move |_| {
        log::info!("Unlocked");
        state_unlocked.app.quit();
    });

    state.lock.connect_monitor(move |_, monitor| {
        log::debug!("Monitor changed: {:?}", monitor);
        on_monitor(&state_monitor, monitor);
    });

    log::info!("Locking");
    state.lock.lock();
}

fn main() {
    env_logger::init();
    gio::resources_register_include!("resources.rs").expect("Failed to register resources");

    let application = gtk::Application::new(
        Some("com.example.hello-gtk"),
        Default::default(),
    );

    application.connect_activate(activate);
    application.run();
}
