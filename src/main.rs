use bevy::prelude::*;
use std::f32::consts::PI;
use rand::Rng;

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Asteroid{
    divisible:bool
}

#[derive(Component)]
struct Laser;

#[derive(Component)]
struct Scoretext;

#[derive(Component)]
struct Shiptext;

#[derive(Component)]
struct TurnSpeed{
    value:f32
}

#[derive(Component)]
struct Speed{
    value:f32
}

#[derive(Component)]
struct Timer{
    value:f32
}

#[derive(Component)]
struct Shake{
    value:bool,
    default_time:f32,
    time:f32,
}

#[derive(Resource)]
struct Score {
    value:i32,
    ships:i32
}

impl Default for Score{
    fn default() -> Self {
        Self {
            value:0,
            ships:3,
        }
    }
}

#[derive(Resource)]
struct CountLaser{
    value:i32
}

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa {samples: 4})
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE))
        .insert_resource(Score::default())
        .insert_resource(CountLaser{value:0})
        //bevy itself
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "bevyastro".to_string(),
                width: 800.0,
                height: 600.0,
                ..default()
            },
            ..default()
        }))
        // system once
        .add_startup_system(setup)
        // system frame
        .add_system(input_ship)
        .add_system(turn)
        .add_system(moving)
        .add_system(timer)
        .add_system(shake)
        .add_system(scoreboard)
        .add_system(collision_ship)
        .add_system(collision_laser)
        .run();
}

const ASTROID_NUM:i32=3;
const ASTROID_SIZE:f32=0.5;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // light
    commands.spawn(PointLightBundle{
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
   commands.spawn(Camera3dBundle{ //0.8
        transform: Transform::from_xyz(0.0,20.0,0.5).looking_at(Vec3::new(0.,0.,0.), Vec3::Y),
        ..Default::default()
    })
   .insert(UiCameraConfig {
       show_ui: true,
       ..default()
   });

    // scoreboard
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Score:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            }
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Scoretext);

    commands.spawn(TextBundle {
        text: Text::from_section(
            "Ship:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            }
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(25.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Shiptext);

    // ship
    let ship_position = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn(TransformBundle{
        local: Transform::from_translation(ship_position),
        global: GlobalTransform::IDENTITY
        })
        .insert(SceneBundle {
                scene: asset_server.load("models/ship.gltf#Scene0"),
                ..Default::default()
            })
    .insert(Ship)
    .insert(TurnSpeed{value:0.0})
    .insert(Shake{value:false, default_time:2.0, time:0.0})
    .insert(Speed{value:0.0});

    //Asteroids
    let mut rng = rand::thread_rng();
    for _ in 0..ASTROID_NUM {
        //find position
        let mut pos = Vec3::new(0.0,0.0,0.0);
        let mut found = false;
        while !found {
            let x = rng.gen_range(BOUND_MIN_X..BOUND_MAX_X);
            let z = rng.gen_range(BOUND_MIN_Z..BOUND_MAX_Z);
            pos = Vec3::new(x, 0.0, z);
            found = pos.distance(ship_position) > 2.0 * ASTROID_SIZE;
        }
        //create parts
        let mut children_list:Vec<Entity> = Vec::new();
        for _ in 0..4{
            let child_position = Vec3::new(rng.gen_range(0.0..ASTROID_SIZE),
                rng.gen_range(0.0..ASTROID_SIZE),
                rng.gen_range(0.0..ASTROID_SIZE));
            let entity = commands.spawn(PbrBundle {
               mesh: meshes.add(Mesh::from(shape::Icosphere { radius: ASTROID_SIZE, subdivisions: 32, })),
               material: materials.add(asteroid_color().into()),
               transform: Transform::from_translation(child_position),
                ..Default::default()
            }).id();
            children_list.push(entity);
        }
        //direction
        let direction = rng.gen_range(0.0..2.0);

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: ASTROID_SIZE, subdivisions: 32, })),
            material: materials.add(asteroid_color().into()),
            transform: Transform{
                translation: pos,
                rotation: Quat::from_rotation_y(direction*PI),
                scale: Vec3::new(1.0,1.0,1.0)
            },
            ..Default::default()
        })
        .push_children(&children_list)
        .insert(Speed{value:1.0})
        .insert(Asteroid{divisible:true});
    }
}

fn asteroid_color()->Color {
    let mut rng = rand::thread_rng();
    Color::from([rng.gen_range(0.8..1.0),
        rng.gen_range(0.8..1.0),
        rng.gen_range(0.8..1.0)])
}

const TURN_SPEED:f32= PI;
const SHIP_THRUST:f32= 1.0;
const FRICTION:f32=0.8;
const MAX_LASER:i32=10;

