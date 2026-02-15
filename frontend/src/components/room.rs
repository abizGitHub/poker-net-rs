use yew::prelude::*;
use yew_hooks::UseWebSocketHandle;

use crate::components::{game_tabel::GameTable, hand::Hand, players::Players, user::User};

#[derive(PartialEq, Clone)]
pub struct MouseClikType {
    pub raise: Callback<MouseEvent>,
    pub check: Callback<MouseEvent>,
    pub all_in: Callback<MouseEvent>,
    pub fold: Callback<MouseEvent>,
}

impl MouseClikType {
    pub fn new(ws: &UseWebSocketHandle) -> Self {
        let cloned1_ws = ws.clone();
        let cloned2_ws = ws.clone();
        let cloned3_ws = ws.clone();
        let cloned4_ws = ws.clone();
        Self {            
            all_in: Callback::from(move |_: MouseEvent| {
                cloned1_ws.send(format!("all_in"));
            }),
            raise: Callback::from(move |_: MouseEvent| {
                cloned2_ws.send(format!("raise"));
            }),
            check: Callback::from(move |_: MouseEvent| {
                cloned3_ws.send(format!("ready"));
            }),
            fold: Callback::from(move |_: MouseEvent| {
                cloned4_ws.send(format!("fold"));
            }),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub click_types: MouseClikType,
}

#[function_component(Room)]
pub fn room(props: &Props) -> Html {
    let click_types = props.click_types.clone();
    html! {
    <div class="container">
    <table class="table">

        <tbody>
            <tr>
                <td class="players">
                    <Players />
                </td>
                <td>
                    <GameTable />
                </td>
            </tr>
            <tr class="narrow_row">
                <td>
                    <User />
                </td>
                <td>
                    <Hand {click_types}/>
                </td>
            </tr>
        </tbody>

    </table>
    </div>}
}
