use std::io::Cursor;
#[cfg(feature = "debug")]
use std::time::Duration;

#[cfg(feature = "debug")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowTheme};
use bevy::winit::WinitWindows;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use winit::window::Icon;

const WINDOW_TITLE: &str = "3D Cellular Automata";
const INITIAL_RESOLUTION_X: u16 = 1280;
const INITIAL_RESOLUTION_Y: u16 = 720;

fn main() {
    let mut app = App::new();

    // Set window properties
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: WINDOW_TITLE.into(),
            resolution: (INITIAL_RESOLUTION_X, INITIAL_RESOLUTION_Y).into(),
            present_mode: PresentMode::AutoVsync,
            window_theme: Some(WindowTheme::Dark),
            focused: true,
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: true,
                close: true,
                minimize: true,
            },
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "debug")]
    add_debug_plugins(&mut app);

    app.add_systems(Startup, set_window_icon);
    app.run();
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    info!("Setting the window icon");
    let primary_entity = primary_window.single();

    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };

    let icon_buf = Cursor::new(include_bytes!("../icons/C-Icon-256.png"));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

#[cfg(feature = "debug")]
fn add_debug_plugins(app: &mut App) {
    info!("Debug mode plugins enabled");
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(LogDiagnosticsPlugin {
        wait_duration: Duration::from_secs(5),
        ..default()
    });
    app.add_plugins(WorldInspectorPlugin::new());
}
