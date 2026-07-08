// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(all(target_env = "gnu", not(target_os = "windows")))]
fn limit_malloc_arenas(arenas: std::os::raw::c_int) {
    // Each glibc malloc arena keeps its own dirtied heap pages; with the
    // default of 8 arenas per core, idle applets waste several MiB each.
    const M_ARENA_MAX: std::os::raw::c_int = -8;
    unsafe extern "C" {
        fn mallopt(param: std::os::raw::c_int, value: std::os::raw::c_int) -> std::os::raw::c_int;
    }
    unsafe {
        mallopt(M_ARENA_MAX, arenas);
    }
}

fn main() -> cosmic::iced::Result {
    // The iced daemon runner spawns a default tokio runtime with one worker
    // per core; applets are idle most of the time and don't need that.
    if std::env::var_os("TOKIO_WORKER_THREADS").is_none() {
        // Safe: no other threads exist yet.
        unsafe { std::env::set_var("TOKIO_WORKER_THREADS", "2") };
    }
    #[cfg(all(target_env = "gnu", not(target_os = "windows")))]
    limit_malloc_arenas(2);

    tracing_subscriber::fmt().with_env_filter("warn").init();
    let _ = tracing_log::LogTracer::init();

    let Some(applet) = std::env::args().next() else {
        return Ok(());
    };

    let start = applet.rfind('/').map_or(0, |v| v + 1);
    let cmd = &applet.as_str()[start..];

    tracing::info!("Starting `{cmd}` with version {VERSION}");

    match cmd {
        "cosmic-app-list" => cosmic_app_list::run(),
        "cosmic-applet-a11y" => cosmic_applet_a11y::run(),
        "cosmic-applet-audio" => cosmic_applet_audio::run(),
        "cosmic-applet-battery" => cosmic_applet_battery::run(),
        "cosmic-applet-bluetooth" => cosmic_applet_bluetooth::run(),
        "cosmic-applet-minimize" => cosmic_applet_minimize::run(),
        "cosmic-applet-network" => cosmic_applet_network::run(),
        "cosmic-applet-notifications" => cosmic_applet_notifications::run(),
        "cosmic-applet-power" => cosmic_applet_power::run(),
        "cosmic-applet-status-area" => cosmic_applet_status_area::run(),
        "cosmic-applet-tiling" => cosmic_applet_tiling::run(),
        "cosmic-applet-time" => cosmic_applet_time::run(),
        "cosmic-applet-workspaces" => cosmic_applet_workspaces::run(),
        "cosmic-applet-input-sources" => cosmic_applet_input_sources::run(),
        "cosmic-panel-button" => cosmic_panel_button::run(),
        _ => Ok(()),
    }
}
