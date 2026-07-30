mod player;
mod game;
mod map;
mod physics;
mod render;
mod ui;
mod common;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, player::spawn_player, player::lock_cursor))
        .add_systems(Update, (update_text,
            player::move_player,
            player::look_player,
            player::look_camera,
            player::handle_grounding,
            setup_level_collision,
            quit
        ))
        .add_systems(Startup, map::spawn_map)
        .run();
}

fn quit(keyboard: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((
        WorldAssetRoot(asset_server.load("levels/DebugGym.glb#Scene0")),
        Transform::default(),
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            range: 20.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(
            Quat::from_euler(
                EulerRot::XYZ,
                -1.0,
                -0.5,
                0.0,
            )
        ),
    ));

    //commands.spawn(Camera2d); //NOTE: So we don't need a second camera for UI

    commands.spawn((
        Text::new("Hello Bevy!/n"),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
    ));
}

fn setup_level_collision(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    query: Query<(Entity, &Mesh3d), Without<Collider>>,
) {
    for (entity, mesh_handle) in &query {
        if let Some(mesh) = meshes.get(&mesh_handle.0) {

            if let Some(collider) =
                Collider::from_bevy_mesh(
                    mesh,
                    &ComputedColliderShape::TriMesh(
                        TriMeshFlags::default()
                    ),
                )
                {
                    commands.entity(entity)
                    .insert(RigidBody::Fixed)
                    .insert(collider);
                }
        }
    }
}

fn update_text(
    mut query: Query<&mut Text>,
) {
    for mut text in &mut query {
        *text = Text::new("Scrunklebunkle!\nFuture debug readout!");
    }
}

/*
//TODO: DELETE ALL OF THIS FROM HERE THIS IS DEMO CODE
fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>, mut commands: Commands) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
        //commands.spawn((Person, Name("Bumbleshit Fartkin".to_string())));
    }
}


fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}
*/
