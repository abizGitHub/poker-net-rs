use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub rank: AttrValue,
    pub suit: AttrValue,
}

#[function_component(Card)]
pub fn card(c: &CardProps) -> Html {
    let (card_state, color) = match c.suit.clone().as_str() {
        "♥" | "♦" => ("card", "red"),
        "♣" | "♠" => ("card", "black"),
        _ => ("mock", ""),
    };

    html! {
       <>
         <div class={card_state}>
            <div class={ format!("corner top-left {color}")}>{c.rank.clone()}{c.suit.clone()}</div>
            <div class={ format!("suit-big {color}")}>{c.suit.clone()}</div>
            <div class={ format!("corner bottom-right {color}")}>{c.rank.clone()}{c.suit.clone()}</div>
         </div>
       </>
    }
}
