use std::sync::Arc;
use std::sync::Mutex;
use crate::engine::{Engine, MathExpression};

pub struct Server {
    engine: Engine
}

impl Server {
    pub fn new() -> Server {
        Server {
            engine: Engine::new()
        }
    }

    pub fn create_new_math_stack(&mut self) -> MathExpression {
        self.engine.create_new_math_stack()
    }
}

lazy_static!{
    pub static ref SERVER: Mutex<Server> = {
        Mutex::new(Server::new())
    };
}