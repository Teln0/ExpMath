use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::borrow::Borrow;

#[derive(Clone, Serialize, Deserialize)]
pub enum MathExpressionValue {
    Number(i64),
    Variable(String),
    Factor(Vec<Arc<RwLock<MathExpression>>>),
    Sum(Vec<Arc<RwLock<MathExpression>>>)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MathExpression {
    value: MathExpressionValue,
    id: usize
}

impl MathExpression {
    pub fn new(value: MathExpressionValue) -> MathExpression {
        MathExpression {
            value,
            id: 0
        }
    }
}

pub struct MathExpressionManager {
    current_id: usize,
    all_expressions: HashMap<usize, Arc<RwLock<MathExpression>>>,
    root: Arc<RwLock<MathExpression>>
}

impl MathExpressionManager {
    pub fn generate_ids(&mut self, e: Arc<RwLock<MathExpression>>) {
        let mut expression = e.write().unwrap();
        expression.id = self.current_id;
        self.all_expressions.insert(self.current_id, e.clone());
        self.current_id += 1;

        match expression.value.clone() {
            MathExpressionValue::Factor(factor) => {
                std::mem::drop(expression);
                for i in factor {
                    self.generate_ids(i.clone());
                }
            }
            MathExpressionValue::Sum(sum) => {
                std::mem::drop(expression);
                for i in sum {
                    self.generate_ids(i.clone());
                }
            }
            _ => {
                // No inner IDS to generate
            }
        }
    }

    pub fn new(root: MathExpressionValue) -> MathExpressionManager {
        let root = MathExpression::new(root);
        let root = Arc::new(RwLock::new(root));

        let all_expressions = HashMap::new();

        let mut mem = MathExpressionManager {
            current_id: 0,
            all_expressions,
            root: root.clone()
        };

        mem.generate_ids(root.clone());

        mem
    }
}

pub struct MathStack {
    current: MathExpressionManager
}

impl MathStack {
    pub fn new() -> MathStack {
        MathStack {
            current: MathExpressionManager::new(MathExpressionValue::Factor(vec![
                Arc::new(RwLock::new(MathExpression::new(MathExpressionValue::Variable("x".to_string())))),
                Arc::new(RwLock::new(MathExpression::new(MathExpressionValue::Variable("y".to_string())))),
                Arc::new(RwLock::new(MathExpression::new(MathExpressionValue::Variable("w".to_string()))))
            ]))
        }
    }
}

pub struct Engine {
    math_stacks: Vec<MathStack>
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            math_stacks: vec![]
        }
    }

    pub fn create_new_math_stack(&mut self) -> MathExpression {
        self.math_stacks.push(MathStack::new());
        self.math_stacks.last().unwrap().current.root.deref().read().unwrap().deref().clone()
    }
}