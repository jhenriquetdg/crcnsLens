pub mod controller;

pub use controller::ArcBallController;

/// Arcball camera parameters
#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArcBall {
    pub pivot: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

// Arbitrary
impl Default for ArcBall {
    fn default() -> Self {
        Self {
            pivot: Vec3::ZERO,
            pitch: 0.3,
            yaw: -1.92,
            distance: 3.,
        }
    }
}

impl ArcBall {
    pub fn matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.pivot + self.eye(),
            self.pivot,
            Vec3::new(0.0, 1.0, 0.0),
        )
    }

    pub fn eye(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos().abs(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos().abs(),
        ) * self.distance
    }
}
