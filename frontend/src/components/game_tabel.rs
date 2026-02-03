use yew::prelude::*;

use crate::{components::card::CardUi, contexts::game_state::ContextHolder};

#[function_component(GameTable)]
pub fn game_table() -> Html {
    let ctx = use_context::<ContextHolder>().unwrap();

    let cards = ctx
        .cards_on_table
        .iter()
        .map(|c| {
            c.to_string();
            let rank = c.rank.to_string();
            let suit = c.suit.to_string();
            html! {<CardUi {rank} {suit} />}
        })
        .collect::<Html>();

    html!(
        <div class="a-row">
         {cards}
        </div>
    )
}
