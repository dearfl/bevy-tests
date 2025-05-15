use bevy::{prelude::*, window::WindowMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Fullscreen(
                    // this won't work, panics
                    MonitorSelection::Primary,
                    // this won't work either, panics
                    // MonitorSelection::Current,
                    // this works, though fullscreen is ignored on wayland
                    // MonitorSelection::Index(0),
                    VideoModeSelection::Current,
                ),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .run();
}
