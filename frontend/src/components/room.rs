use yew::prelude::*;

use crate::components::{game_tabel::GameTable, hand::Hand, players::Players, user::User};

#[derive(PartialEq, Clone)]
pub struct MouseClikType {
    pub raise: Callback<MouseEvent>,
    pub check: Callback<MouseEvent>,
    pub all_in: Callback<MouseEvent>,
    pub fold: Callback<MouseEvent>,
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
