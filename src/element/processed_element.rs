use raylib::{text::RaylibFont, RaylibHandle};

use crate::{consts::DEFAULT_FONT_SIZE, math::size::Size};

use super::{container_direction::ContainerDirection, container_expand::ContainerExpand, prepared_element::PreparedElement};



#[derive(Debug, Clone)]
pub enum ProcessedElement {
    Text(String),
    Element {
        childs: Vec<ProcessedElement>,
        expand: Option<ContainerExpand>,
        direction: ContainerDirection,
        gap: u16,
        margin: u16,
        padding: u16,
        scrollable: Option<ContainerDirection>,
        font_size: Option<u8>,
    },
}

impl ProcessedElement {
    pub fn with_wanted_size(self, rl: &RaylibHandle, parent_font_size: Option<u8>) -> PreparedElement {
        match self {
            Self::Text(text) => {
                let actual_font_size = parent_font_size.unwrap_or(DEFAULT_FONT_SIZE);
                let text_size = rl.get_font_default().measure_text(
                    &text,
                    actual_font_size as f32,
                    if actual_font_size > 10 { actual_font_size as f32 / 10.0 } else { 1.0 }
                );

                PreparedElement::Text(text, actual_font_size, ( Size::Fixed(text_size.x as i32), Size::Fixed(text_size.y as i32) ))
            },
            Self::Element { childs, expand, direction, gap, margin, padding, scrollable, font_size } => {
                let childs: Vec<PreparedElement> = childs.into_iter()
                    .map(|child|
                        child.with_wanted_size(rl, font_size.or(parent_font_size))
                    ).collect();

                let margin_and_padding = margin * 2 + padding * 2;

                let inner_width = match direction {
                    ContainerDirection::Horizontal => childs.iter().fold(0, |acc, child| acc + child.width().without_expand() + gap as i32) - gap as i32,
                    ContainerDirection::Vertical => childs.iter().map(|child| child.width().without_expand()).max().unwrap_or(0),
                } + margin_and_padding as i32;
                
                let inner_height = match direction {
                    ContainerDirection::Horizontal => childs.iter().map(|child| child.height().without_expand()).max().unwrap_or(0),
                    ContainerDirection::Vertical => childs.iter().fold(0, |acc, child| acc + child.height().without_expand() + gap as i32) - gap as i32,
                } + margin_and_padding as i32;

                PreparedElement::Element {
                    size: match expand {
                        Some(ContainerExpand::All) => (Size::Expand(inner_width), Size::Expand(inner_height)),
                        Some(ContainerExpand::Width) => (Size::Expand(inner_width), Size::Fixed(inner_height)),
                        Some(ContainerExpand::Height) => (Size::Fixed(inner_width), Size::Expand(inner_height)),
                        None => (Size::Fixed(inner_width), Size::Fixed(inner_height)),
                    },
                    childs, direction, gap, margin, padding, scrollable,
                }
            }
        }
    }
}
