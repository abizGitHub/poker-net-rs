use wasm_bindgen::{JsCast, prelude::wasm_bindgen};

use crate::game_base::tmp_two_player;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, js_sys};

mod game_base;

#[wasm_bindgen]
pub fn on_call(event_type: &str) -> Option<Vec<String>> {
    match event_type {
        "deal" => Some(tmp_two_player()),
        _ => None, //vec![format!("{event_type}")]
    }
}

#[wasm_bindgen]
pub fn set_canvas(canvas: HtmlCanvasElement) {
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.fill_rect(331 as f64, 35 as f64, 110 as f64, 100 as f64);
}

#[cfg(test)]
mod tests {
    use crate::game_base::tmp_two_player;

    #[test]
    fn it_works() {
        assert!(tmp_two_player().len() > 6);
    }
}
