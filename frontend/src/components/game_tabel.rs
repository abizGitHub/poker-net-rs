use common::GameResult;
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

    let game_result = match ctx.result {
        Some(g_result) => {
            let r = match g_result {
                GameResult::Winner(w) => html! {
                    <tr class="players_header">
                        <td> {"winner"} </td>
                        <td> {w.id} </td>
                        <td> {format!("{:?}", w.rank)}    </td>
                    </tr>
                },
                GameResult::Tie(p1, p2) => html! {
                    <>
                    <tr class="players_header">
                        <td> {"tie"} </td>
                        <td> {p1.id} </td>
                        <td> {format!("{:?}", p1.rank)}    </td>
                    </tr>
                    <tr class="players_header">
                        <td> </td>
                        <td> {p2.id} </td>
                        <td> {format!("{:?}", p2.rank)}    </td>
                    </tr>
                    </>
                },
            };
            html! {
              <>
                <table>
                    <tbody>
                     <th>{"Game Result"}</th>
                     {r}
                    </tbody>
                </table>
              </>
            }
        }
        None => html!(),
    };

    html!(
        <div class="a-row">
         {game_result}
         {cards}
        </div>
    )
}
