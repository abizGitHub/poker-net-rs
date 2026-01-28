use yew::prelude::*;

use crate::components::{game_tabel::GameTable, hand::Hand, players::Players, user::User};

#[function_component(Room)]
pub fn room() -> Html {
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
                    <Hand />
                </td>
            </tr>
        </tbody>

    </table>
    </div>}
}
