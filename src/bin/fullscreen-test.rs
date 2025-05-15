use bevy::{prelude::*, window::WindowMode};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current),
            ..Default::default()
        }),
        ..Default::default()
    }));

    #[cfg(feature = "inspect")]
    app.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin {
        enable_multipass_for_primary_context: true,
    })
    .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());

    app.run();
}
