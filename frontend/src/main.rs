use crate::{
    components::{
        first_page::{FirstPage, TableCall},
        room::{MouseClikType, Room},
    },
    contexts::game_state::ContextHolder,
};
use common::{Card, GameState, Player};
use yew::prelude::*;
use yew_hooks::{UseWebSocketReadyState, use_websocket};
mod components;
mod contexts;

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:9001".to_string());
    let cloned_ws = ws.clone();

    let cloned1_ws = ws.clone();
    let cloned2_ws = ws.clone();
    let cloned3_ws = ws.clone();
    let cloned4_ws = ws.clone();
    let cloned5_ws = ws.clone();
    let cloned6_ws = ws.clone();
    let cloned7_ws = ws.clone();
    let cloned8_ws = ws.clone();

    let mut cloned_messages = messages.clone();

    let ctx = use_state(ContextHolder::default);
    let tables_list = use_state(|| Vec::<TableCall>::new());
    let tables_list_cloned = tables_list.clone();
    let click_types = MouseClikType {
        all_in: Callback::from(move |_: MouseEvent| {
            cloned1_ws.send(format!("all_in"));
        }),
        raise: Callback::from(move |_: MouseEvent| {
            cloned2_ws.send(format!("raise"));
        }),
        check: Callback::from(move |_: MouseEvent| {
            cloned4_ws.send(format!("ready"));
        }),
        fold: Callback::from(move |_: MouseEvent| {
            cloned3_ws.send(format!("fold"));
        }),
    };

    let set_a_table = Callback::from(move |_: MouseEvent| {
        cloned5_ws.send(format!("set_a_table"));
    });

    use_effect_with(cloned7_ws.ready_state, move |state| {
        if **state == UseWebSocketReadyState::Open {
            cloned8_ws.send("all_tables".to_string());
        };
    });

    let ctx_cloned = ctx.clone();
    use_effect_with(ws.message, move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            cloned_messages.push(ws_msg.clone());
            messages_handle.set(cloned_messages);
            let cloned = ws_msg.clone();
            if cloned.starts_with("table_id") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                let table_id = cloned.split("::").skip(1).next().unwrap().to_string();
                ctx_cloned.table_id = table_id.clone();
                ctx.set(ctx_cloned);
                cloned6_ws.send(format!("add_player_to_table::{table_id}"));
            }
            if cloned.starts_with("user_id") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                ctx_cloned.user_id = cloned.split("::").skip(1).next().unwrap().to_string();
                ctx.set(ctx_cloned);
            }
            if cloned.starts_with("players") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                ctx_cloned.players =
                    serde_json::from_str::<Vec<Player>>(cloned.split("::").skip(1).next().unwrap())
                        .unwrap()
                        .into_iter()                        
                        .collect();
                ctx.set(ctx_cloned);
            }
            if cloned.starts_with("game") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                ctx_cloned.game_state = serde_json::from_str::<GameState>(cloned.split("::").skip(1).next().unwrap()).unwrap();
                ctx.set(ctx_cloned);
            }
            if cloned.starts_with("all_tables") {
                let list =
                    serde_json::from_str::<Vec<String>>(cloned.split("::").skip(1).next().unwrap())
                        .unwrap()
                        .into_iter()
                        .map(|table_id| {
                            let ws = cloned_ws.clone();
                            TableCall {
                                table_id: table_id.clone(),
                                callback: Callback::from(move |_: MouseEvent| {
                                    ws.send(format!("join_to_table::{}", table_id.clone()));
                                }),
                            }
                        })
                        .collect();

                tables_list_cloned.set(list);
            }
            if cloned.starts_with("table::") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                ctx_cloned.cards_on_table =
                    serde_json::from_str::<Vec<Card>>(cloned.split("::").skip(1).next().unwrap())
                        .unwrap();

                ctx.set(ctx_cloned);
            }
        }
    });
    if ctx.table_id.is_empty() {
        html! {
            <>
            <ContextProvider<Vec<TableCall>> context={(*tables_list).clone()}>
            <FirstPage {set_a_table}/>
            </ContextProvider<Vec<TableCall>>>
            </>
        }
    } else {
        let log = html! {
            <ul id="chat">
                {messages
                   .iter()
                   .map(|m| html!{<li> {m} </li>})
                   .collect::<Html>()}
            </ul>
        };
        html! {
         <>
           {log}
            <ContextProvider<ContextHolder> context={(*ctx).clone()}>
               <Room {click_types}/>
            </ContextProvider<ContextHolder>>
         </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
