use std::error::Error;

use application::{Application, Handle};
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
    counter: u16,
    projects: Vec<Project>
}

#[derive(Serialize)]
struct Project {
    id: u16,
    name: String
}

impl Default for Context {
    fn default() -> Self {
        Self {
            app_name: String::from("Demo App"),
            counter: 0,
            projects: vec![],
        }
    }
}

impl Application for Context {
    fn update(self: &mut Self, handle: &Handle) {
        if handle.is_clicked("add-project") {
            self.projects.push(Project { id: self.counter, name: format!("Project {}", self.counter) });
            self.counter += 1;
        }

        self.projects.retain(|project| !handle.is_clicked(&format!("delete-{}", project.id)));

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Context::render("./gui");

    Ok(())
}