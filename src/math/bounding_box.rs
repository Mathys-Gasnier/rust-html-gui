use raylib::math::{Rectangle, Vector2};


#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub margin: i32,
    pub padding: i32,
}

impl BoundingBox {

    pub fn within(&self, position: Vector2) -> bool {
        self.border_rect().check_collision_point_rec(position)
    }

    pub fn vector2(&self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }

    pub fn border_rect(&self) -> Rectangle {
        Rectangle::new(
            (self.x + self.margin) as f32, (self.y + self.margin) as f32,
            (self.width - self.margin * 2) as f32, (self.height - self.margin * 2) as f32
        )
    }

}