mod player;
mod game;
mod map;
mod ui;
mod common;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use player::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, player::spawn_player))
        .add_systems(Update, (update_text,
            player::move_player_gameplay,
            player::move_player_editor,
            player::toggle_player_mode,
            player::look_player,
            player::look_camera,
            player::lock_cursor,
            player::handle_grounding,
            map::editor_pick,
            map::setup_level_collision,
            quit
        ))
        .insert_resource(player::PlayerMode::Editor)
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

fn update_text(
    mode: Res<PlayerMode>,
    player: Query<&PlayerVelocity, With<Player>>,
    mut text_query: Query<&mut Text>,
) {
    let velocity = player.single().unwrap();

    for mut text in &mut text_query {
        *text = Text::new(format!(
            "Mode: {}\n\
            Velocity: {:.2}, {:.2}, {:.2}",
            mode.name(),
            velocity.velocity.x,
            velocity.velocity.y,
            velocity.velocity.z,
        ));
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
