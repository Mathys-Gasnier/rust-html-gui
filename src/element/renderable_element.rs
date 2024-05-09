use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, math::Vector2};

use crate::math::bounding_box::BoundingBox;

use super::container_direction::ContainerDirection;


#[derive(Debug, Clone)]
pub enum RenderableElement {
    Text(String, u8, Color, BoundingBox),
    Element {
        id: Option<String>,
        bounding_box: BoundingBox,
        childs: Vec<RenderableElement>,
        scrollable: Option<ContainerDirection>,
        background: Option<Color>,
        border: Option<Color>,
    },
}

impl RenderableElement {

    pub fn id(&self) -> &Option<String> {
        match self {
            Self::Text(_, _, _, _) => &None,
            Self::Element { id, .. } => id,
        }
    }

    pub fn bounding_box(&self) -> &BoundingBox {
        match self {
            Self::Text(_, _, _, bounding_box) => bounding_box,
            Self::Element { bounding_box, .. } => bounding_box,
        }
    }

    pub fn childs(&self) -> Option<&Vec<RenderableElement>> {
        match self {
            Self::Text(_, _, _, _) => None,
            Self::Element { childs, .. } => Some(childs),
        }
    }

    pub fn ids_at_position(&self, position: Vector2) -> Vec<String> {
        let mut vec = vec![];

        if self.bounding_box().within(position) {
            if let Some(id) = self.id() {
                vec.push(id.clone());
            }
        }

        if let Some(childs) = self.childs() {
            for child in childs {
                vec.append(&mut child.ids_at_position(position));
            }
        }

        vec
    }

    pub fn render(self, d: &mut RaylibDrawHandle) {
        match self {
            Self::Text(text, font_size, text_color, bounding_box) => d.draw_text_ex(
                d.get_font_default(),
                &text,
                bounding_box.vector2(),
                font_size as f32,
                if font_size > 10 { font_size as f32 / 10.0 } else { 1.0 },
                text_color
            ),
            Self::Element { bounding_box, childs, scrollable, border, background, .. } => {
                if let Some(color) = background {
                    d.draw_rectangle_rec(bounding_box.border_rect(), color);
                }
                if let Some(color) = border {
                    d.draw_rectangle_lines_ex(bounding_box.border_rect(), 1.0, color);
                }

                childs.into_iter().for_each(|child| child.render(d));
            }
        }
    }

}