use eframe::emath::RectTransform;
use egui::{Color32, Pos2, Vec2};

#[derive(Debug)]
pub struct Shape {
    pub pos: Vec2,
    pub size: f32,
    pub ty: ShapeType,
    pub color: usize,
}

impl Shape {
    pub fn as_egui_shape(&self, transform: RectTransform, colors: &[[u8; 3]; 16]) -> egui::Shape {
        let color_array = colors[self.color];
        let color = Color32::from_rgb(color_array[0], color_array[1], color_array[2]);

        match self.ty {
            ShapeType::Circle => egui::Shape::Circle(eframe::epaint::CircleShape::filled(
                transform * (Pos2::new(0.5, 0.5) + self.pos),
                transform.scale().max_elem() * (self.size + 0.5),
                color,
            )),
            ShapeType::Square => egui::Shape::Rect(eframe::epaint::RectShape::filled(
                egui::Rect::from_min_max(
                    transform * Pos2::new(self.pos.x - self.size, self.pos.y - self.size),
                    transform
                        * Pos2::new(self.pos.x + self.size + 1.0, self.pos.y + self.size + 1.0),
                ),
                egui::Rounding::none(),
                color,
            )),
            ShapeType::Triangle => egui::Shape::Mesh({
                let mut mesh = egui::Mesh::default();
                mesh.colored_vertex(
                    transform * Pos2::new(self.pos.x - self.size, self.pos.y + 1.0 + self.size),
                    color,
                );
                mesh.colored_vertex(
                    transform
                        * Pos2::new(self.pos.x + 1.0 + self.size, self.pos.y + 1.0 + self.size),
                    color,
                );
                mesh.colored_vertex(
                    transform * Pos2::new(self.pos.x + 0.5, self.pos.y - self.size),
                    color,
                );
                mesh.indices = vec![0, 1, 2];
                mesh
            }),
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            pos: Vec2::new(7.0, 7.0),
            size: 0.0,
            ty: ShapeType::Circle,
            color: 0,
        }
    }
}

#[derive(Debug)]
pub enum ShapeType {
    Circle,
    Square,
    Triangle,
}
