use serde::{Deserialize, Serialize};

pub mod ship;

pub mod prelude {
    pub use super::consts::*;
    pub use super::ship;
}

pub mod consts {
    pub const KILOMETER: f32 = 1e-3;
    pub const RADIUS_SUN: f32 = 1400000.0 * KILOMETER;
    pub const ORBIT_EARTH: f32 = 14000000.0 * KILOMETER;
    pub const RADIUS_EARTH: f32 = 6100.0 * KILOMETER * 1.0;
    pub const RADIUS_MOON: f32 = 1700.0 * KILOMETER;
    pub const ORBIT_MOON: f32 = 370000.0 * KILOMETER;
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub name: String,
    pub orbit: f32,
    pub orbit_time: f32,
    pub day: f32,
    pub satellites: Vec<Body>,
    pub radius: f32,
    pub appearance: String,
}

#[test]
fn test_body() {
    let sun = Body {
        name: "sun".to_string(),
        orbit: 0.0,
        orbit_time: 0.0,
        day: -365.0,
        radius: 1.4e6,
        appearance: "earth_gltf02/earth.gltf".into(),
        satellites: vec![Body {
            name: "earth".into(),
            orbit: 14e6,
            orbit_time: 365.0,
            day: 1.0,
            radius: 6.1e3,
            appearance: "earth_gltf02/earth.gltf".into(),
            satellites: vec![Body {
                name: "moon".into(),
                orbit: 370e6,
                orbit_time: 31.0,
                day: 31.0,
                radius: 1.7e3,
                appearance: "moon_gltf01/moon.gltf".into(),
                satellites: vec![],
            }],
        }],
    };

    println!("{}", serde_yaml::to_string(&sun).unwrap());
}