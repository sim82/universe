use bevy::prelude::*;
use heron::*;

use crate::consts::KILOMETER;

#[derive(Component)]
pub struct Ship {}

pub fn acceleration_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Acceleration,
            &mut Velocity,
            &mut Transform,
            &GlobalTransform,
        ),
        With<Ship>,
    >,
) {
    const LIN_ACCEL: f32 = 0.01 / KILOMETER;
    for (mut acceleration, mut velocity, mut transform, global_transform) in query.iter_mut() {
        let forward = global_transform.rotation * Vec3::X;
        let up = global_transform.rotation * Vec3::Y;
        let right = global_transform.rotation * Vec3::Z;

        if keyboard_input.pressed(KeyCode::Q) {
            // acceleration. += Velocity::from_angular(AxisAngle::new(Vec3::X, 0.1));
            *acceleration = Acceleration::from_angular(AxisAngle::new(forward, 0.1))
        } else if keyboard_input.pressed(KeyCode::E) {
            // acceleration. += Velocity::from_angular(AxisAngle::new(Vec3::X, 0.1));
            *acceleration = Acceleration::from_angular(AxisAngle::new(forward, -0.1))
        } else if keyboard_input.pressed(KeyCode::A) {
            // acceleration. += Velocity::from_angular(AxisAngle::new(Vec3::X, 0.1));
            *acceleration = Acceleration::from_angular(AxisAngle::new(up, 0.1))
        } else if keyboard_input.pressed(KeyCode::D) {
            // acceleration. += Velocity::from_angular(AxisAngle::new(Vec3::X, 0.1));
            *acceleration = Acceleration::from_angular(AxisAngle::new(up, -0.1))
        } else if keyboard_input.pressed(KeyCode::R) {
            // acceleration. += Velocity::from_angular(AxisAngle::new(Vec3::X, 0.1));
            *acceleration = Acceleration::from_angular(AxisAngle::new(right, 0.1))
        } else if keyboard_input.pressed(KeyCode::F) {
            // acceleration. += Velocity::from_angular(AxisAngle::new(Vec3::X, 0.1));
            *acceleration = Acceleration::from_angular(AxisAngle::new(right, -0.1))
        } else if keyboard_input.pressed(KeyCode::W) {
            *acceleration = Acceleration::from_linear(forward * -LIN_ACCEL)
        } else if keyboard_input.pressed(KeyCode::S) {
            *acceleration = Acceleration::from_linear(forward * LIN_ACCEL)
        } else if keyboard_input.pressed(KeyCode::Escape) {
            *velocity = Velocity::default()
        } else {
            *acceleration = Acceleration::default()
        }
        // info!("vel: {:?}", velocity);
        // info!("transform: {:?}", forward);
    }
}
