use crate::{save_theme_value, Theme};
use gtk::prelude::GtkSettingsExt;
use gtk::Settings;
use tauri::{command, AppHandle, Runtime};

#[command]
pub fn set_theme<R: Runtime>(app: AppHandle<R>, theme: Theme) -> Result<(), &'static str> {
    save_theme_value(&app, theme);
    match theme {
        Theme::Auto => {}
        Theme::Light => {
            update_theme(app, false);
        }
        Theme::Dark => {
            update_theme(app, true);
        }
    }
    Ok(())
}

fn update_theme<R: Runtime>(app: AppHandle<R>, dark_theme: bool) {
    let _ = app.run_on_main_thread(move || {
        if let Some(settings) = Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(dark_theme);
        }
    });
}
