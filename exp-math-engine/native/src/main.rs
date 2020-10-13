pub mod server;
pub mod engine;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate neon;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

use engine::{MathStack, Engine};

fn main() {
    let mut engine = Engine::new();
    println!("{}", serde_json::to_string(&engine.create_new_math_stack()).unwrap_or("Could not unwrap".to_string()));
}