use bevy::prelude::*;
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Spellstorm".into(),
                    resizable: false, //keep to false, wasm incompatability
                    ..default()
                }),
                ..default()
            })
            .build(),
    );
    app.add_systems(Startup, camera_system);
    app.add_systems(Startup, circle_mesh);
    app.run();
}
/*

GAME SYSTEMS
TODO: make modules for each isolated system
camera
player
enemy

*/
fn camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn circle_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle = meshes.add(Circle::new(50.0));
    let color = Color::hsl(0., 0.95, 0.7);

    commands.spawn((Mesh2d(circle), MeshMaterial2d(materials.add(color))));
}
