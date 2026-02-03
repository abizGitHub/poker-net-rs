use common::GameState;
use yew::prelude::*;

use crate::{
    components::{card::CardUi, room::Props},
    contexts::game_state::ContextHolder,
};

#[function_component(Hand)]
pub fn hand(props: &Props) -> Html {
    let all_in = props.click_types.all_in.clone();
    let check = props.click_types.check.clone();
    let fold = props.click_types.fold.clone();
    let raise = props.click_types.raise.clone();

    let ctx = use_context::<ContextHolder>().unwrap();

    let cards = match ctx.players.iter().find(|p| ctx.user_id == p.id) {
        Some(user) => user
            .hand
            .iter()
            .map(|c| {
                c.to_string();
                let rank = c.rank.to_string();
                let suit = c.suit.to_string();
                html! {<CardUi {rank} {suit} />}
            })
            .collect::<Html>(),
        None => html!(),
    };

    let left_side = match ctx.game_state {
    
        GameState::PreDeal | GameState::Blinds | GameState::PreFlop => html!(),
    
        _ => html!{<>
             <button class="btns" onclick={all_in}>{"All-In🚀"}</button>
             <button class="btns" onclick={raise}>{"🔺"}</button>
            </>},
    };

    let right_side = match ctx.game_state {
    
        GameState::Ended => html!(),

        _ => html!{<>
             <button class="btns" onclick={check}>{"👍"}</button>
             <button class="btns" onclick={fold}>{"👎"}</button>
            </>},
    };

    html!(<section class="a-row">
             {left_side}             
             {cards}
             {right_side}             
         </section>)
}
