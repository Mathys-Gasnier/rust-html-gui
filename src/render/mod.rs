use raylib::{color::Color, drawing::RaylibDraw};
use serde::Serialize;
use serde_json::Value;
use crate::element::Element;

use self::renderable::Renderable;

mod size;
mod renderable;
mod format;

pub fn render<Context: Serialize>(window: Element, context: &Context) {
    let (mut rl, thread) = raylib::init()
        .size(1200, 675)
        .title("GUI")
        .build();

    while !rl.window_should_close() {
        let viewport_width = rl.get_screen_width();
        let viewport_height = rl.get_screen_height();

        let context_value = serde_json::to_value(context).expect("Failed to convert context to value");

        assert!(matches!(context_value, Value::Object(..)), "Context must be a struct or object like !");

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        
        window.render(&mut d, &context_value, 0, 0, viewport_width, viewport_height);
    }
}
