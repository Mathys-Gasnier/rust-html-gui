use crate::math::{bounding_box::BoundingBox, size::Size};

use super::{container_direction::ContainerDirection, renderable_element::RenderableElement};

#[derive(Debug, Clone)]
pub enum PreparedElement {
    Text(String, u8, ( Size, Size )),
    Element {
        size: (Size, Size),
        childs: Vec<PreparedElement>,
        direction: ContainerDirection,
        gap: u16,
        margin: u16,
        padding: u16,
        scrollable: Option<ContainerDirection>,
    },
}

impl PreparedElement {
    pub fn width(&self) -> &Size {
        match self {
            Self::Text(_, _, (width, _)) => width,
            Self::Element { size: (width, _), .. } => width,
        }
    }
    pub fn height(&self) -> &Size {
        match self {
            Self::Text(_, _, (_, height)) => height,
            Self::Element { size: (_, height), .. } => height,
        }
    }

    pub fn into_renderable(self, x: i32, y: i32, width: i32, height: i32) -> RenderableElement {
        match self {
            PreparedElement::Text(text, font_size, (width, height)) => RenderableElement::Text(
                text, font_size,
                BoundingBox { x, y, width: width.as_fixed(), height: height.as_fixed(), margin: 0, padding: 0 },
            ),
            PreparedElement::Element { childs, direction, gap, margin, padding, scrollable, .. } => {

                let context_x = x + margin as i32 + padding as i32;
                let context_y = y + margin as i32 + padding as i32;
                let content_width = width - margin as i32 * 2 - padding as i32 * 2;
                let content_height = height - margin as i32 * 2 - padding as i32 * 2;

                let left_over_width = content_width - childs.iter().fold(0, |acc, child| acc + child.width().without_expand());
                let dyn_size_alloc_number_width = childs.iter().filter(|child| child.width().is_expand()).count() as i32;
                let dyn_size_alloc_width = if dyn_size_alloc_number_width > 0 { left_over_width / dyn_size_alloc_number_width } else { 0 };

                let left_over_height = content_height - childs.iter().fold(0, |acc, child| acc + child.height().without_expand());
                let dyn_size_alloc_number_height = childs.iter().filter(|child| child.height().is_expand()).count() as i32;
                let dyn_size_alloc_height = if dyn_size_alloc_number_height > 0 { left_over_height / dyn_size_alloc_number_height } else { 0 };

                let mut current_x = context_x;
                let mut current_y = context_y;

                let childs: Vec<RenderableElement> = childs.into_iter().map(|child| {
                    let actual_child_width = child.width().fixed_or(match direction {
                        ContainerDirection::Horizontal => dyn_size_alloc_width,
                        ContainerDirection::Vertical => content_width
                    });
                    let actual_child_height = child.height().fixed_or(match direction {
                        ContainerDirection::Horizontal => content_height,
                        ContainerDirection::Vertical => dyn_size_alloc_height
                    });

                    let new = child.into_renderable(
                        current_x, current_y,
                        actual_child_width, actual_child_height
                    );

                    match direction {
                        ContainerDirection::Horizontal => current_x += actual_child_width + gap as i32,
                        ContainerDirection::Vertical => current_y += actual_child_height + gap as i32,
                    }

                    new
                }).collect();
                
                RenderableElement::Element {
                    bounding_box: BoundingBox { x, y, width, height, margin: margin as i32, padding: padding as i32 },
                    childs, scrollable,
                }
            }
        }
    }
}