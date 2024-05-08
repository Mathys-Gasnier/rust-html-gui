use serde::Serialize;
use raylib::{color::Color, drawing::RaylibDraw};
use serde_json::Value;

use crate::loader::Loader;

pub trait Application: Serialize + Default {

    fn render(dir: &str) {
        let loader = Loader::new(dir);

        let window = loader.get("main.gui");
    
        let mut context = Self::default();
    
        let (mut rl, thread) = raylib::init()
            .size(1200, 675)
            .title("GUI")
            .build();

        while !rl.window_should_close() {
            let viewport_width = rl.get_screen_width();
            let viewport_height = rl.get_screen_height();

            context.update();

            let context_value = serde_json::to_value(&context).expect("Failed to convert context to value");

            assert!(matches!(context_value, Value::Object(..)), "Context must be a struct or object like !");

            // Compute logic
            let expanded = window.clone().expand(&rl, &context_value)[0].clone();
            // Compute wanted sizes
            let with_wanted_size = expanded.with_wanted_size(&rl, None);
            // compute actual sizes
            let renderable = with_wanted_size.into_renderable(0, 0, viewport_width, viewport_height);

            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            
            // window.render(&mut d, &context_value, 0, 0, viewport_width, viewport_height);
            renderable.render(&mut d);
        }
    }

    fn update(self: &mut Self);
}