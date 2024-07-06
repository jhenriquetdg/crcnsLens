#[derive(serde::Serialize, serde::Deserialize, Copy, Clone)]
pub struct Perspective {
    pub fov: f32,
    pub clip_near: f32,
    pub clip_far: f32,
}

// Arbitrary
impl Default for Perspective {
    fn default() -> Self {
        Self {
            fov: 60.0f32.to_radians(),
            clip_near: 0.01,
            clip_far: 2_000.0,
        }
    }
}

impl Perspective {
    pub fn matrix(&self, width: f32, height: f32) -> Mat4 {
        Mat4::perspective_rh(self.fov, width / height, self.clip_near, self.clip_far)
    }
}
