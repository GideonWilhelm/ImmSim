use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub enum PlayerMode {
    Gameplay,
    Editor,
}

impl PlayerMode {
    pub fn name(&self) -> &'static str {
        match self {
            PlayerMode::Gameplay => "Gameplay",
            PlayerMode::Editor => "Editor",
        }
    }
}

#[derive(Component)]
pub struct PlayerVelocity {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct LookAngles {
    pub yaw: f32,
    pub pitch: f32,
}


pub fn spawn_player(mut commands: Commands) {
    let player = commands.spawn((
        Player,
        PlayerVelocity {
            velocity: Vec3::ZERO,
        },
        LookAngles {
            yaw: 0.0,
            pitch: 0.0,
        },
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.4),
                min_width: CharacterLength::Absolute(0.2),
                include_dynamic_bodies: false,
            }),
            snap_to_ground: Some(CharacterLength::Absolute(0.3)),
            ..default()
        },
        Collider::capsule_y(0.9, 0.3),
        Transform::from_xyz(0.0, 2.0, 3.0),
        InheritedVisibility::VISIBLE,
    )).id();


    commands.entity(player).with_children(|parent| {
        parent.spawn((
            PlayerCamera,
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.7, 0.0),
        ));
    });
}

pub fn toggle_player_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<PlayerMode>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        *mode = match *mode {
            PlayerMode::Gameplay => PlayerMode::Editor,
            PlayerMode::Editor => PlayerMode::Gameplay,
        };
    }
}

pub fn move_player_gameplay(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<PlayerMode>,
    mut query: Query<(
        &mut KinematicCharacterController,
        &mut PlayerVelocity,
        &Transform,
    ), With<Player>>,
) {
    let (mut controller, mut velocity, transform) = query.single_mut().unwrap();

    if !matches!(*mode, PlayerMode::Gameplay) {
        return;
    }

    let dt = time.delta_secs();

    let mut input = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        input.z += 1.0; //positive z is forward in this ecosystem for some reason
    }
    if keyboard.pressed(KeyCode::KeyS) {
        input.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    if input.length_squared() > 0.0 {
        input = input.normalize();
    }

    let movement =
    (transform.right() * input.x +
    transform.forward() * input.z)
    * 5.0;

    velocity.velocity.y -= 9.81 * dt;

    controller.translation = Some(
        movement * dt +
        Vec3::Y * velocity.velocity.y * dt
    );
}

pub fn move_player_editor(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<PlayerMode>,
    mut query: Query<(
        &mut KinematicCharacterController,
        &mut PlayerVelocity,
        &Transform,
    ), With<Player>>,
) {
    let (mut controller, velocity, transform) = query.single_mut().unwrap();

    if !matches!(*mode, PlayerMode::Editor) {
        return;
    }

    let dt = time.delta_secs();

    let mut input = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        input.z += 1.0; //positive z is forward in this ecosystem for some reason
    }
    if keyboard.pressed(KeyCode::KeyS) {
        input.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Space) {
        input.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ControlLeft) {
        input.y -= 1.0;
    }

    if input.length_squared() > 0.0 {
        input = input.normalize();
    }

    let movement = (
            transform.right() * input.x +
            transform.forward() * input.z +
            transform.up() * input.y
        )
        * 7.0;

    controller.translation = Some(
        movement * dt +
        Vec3::Y * velocity.velocity.y * dt
    );
}

pub fn handle_grounding(
    mut query: Query<(
        &mut PlayerVelocity,
        &KinematicCharacterControllerOutput
    ), With<Player>>,
) {
    for (mut velocity, output) in &mut query {
        if output.grounded {
            velocity.velocity.y = 0.0;
        }
    }
}

pub fn look_player(
    mut mouse: MessageReader<MouseMotion>,
    mut query: Query<(&mut LookAngles, &mut Transform), With<Player>>,
) {
    let Ok((mut look, mut transform)) = query.single_mut() else {
        return;
    };

    let mut delta = Vec2::ZERO;

    for event in mouse.read() {
        delta += event.delta;
    }


    let sensitivity = 0.002;


    look.yaw -= delta.x * sensitivity;
    look.pitch -= delta.y * sensitivity;


    look.pitch = look.pitch.clamp(-1.54,1.54);


    transform.rotation =
    Quat::from_rotation_y(look.yaw);


    // camera pitch handled separately
}

pub fn lock_cursor(
    mut cursor: Single<&mut CursorOptions>
) {
    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn look_camera(
    mut query: Query<(&ChildOf, &mut Transform), With<PlayerCamera>>,
                   player_query: Query<&LookAngles>,
) {
    for (parent, mut transform) in &mut query {

        if let Ok(look) = player_query.get(parent.parent()) {
            transform.rotation =
            Quat::from_rotation_x(look.pitch);
        }
    }
}
