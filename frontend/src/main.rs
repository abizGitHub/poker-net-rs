use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();

    let ws = use_websocket("ws://127.0.0.1:9001".to_string());
    let cloned_ws = ws.clone();  

    let mut cloned_messages = messages.clone();

    use_effect_with(ws.message, move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            cloned_messages.push(ws_msg.clone());
            messages_handle.set(cloned_messages);
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
    let on_button_click = Callback::from(move |_:MouseEvent|{
        cloned_ws.send(cloned_new_message.clone());  
        new_message_handle.set("".to_string());
    });

    html! {
     <>
        <ul id="chat">
            { messages.iter().map(|m| html!{
                       <li> {m} </li>
            }).collect::<Html>()}
        </ul>
        <textarea onchange={on_message_change} value={new_message}/>
        <button type="submit" onclick={on_button_click}>{"send"}</button>
     </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
