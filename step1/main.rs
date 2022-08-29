use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct TurnSpeed{
    value:f32
}

#[derive(Component)]
struct Speed{
    value:f32
}

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa {samples: 4})
        .insert_resource(WindowDescriptor{
            title: "bevyastro".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        //.insert_resource(Score::default())
        //bevy itself
        .add_plugins(DefaultPlugins)
        // system once
        .add_startup_system(setup)
        // system frame
        .add_system(input_ship)
        .add_system(turn)
        .add_system(moving)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // light
    commands.spawn_bundle(PointLightBundle{
        point_light: PointLight{
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..Default::default()
    });
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });
    //camera
   commands.spawn_bundle(PerspectiveCameraBundle{
        transform: Transform::from_xyz(0.0,20.0,0.5).looking_at(Vec3::new(0.,0.,0.), Vec3::Y),
        ..Default::default()
    });

    // ship
    commands.spawn_bundle((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::identity(),
    ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/ship.gltf#Scene0"));
    })
    .insert(Ship)
    .insert(TurnSpeed{value:0.0})
    .insert(Speed{value:0.0});
}

const TURN_SPEED:f32= PI;
const SHIP_THRUST:f32= 1.0;
const FRICTION:f32=0.8;

fn input_ship(
    time:Res<Time>,
    keyboard_input:Res<Input<KeyCode>>,
    mut query: Query<(&mut TurnSpeed,&mut Speed), With<Ship>>
){
    let (mut turnspeed,mut speed) = query.single_mut();
    turnspeed.value = if keyboard_input.pressed(KeyCode::Left) {
        TURN_SPEED
    } else if keyboard_input.pressed(KeyCode::Right){
        -TURN_SPEED
    } else {
        0.0
    };
    speed.value = if keyboard_input.pressed(KeyCode::Up) {
        speed.value + SHIP_THRUST * time.delta_seconds()
    } else {
        if speed.value > 0.0 {
            speed.value - FRICTION * time.delta_seconds()
        } else {
            0.0
        }
    };
}

fn turn(
    time:Res<Time>,
    mut query: Query<(&mut Transform, &mut TurnSpeed)>
){
    for (mut transform,mut turnspeed) in query.iter_mut() {
        if turnspeed.value != 0.0 {
            let rotation_change = Quat::from_rotation_y(turnspeed.value*time.delta_seconds());
            transform.rotate(rotation_change);
        }
    }
}

const BOUND_MAX_X:f32 = 11.0;
const BOUND_MIN_X:f32 = -11.0;
const BOUND_MAX_Z:f32 = 8.0;
const BOUND_MIN_Z:f32 = -8.0;

fn moving(
    time:Res<Time>,
    mut query: Query<(&mut Transform, &mut Speed)>,
){
    for (mut transform, mut speed) in query.iter_mut() {
        if speed.value != 0.0 {
            let translation_change = transform.forward() * speed.value * time.delta_seconds();
            transform.translation -= translation_change;

            if transform.translation.x < BOUND_MIN_X { transform.translation.x = BOUND_MAX_X}
            else if transform.translation.x > BOUND_MAX_X {transform.translation.x = BOUND_MIN_X};
            if transform.translation.z < BOUND_MIN_Z { transform.translation.z = BOUND_MAX_Z}
            else if transform.translation.z > BOUND_MAX_Z {transform.translation.z = BOUND_MIN_Z};
        }
    }
}