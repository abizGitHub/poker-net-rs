
use wasm_bindgen::prelude::wasm_bindgen;

use crate::game_base::tmp_two_player;
mod game_base;

#[wasm_bindgen]
pub fn on_call(event_type: &str) -> Option<Vec<String>> {
    match event_type {
        "deal" => Some(tmp_two_player()),
        _ => None//vec![format!("{event_type}")]
    }
}

#[cfg(test)]
mod tests {
    use crate::game_base::tmp_two_player;


    #[test]
    fn it_works() {
        assert!(tmp_two_player().len() > 6);
    }
}
