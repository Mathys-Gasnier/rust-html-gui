use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}};

use crate::math::bounding_box::BoundingBox;

use super::container_direction::ContainerDirection;


#[derive(Debug, Clone)]
pub enum RenderableElement {
    Text(String, u8, BoundingBox),
    Element {
        bounding_box: BoundingBox,
        childs: Vec<RenderableElement>,
        scrollable: Option<ContainerDirection>
    },
}

impl RenderableElement {

    pub fn render(self, d: &mut RaylibDrawHandle) {
        match self {
            Self::Text(text, font_size, bounding_box) => d.draw_text_ex(
                d.get_font_default(),
                &text,
                bounding_box.vector2(),
                font_size as f32,
                if font_size > 10 { font_size as f32 / 10.0 } else { 1.0 },
                Color::BLACK
            ),
            Self::Element { bounding_box, childs, scrollable } => {
                #[cfg(debug_assertions)]
                d.draw_rectangle_lines_ex(bounding_box.border_rect(), 1.0, Color::BLACK);

                childs.into_iter().for_each(|child| child.render(d));
            }
        }
    }

}