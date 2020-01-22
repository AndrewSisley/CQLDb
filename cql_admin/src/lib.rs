use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::panic;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn start_app() -> JsValue {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let axis_definitions: [cql_db::AxisDefinition; 2] = [
        cql_db::AxisDefinition { id: 1, max: 2 },
        cql_db::AxisDefinition { id: 2, max: 2 }
        
            ];

    cql_db::create_db::<Option<f64>>("./cql/", &axis_definitions);

    let app_model = "<div>hello from rust</div>";

    return JsValue::from_serde(&app_model).unwrap();
}
