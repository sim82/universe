use bevy::{prelude::*, reflect::TypeRegistry, render::primitives::Frustum, scene::InstanceId};
use heron::*;
use universe::prelude::*;

#[derive(Component)]
struct Planet {
    vel: f32,
}

#[derive(Component, Reflect)]
struct Center {
    vel: f32,
    name: String,
}

#[derive(Component)]
struct BodyAppearance {
    radius: f32,
    model: String,
    vel: f32,
}

impl Center {
    pub fn new(name: &str) -> Self {
        Center {
            vel: 0.0,
            name: name.to_string(),
        }
    }
}

#[derive(Component)]
struct Rotation {
    vel: f32,
}

fn main() {
    App::new()
        .register_type::<Center>()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Project Universe"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup)
        .add_system(spawn_planets)
        // .add_system(animate_light_direction)
        .add_system(animate_camera)
        .add_system(turn_earth)
        .add_system(rotation_system)
        .add_system(ship::acceleration_system)
        .run();
}

#[derive(Bundle, Clone, Default)]
struct TransformNodeBundle {
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    global_transform: GlobalTransform,
    transform: Transform,
}

fn planet_bundle(name: &str, radius: f32, asset_server: &AssetServer) -> PbrBundle {
    let earth_handle = asset_server.load(&format!("{}#Mesh0/Primitive0", name));
    // You can also add assets directly to their Assets<T> storage:

    let material_handle = asset_server.load(&format!("{}#Material0", name));

    PbrBundle {
        mesh: earth_handle,
        material: material_handle,
        transform: Transform::from_scale(Vec3::new(radius, radius, radius)),
        ..Default::default()
    }
}

fn spawn_satellites(bodies: &[universe::Body], f: &mut ChildBuilder) {
    for body in bodies.iter() {
        let oribit_vel = if body.orbit_time > 0.0 {
            1.0 / body.orbit_time
        } else {
            0.0
        };
        let vel = if body.day > 0.0 { 1.0 / body.day } else { 0.0 };
        f.spawn_bundle(TransformNodeBundle::default())
            .insert(Rotation { vel: oribit_vel })
            .with_children(|f| {
                f.spawn_bundle(TransformNodeBundle {
                    transform: Transform::from_translation(Vec3::new(
                        body.orbit * KILOMETER,
                        0.0,
                        0.0,
                    )),
                    ..Default::default()
                })
                .insert(Center::new(&body.name))
                .insert(BodyAppearance {
                    radius: body.radius,
                    model: body.appearance.clone(),
                    vel,
                })
                //.insert(Rotation { vel })
                .with_children(|f| {
                    spawn_satellites(&body.satellites, f);
                });
            });
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun: universe::Body = serde_yaml::from_reader(std::io::BufReader::new(
        std::fs::File::open("assets/system.yaml").unwrap(),
    ))
    .unwrap();

    commands
        .spawn_bundle(TransformNodeBundle::default())
        .insert(Center::new(&sun.name))
        .insert(BodyAppearance {
            radius: sun.radius,
            model: sun.appearance,
            vel: 0.0,
        })
        .with_children(|f| {
            spawn_satellites(&sun.satellites, f);
        })
        .id();

    let perspective_projection = PerspectiveProjection {
        fov: std::f32::consts::PI / 4.0,
        near: 0.1,
        far: 10.0 * ORBIT_EARTH,
        aspect_ratio: 1.0,
    };

    // Cube (with radius)
    let ship = commands
        .spawn_bundle(TransformNodeBundle::default())
        .insert(Transform {
            translation: Vec3::new(
                ORBIT_EARTH * 1e-1 + RADIUS_EARTH + 100.0 * KILOMETER,
                0.0,
                0.0,
            ),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(0.3, 0.3, 0.3),
            border_radius: Some(0.3),
        })
        .insert(Acceleration::default())
        .insert(Velocity::default())
        //.insert(Velocity::from_angular(AxisAngle::new(Vec3::X, 1.0)))
        .insert(ship::Ship {})
        .with_children(|f| {
            f.spawn_bundle(PerspectiveCameraBundle {
                // transform: Transform::from_xyz(ORBIT_EARTH, 10e6, 0.0)
                //     .looking_at(Vec3::new(ORBIT_EARTH, 0.0, 0.0), Vec3::Z),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Y),
                perspective_projection,
                ..Default::default()
            });
        })
        .id();

    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_planets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &Center, &BodyAppearance), Added<Center>>,
) {
    for (entity, center, appearance) in query.iter() {
        info!(
            "spawn {} {} {} {}",
            center.name, appearance.model, appearance.radius, appearance.vel
        );
        commands.entity(entity).with_children(|f| {
            f.spawn_bundle(planet_bundle(
                &format!("models/{}", appearance.model),
                appearance.radius * KILOMETER,
                &asset_server,
            ))
            .insert(Rotation {
                vel: appearance.vel,
            });
        });
    }
}
fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

fn animate_camera(time: Res<Time>, mut query: Query<(&mut Transform, &Frustum), With<Camera>>) {
    for (mut transform, frustum) in query.iter_mut() {
        // transform.translation.x += 1000.0;
        // info!("camera: {:?} {:?}", transform.translation, frustum);
    }
}

fn turn_earth(time: Res<Time>, mut query: Query<(&mut Transform, &Planet)>) {
    for (mut transform, planet) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(planet.vel * 0.001 * time.delta_seconds());
        // transform.translation.x += 1000.0;
        // info!("camera: {:?} {:?}", transform.translation, frustum);
    }
}

fn rotation_system(time: Res<Time>, mut query: Query<(&mut Transform, &Rotation)>) {
    for (mut transform, rotation) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(rotation.vel * 1.0e-1 * time.delta_seconds());
        // transform.translation.x += 1000.0;
        // info!("camera: {:?} {:?}", transform.translation, frustum);
    }
}
