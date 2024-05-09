use serde::Serialize;
use raylib::{color::Color, drawing::RaylibDraw, RaylibHandle};
use serde_json::Value;

use crate::loader::Loader;

pub trait Application: Serialize + Default {

    fn render(dir: &str) {
        let loader = Loader::new(dir);

        let window = loader.get("main.gui");
    
        let mut context = Self::default();
        let mut handle = Handle::new();
    
        let (mut rl, thread) = raylib::init()
            .size(1200, 675)
            .title("GUI")
            .build();

        while !rl.window_should_close() {
            let viewport_width = rl.get_screen_width();
            let viewport_height = rl.get_screen_height();

            context.update(&handle);

            let context_value = serde_json::to_value(&context).expect("Failed to convert context to value");

            assert!(matches!(context_value, Value::Object(..)), "Context must be a struct or object like !");

            // Compute logic
            let expanded = window.clone().expand(&rl, &context_value)[0].clone();
            // Compute wanted sizes
            let with_wanted_size = expanded.with_wanted_size(&rl, None, None);
            // compute actual sizes
            let renderable = with_wanted_size.into_renderable(0, 0, viewport_width, viewport_height);
            handle.update(
                &rl,
                renderable.ids_at_position(rl.get_mouse_position())
            );

            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            
            // window.render(&mut d, &context_value, 0, 0, viewport_width, viewport_height);
            renderable.render(&mut d);
        }
    }

    fn update(self: &mut Self, handle: &Handle);
}

pub struct Handle {
    cursor_ids: Vec<String>,
    clicked: bool,
    pressed: bool,
}

impl Handle {

    fn new() -> Self {
        Self {
            cursor_ids: vec![],
            clicked: false,
            pressed: false
        }
    }

    fn update(&mut self, rl: &RaylibHandle, cursor_ids: Vec<String>) {
        self.cursor_ids = cursor_ids;

        self.clicked = rl.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT);
        self.pressed = rl.is_mouse_button_down(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT);
    }

    pub fn is_hovered(&self, id: &str) -> bool {
        self.cursor_ids.contains(&id.to_owned())
    }

    pub fn is_clicked(&self, id: &str) -> bool {
        self.clicked && self.cursor_ids.contains(&id.to_owned())
    }

    pub fn is_pressed(&self, id: &str) -> bool {
        self.pressed && self.cursor_ids.contains(&id.to_owned())
    }
}