use bevy::{prelude::*, window::Monitor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_systems(Update, log_monitor_update)
        .run();
}

fn log_monitor_update(
    monitors_added: Query<(Entity, &Monitor), Added<Monitor>>,
    mut monitors_removed: RemovedComponents<Monitor>,
) {
    // if a monitor was attached after the game has started,
    // the added monitor don't have any proper information
    // you can check the log here or inspector
    for (entity, monitor) in monitors_added {
        println!("Added: {} {:?}", entity, monitor);
    }
    for monitor in monitors_removed.read() {
        println!("Removed: {:?}", monitor);
    }
}
