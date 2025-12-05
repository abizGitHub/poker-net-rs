use wasm_bindgen::{prelude::wasm_bindgen};

mod backend;

use crate::backend::casino;

#[wasm_bindgen]
pub fn set_a_table() -> String {
    casino::set_a_table()
}

#[wasm_bindgen]
pub fn on_call(event_type: &str) -> Option<Vec<String>> {
    match event_type {
        "deal" => None,
        _ => None, //vec![format!("{event_type}")]
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
