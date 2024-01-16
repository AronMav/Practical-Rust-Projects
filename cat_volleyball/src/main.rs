use bevy::prelude::*;

const ARENA_WIDTH: f32 = 200.0;
const ARENA_HEIGHT: f32 = 200.0;

fn setup(mut commands: Commands,){
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(
            ARENA_WIDTH/2.0,
            ARENA_HEIGHT/2.0,1.0),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cat Volleyball".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb (0.0, 0.0, 13.0)))
        .add_startup_system(setup)
        .run();
}
