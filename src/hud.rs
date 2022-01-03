use std::collections::VecDeque;

use bevy::{
    diagnostic::{DiagnosticId, Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

// mod button;

// // FIXME: only defined here because hud code directly modifies it. Implementation should be moved from main.rs
pub struct DemoSystemState {
    pub cycle_timer: Timer,
}

impl Default for DemoSystemState {
    fn default() -> Self {
        DemoSystemState {
            cycle_timer: Timer::from_seconds(1f32, true),
        }
    }
}

// /// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
pub struct RenderStatus {
    pub text: String,
}

impl Default for RenderStatus {
    fn default() -> Self {
        RenderStatus {
            text: "unknown".into(),
        }
    }
}

// #[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum HudSrc {
    Diagnostics(String, DiagnosticId, bool),
    RenderStatus,
    LoadingScreen,
    PropertName(String),
    PropertyAccess,
}

#[derive(Clone, Debug, Component)]
pub enum HudElement {
    TextWithSource(HudSrc),
    ToggleButtonProperty(String, String, String),
    EditThis,
}

#[derive(Component)]
pub struct HudPlotDiagnostic {
    pub(crate) id: DiagnosticId,
    pub(crate) name: String,
    pub(crate) buf: VecDeque<bevy_egui::egui::plot::Value>,
    pub(crate) x: f64,
}

impl HudPlotDiagnostic {
    pub fn new(id: DiagnosticId, name: &str) -> Self {
        HudPlotDiagnostic {
            id,
            name: name.to_string(),
            buf: VecDeque::new(),
            x: 0.0,
        }
    }
}
