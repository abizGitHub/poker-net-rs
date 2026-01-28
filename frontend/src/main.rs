use crate::{components::room::Room, contexts::game_state::ContextHolder};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::use_websocket;
mod components;
mod contexts;

#[function_component]
fn App() -> Html {
    let ctx = use_state(ContextHolder::default);
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:9001".to_string());
    let cloned_ws = ws.clone();

    let mut cloned_messages = messages.clone();
    let ctx_cloned = ctx.clone();
    use_effect_with(ws.message, move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            cloned_messages.push(ws_msg.clone());
            messages_handle.set(cloned_messages);
            let cloned = ws_msg.clone();
            if cloned.starts_with("table_id") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                ctx_cloned.table_id = cloned.split("::").skip(1).next().unwrap().to_string();
                ctx.set((ctx_cloned).clone());
            }
            if cloned.starts_with("user_id") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                ctx_cloned.user_id = cloned.split("::").skip(1).next().unwrap().to_string();
                ctx.set((ctx_cloned).clone());
            }
            if cloned.starts_with("players") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                ctx_cloned.players = cloned
                    .split("::")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect();
                ctx.set((ctx_cloned).clone());
            }
            if cloned.starts_with("game") {
                let ctx = ctx_cloned.clone();
                let mut ctx_cloned = (*ctx_cloned).clone();
                ctx_cloned.game_state = cloned.split("::").skip(1).next().unwrap().to_string();
                ctx.set((ctx_cloned).clone());
            }
        }
    });

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();

        if let Some(textarea) = target {
            cloned_new_message_handle.set(textarea.value());
        }
    });

    let cloned_new_message = new_message.clone();
    let on_button_click = Callback::from(move |_: MouseEvent| {
        cloned_ws.send(cloned_new_message.clone());
        new_message_handle.set("".to_string());
    });

    html! {
      <>
         <ul id="chat">
             {messages
                .iter()
                .map(|m| html!{<li> {m} </li>})
                .collect::<Html>()}
         </ul>
         <textarea onchange={on_message_change} value={new_message}/>
         <button type="submit" onclick={on_button_click}>{"send"}</button>
         <components::card::Card rank="A" suit ="♠"/>
      
         <ContextProvider<ContextHolder> context={(*ctx).clone()}>
            <Room/>
         </ContextProvider<ContextHolder>>
      </>
     }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
