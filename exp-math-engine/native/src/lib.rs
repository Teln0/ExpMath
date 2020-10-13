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

use neon::prelude::*;

fn start_server(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    Ok(cx.boolean(true))
}

fn create_new_math_stack(mut cx: FunctionContext) -> JsResult<JsValue> {
    let expression = server::SERVER.lock().unwrap().create_new_math_stack();
    Ok(neon_serde::to_value(&mut cx, &expression).unwrap_or(cx.undefined().upcast()))
}

register_module!(mut cx, {
    cx.export_function("start_server", start_server)?;
    cx.export_function("create_new_math_stack", create_new_math_stack)
});
