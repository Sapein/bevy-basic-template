use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{ project-name }}".into(),
                name: Some("{{ project-name }}".into()),

                // This allows i3wm to force it into floating.
                resizable: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
