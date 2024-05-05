use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, math::Vector2, text::RaylibFont, RaylibHandle};
use crate::element::{container_direction::ContainerDirection, container_expand::ContainerExpand, Element};

trait Renderable {
    fn render(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, width: i32, height: i32);
    fn size(&self, rl: &RaylibHandle) -> (Option<i32>, Option<i32>);
}

impl Renderable for Element {
    fn render(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, width: i32, height: i32) {

        match self {
            Element::Window { childs, direction, gap, margin, padding, .. } |
            Element::Container { childs, direction, gap, margin, padding, .. } |
            Element::Button { childs, direction, gap, margin, padding, .. } => {

                let border_x = x + *margin as i32;
                let border_y = y + *margin as i32;
                let border_width = width - *margin as i32 * 2;
                let border_height = height - *margin as i32 * 2;

                #[cfg(debug_assertions)]
                d.draw_rectangle_lines(border_x, border_y, border_width, border_height, Color::BLACK);

                let content_x = border_x + *padding as i32;
                let content_y = border_y + *padding as i32;
                let content_width = border_width - *padding as i32 * 2;
                let content_height = border_height - *padding as i32 * 2;

                let child_sizes: Vec<(&Element, ( Option<i32>, Option<i32>))> = childs.iter().map(|child| (child, child.size(&d))).collect();
                
                let left_over_width = content_width - child_sizes.iter().fold(0, |acc, (_, (width, _))| acc + width.unwrap_or(0));
                let dyn_size_alloc_number_width = child_sizes.iter().filter(|(_, (width, _))| width.is_none()).count() as i32;
                let dyn_size_alloc_width = if dyn_size_alloc_number_width > 0 { left_over_width / dyn_size_alloc_number_width } else { 0 };

                let left_over_height = content_height - child_sizes.iter().fold(0, |acc, (_, (_, height))| acc + height.unwrap_or(0));
                let dyn_size_alloc_number_height = child_sizes.iter().filter(|(_, (_, height))| height.is_none()).count() as i32;
                let dyn_size_alloc_height = if dyn_size_alloc_number_height > 0 { left_over_height / dyn_size_alloc_number_height } else { 0 };

                let mut draw_x = content_x;
                let mut draw_y = content_y;
                for (child, ( child_width, child_height )) in child_sizes {
                    let actual_child_width = child_width.unwrap_or(match direction {
                        ContainerDirection::Horizontal => dyn_size_alloc_width,
                        ContainerDirection::Vertical => content_width
                    });
                    let actual_child_height = child_height.unwrap_or(match direction {
                        ContainerDirection::Horizontal => content_height,
                        ContainerDirection::Vertical => dyn_size_alloc_height
                    });

                    child.render(
                        d,
                        draw_x, draw_y,
                        actual_child_width, actual_child_height,
                    );

                    match direction {
                        ContainerDirection::Horizontal => draw_x += actual_child_width + *gap as i32,
                        ContainerDirection::Vertical => draw_y += actual_child_height + *gap as i32,
                    }
                }
            },
            Element::Text { text, font_size, margin, padding } => {

                #[cfg(debug_assertions)]
                d.draw_rectangle_lines(x, y, width, height, Color::RED);

                d.draw_text_ex(
                    d.get_font_default(),
                    text,
                    Vector2::new(x as f32 + *margin as f32 + *padding as f32, y as f32 + *margin as f32 + *padding as f32),
                    *font_size as f32,
                    if *font_size > 10 { *font_size as f32 / 10.0 } else { 1.0 },
                    Color::BLACK
                );
            }
            Element::Space => {},
        }
    }
    
    fn size(&self, rl: &RaylibHandle) -> (Option<i32>, Option<i32>) {
        match self {
            Element::Window { .. } => (None, None),
            Element::Container { childs, expand, direction, gap, margin, padding } |
            Element::Button { childs, expand, direction, gap, margin, padding } => {
                let child_sizes: Vec<( Option<i32>, Option<i32>)> = childs.iter().map(|child| child.size(rl)).collect();

                let inner_width = match direction {
                    ContainerDirection::Horizontal => child_sizes.iter().fold(0, |acc, (width, _)| acc + width.unwrap_or(0) + *gap as i32) - *gap as i32,
                    ContainerDirection::Vertical => child_sizes.iter().map(|(width, _)| width.unwrap_or(0)).max().unwrap_or(0),
                } + (*margin as i32 * 2) + (*padding as i32 * 2);
                let inner_height = match direction {
                    ContainerDirection::Horizontal => child_sizes.iter().map(|(_, height)| height.unwrap_or(0)).max().unwrap_or(0),
                    ContainerDirection::Vertical => child_sizes.iter().fold(0, |acc, (_, height)| acc + height.unwrap_or(0) + *gap as i32) - *gap as i32,
                } + (*margin as i32 * 2) + (*padding as i32 * 2);

                match expand {
                    Some(ContainerExpand::All) => (None, None),
                    Some(ContainerExpand::Width) => (None, Some(inner_height)),
                    Some(ContainerExpand::Height) => (Some(inner_width), None),
                    None => (Some(inner_width), Some(inner_height)),
                }
            },
            Element::Space => (None, None),
            Element::Text { text, font_size, margin, padding } => {
                let text_size = rl.get_font_default().measure_text(
                    &text,
                    *font_size as f32,
                    if *font_size > 10 { *font_size as f32 / 10.0 } else { 1.0 }
                );
                (Some(text_size.x as i32 + (*padding as i32 * 2) + (*margin as i32 * 2)), Some(text_size.y as i32 + (*padding as i32 * 2) + (*margin as i32 * 2)))
            },
        }
    }
}

pub fn render(window: Element) {
    let (mut rl, thread) = raylib::init()
        .size(1200, 675)
        .title("GUI")
        .build();

    while !rl.window_should_close() {
        let viewport_width = rl.get_screen_width();
        let viewport_height = rl.get_screen_height();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        
        window.render(&mut d, 0, 0, viewport_width, viewport_height);
    }
}
