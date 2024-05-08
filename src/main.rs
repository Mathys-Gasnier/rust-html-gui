use std::error::Error;

use application::Application;
use serde::Serialize;

mod consts;
mod element;
mod loader;
mod application;
mod format;
mod math;

#[derive(Serialize)]
struct Context {
    app_name: String,
    counter: u128,
    projects: Vec<Project>
}

#[derive(Serialize)]
struct Project {
    name: String
}

impl Default for Context {
    fn default() -> Self {
        Self {
            app_name: String::from("Demo App"),
            counter: 0,
            projects: vec![
                Project { name: String::from("Project One") },
                Project { name: String::from("Project Two") },
                Project { name: String::from("Project Three") },
            ],
        }
    }
}

impl Application for Context {
    fn update(self: &mut Self) {
        self.counter += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Context::render("./gui");

    Ok(())
}