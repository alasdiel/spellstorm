use bevy::prelude::*;
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Spellstorm".into(),
                    resizable: false, //change maybe
                    ..default()
                }),
                ..default()
            })
            .build()
    );
    app.add_systems(Startup, || {
        // Setup your game here
    });
    app.run();
}
