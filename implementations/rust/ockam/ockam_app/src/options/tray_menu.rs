use tauri::menu::{IconMenuItemBuilder, MenuBuilder, MenuEvent, MenuItemBuilder, NativeIcon};
use tauri::{AppHandle, Manager, Runtime, State};
use tracing::error;

use crate::app::AppState;
use crate::options::reset;

pub const RESET_MENU_ID: &str = "reset";
pub const QUIT_MENU_ID: &str = "quit";
pub const ERROR_MENU_ID: &str = "error";

pub(crate) async fn build_options_section<'a, R: Runtime, M: Manager<R>>(
    app_handle: &AppHandle<R>,
    mut builder: MenuBuilder<'a, R, M>,
) -> MenuBuilder<'a, R, M> {
    let app_state: State<AppState> = app_handle.state();

    builder = builder.items(&[
        &MenuItemBuilder::with_id(RESET_MENU_ID, "Reset")
            .accelerator("cmd+r")
            .build(app_handle),
        &MenuItemBuilder::with_id(QUIT_MENU_ID, "Quit Ockam")
            .accelerator("cmd+q")
            .build(app_handle),
    ]);

    match app_state.is_enrolled().await {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            builder = builder.item(
                &IconMenuItemBuilder::with_id(
                    ERROR_MENU_ID,
                    "The application state is corrupted, please re-enroll, reset or quit the application",
                )
                    .native_icon(NativeIcon::Caution).build(app_handle)
            )
        }
    }

    builder
}

pub fn process_tray_menu_event<R: Runtime>(
    app: &AppHandle<R>,
    event: &MenuEvent,
) -> tauri::Result<()> {
    match event.id.as_ref() {
        RESET_MENU_ID => on_reset(app),
        QUIT_MENU_ID => on_quit(),
        _ => Ok(()),
    }
}

/// Event listener for the "Reset" menu item
/// Reset the persistent state
fn on_reset<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        reset(&app)
            .await
            .map_err(|e| error!(%e, "Failed to reset app"))
    });
    Ok(())
}

/// Event listener for the "Quit" menu item
/// Quit the application when the user wants to
fn on_quit() -> tauri::Result<()> {
    std::process::exit(0);
}