fn input_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut count_laser: ResMut<CountLaser>,
    time:Res<Time>,
    keyboard_input:Res<Input<KeyCode>>,
    mut query: Query<(&mut TurnSpeed,&mut Speed, &Transform, &Shake), With<Ship>>
){
    let (mut turnspeed,mut speed,transform,shake) = query.single_mut();
    if !shake.value {
        turnspeed.value = if keyboard_input.pressed(KeyCode::Left) {
            TURN_SPEED
        } else if keyboard_input.pressed(KeyCode::Right) {
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
        if keyboard_input.just_pressed(KeyCode::Space) {
            if count_laser.value <= MAX_LASER {
                count_laser.value += 1;
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 1.6))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::LIME_GREEN,
                        emissive: Color::LIME_GREEN,
                        ..Default::default()
                    }),
                    transform: Transform {
                        translation: transform.translation,
                        rotation: transform.rotation,
                        scale: Vec3::new(1.0, 1.0, 1.0)
                    },
                    ..Default::default()
                })
                    .insert(Timer { value: 1.0 })
                    .insert(Speed { value: 8.0 })
                    .insert(Laser);
            }
        }
    }
}

fn turn(
    time:Res<Time>,
    mut query: Query<(&mut Transform, &mut TurnSpeed)>
){
    for (mut transform, turnspeed) in query.iter_mut() {
        if turnspeed.value != 0.0 {
            let rotation_change = Quat::from_rotation_y(turnspeed.value*time.delta_seconds());
            transform.rotate(rotation_change);
        }
    }
}

fn timer(
    mut commands: Commands,
    time:Res<Time>,
    mut count_laser: ResMut<CountLaser>,
    mut query: Query<(Entity, &mut Timer)>
){
    for (entity, mut timer) in query.iter_mut(){
        timer.value -= time.delta_seconds();
        if timer.value < 0.0 {
            commands.entity(entity).despawn_recursive();
            count_laser.value -= 1;
        }
    }
}

const COLLISION_DISTANCE:f32 = ASTROID_SIZE * 2.0;

fn collision_laser(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut count_laser: ResMut<CountLaser>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_laser: Query<(Entity, &Transform),With<Laser>>,
    query_astro: Query<(Entity, &Transform, &Asteroid)>
){
    for (laser, laser_transform) in query_laser.iter() {
        for (asteroid_entity, asteroid_transform, asteroid) in query_astro.iter() {
            if laser_transform.translation.distance(asteroid_transform.translation) < COLLISION_DISTANCE {
                commands.entity(laser).despawn_recursive();
                count_laser.value -=1;
                if asteroid.divisible {
                    let mut rng = rand::thread_rng();
                    for _ in 0..5{
                        commands.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: ASTROID_SIZE, subdivisions: 32, })),
                            material: materials.add(asteroid_color().into()),
                            transform: Transform {
                                translation: asteroid_transform.translation.clone(),
                                rotation: Quat::from_rotation_y(rng.gen_range(0.0..2.0)*PI),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Speed{value:1.0})
                        .insert(Asteroid{divisible:false});
                    }
                }
                score.value += 10;
                commands.entity(asteroid_entity).despawn_recursive();
            }
        }

    }
}


const COLLISION_DISTANCE_SHIP:f32 = ASTROID_SIZE * 2.0;

fn collision_ship(
    mut score: ResMut<Score>,
    mut query_ship: Query<(&Transform, &mut Shake),With<Ship>>,
    query_astro: Query<&Transform, With<Asteroid>>
){
    let (ship_transform, mut ship_shake) = query_ship.single_mut();
    if !ship_shake.value {
        for asteroid_transform in query_astro.iter() {
            if ship_transform.translation.distance(asteroid_transform.translation) < COLLISION_DISTANCE_SHIP {
                score.ships -= 1;
                ship_shake.value = true;
                ship_shake.time = ship_shake.default_time;
            }
        }

    }
}

fn scoreboard(
    score: Res<Score>,
    mut score_query: Query<(&mut Text, With<Scoretext>, Without<Shiptext>)>,
    mut ship_query: Query<&mut Text, With<Shiptext>>,
) {
    let (mut text,_,_) = score_query.single_mut();
    text.sections[0].value = format!("Score: {}", score.value);

    let mut ship_text = ship_query.single_mut();
    ship_text.sections[0].value = format!("Ship: {}", score.ships);
}

const BOUND_MAX_X:f32 = 11.0;
const BOUND_MIN_X:f32 = -11.0;
const BOUND_MAX_Z:f32 = 8.0;
const BOUND_MIN_Z:f32 = -8.0;

fn moving(
    time:Res<Time>,
    mut query: Query<(&mut Transform, &mut Speed)>,
){
    for (mut transform, speed) in query.iter_mut() {
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

const SHAKE_VARIANCE:f32=0.5;

fn shake(
    time:Res<Time>,
    mut query: Query<(&mut Shake, &mut Transform)>
){
    for (mut shake, mut transform) in query.iter_mut(){
        if shake.value {
            shake.time -= time.delta_seconds();
            if shake.time < 0.0 {
                shake.value = false;
                shake.time = 0.0
            } else {
                let mut rng = rand::thread_rng();
                transform.translation = Vec3::new(rng.gen_range(0.0..SHAKE_VARIANCE),
                                                  0.0,
                                                  rng.gen_range(0.0..SHAKE_VARIANCE))
            }
        }
    }
}

