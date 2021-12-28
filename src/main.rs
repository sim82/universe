use bevy::{prelude::*, render::primitives::Frustum, scene::InstanceId};

#[derive(Component)]
struct Planet {
    vel: f32,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Project Universe"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(animate_light_direction)
        .add_system(animate_camera)
        .add_system(turn_earth)
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const RADIUS_EARTH: f32 = 6100000.0;
    const RADIUS_MOON: f32 = 1700000.0;

    commands
        .spawn_bundle(planet_bundle(
            "models/earth_gltf02/earth.gltf",
            RADIUS_EARTH,
            &asset_server,
        ))
        .insert(Planet { vel: 0.03 });
    let x = 370000000.0;

    commands
        .spawn_bundle(TransformNodeBundle::default())
        .insert(Planet { vel: 0.3 })
        .with_children(|f| {
            f.spawn_bundle(TransformNodeBundle {
                transform: Transform::from_translation(Vec3::new(x, 0.0, 0.0)),
                ..Default::default()
            })
            .insert(Planet { vel: 0.03 })
            .with_children(|f| {
                f.spawn_bundle(planet_bundle(
                    "models/moon_gltf01/moon.gltf",
                    RADIUS_MOON,
                    &asset_server,
                ));
            });

            // .insert(Planet);
        });

    // commands

    let perspective_projection = PerspectiveProjection {
        fov: std::f32::consts::PI / 4.0,
        near: 0.1,
        far: 1000000000.0,
        aspect_ratio: 1.0,
    };

    //commands.spawn_scene(asset_server.load("models/earth_gltf01/earth.gltf#Scene0"));
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, RADIUS_EARTH + 1000000.0).looking_at(
            Vec3::new(RADIUS_EARTH, 0.0, RADIUS_EARTH - 1000000.0),
            Vec3::Z,
        ),
        // transform: Transform::from_xyz(0.0, RADIUS_EARTH * 10.0, 0.0)
        //     .looking_at(Vec3::new(0.0, 0.0, 0.0), -Vec3::Z),
        perspective_projection,
        ..Default::default()
    });
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
        transform.rotation *= Quat::from_rotation_y(planet.vel * time.delta_seconds());
        // transform.translation.x += 1000.0;
        // info!("camera: {:?} {:?}", transform.translation, frustum);
    }
}
