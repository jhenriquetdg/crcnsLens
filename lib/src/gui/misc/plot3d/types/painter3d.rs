#[derive(Clone)]
pub struct Painter3D {
    transform: Transform,
    painter_2d: egui::Painter,
}

impl Painter3D {
    pub fn new(painter_2d: egui::Painter, transform: Transform) -> Self {
        Self {
            transform,
            painter_2d,
        }
    }

    pub fn arrow(&self, pos: Vec3, dir: Vec3, screen_len: f32, stroke: Stroke) {
        let a = pos;
        let b = pos + dir;

        let Some(a) = self.transform(a) else { return };
        let Some(b) = self.transform(b) else { return };

        self.painter_2d
            .arrow(a, (b - a).normalized() * screen_len, stroke)
    }

    pub fn line(&self, a: Vec3, b: Vec3, stroke: Stroke) {
        let Some(a) = self.transform(a) else { return };
        let Some(b) = self.transform(b) else { return };

        self.painter_2d.line_segment([a, b], stroke);
    }

    pub fn circle_filled(&self, center: Vec3, radius: f32, fill_color: impl Into<Color32>) {
        let Some(center) = self.transform(center) else {
            return;
        };
        self.painter_2d.circle_filled(center, radius, fill_color);
    }

    pub fn circle(&self, center: Vec3, radius: f32, stroke: impl Into<Stroke>) {
        let Some(center) = self.transform(center) else {
            return;
        };
        self.painter_2d.circle_stroke(center, radius, stroke);
    }

    pub fn text(
        &self,
        pos: Vec3,
        anchor: egui::Align2,
        text: impl ToString,
        font_id: egui::FontId,
        text_color: Color32,
    ) -> Option<egui::Rect> {
        self.transform(pos)
            .map(|pos| self.painter_2d.text(pos, anchor, text, font_id, text_color))
    }

    /// Transform a point in world coordinates to egui coordinates
    pub fn transform(&self, pt: Vec3) -> Option<egui::Pos2> {
        let (sc, z) = self.transform.world_to_egui(pt);

        (0.0..=1.0).contains(&z).then(|| sc.to_pos2())
    }

    pub fn internal_transform(&self) -> &Transform {
        &self.transform
    }

    /// Get egui's 2D painter
    pub fn egui(&self) -> &egui::Painter {
        &self.painter_2d
    }

    /*
    /// Returns a painter which has the given transformation prepended
    pub fn transform(&self, mat: Mat4) -> Self {
        Self {
            transform: self.transform.prepend(Transform::from(mat)),
            // Context is Arc underneath so this is cheap
            painter_2d: self.painter_2d.clone(),
        }
    }
    */
}
