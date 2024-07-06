use super::ArcBall;

use glam::Vec4;

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArcBallController {
    pub pan_sensitivity: f32,
    pub swivel_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub closest_zoom: f32,
}
impl ArcBallController {
    pub fn pivot(&mut self, arcball: &mut ArcBall, delta_x: f32, delta_y: f32) {
        arcball.yaw += delta_x * self.swivel_sensitivity;
        arcball.pitch += delta_y * self.swivel_sensitivity;

        arcball.pitch = arcball.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    pub fn pan(&mut self, arcball: &mut ArcBall, delta_x: f32, delta_y: f32) {
        let delta = Vec4::new(
            (-delta_x as f32) * arcball.distance,
            (delta_y as f32) * arcball.distance,
            0.0,
            0.0,
        ) * self.pan_sensitivity;

        // TODO: This is dumb, just use the cross product 4head
        let inv = arcball.matrix().inverse();
        let delta = (inv * delta).xyz();
        arcball.pivot += delta;
    }

    pub fn zoom(&mut self, arcball: &mut ArcBall, delta: f32) {
        arcball.distance += delta * self.zoom_sensitivity.powf(2.) * arcball.distance;
        arcball.distance = arcball.distance.max(self.closest_zoom);
    }
}

// Arbitrary
impl Default for ArcBallController {
    fn default() -> Self {
        Self {
            pan_sensitivity: 0.0015,
            swivel_sensitivity: 0.005,
            zoom_sensitivity: 0.04,
            closest_zoom: 0.01,
        }
    }
}
