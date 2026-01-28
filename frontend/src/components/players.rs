use yew::prelude::*;

use crate::contexts::game_state::ContextHolder;

#[function_component(Players)]
pub fn players() -> Html {
    let ctx = use_context::<ContextHolder>().unwrap();

    html! {
       <div>
         <table>
             <tbody>
                 <tr class="players_header">
                    <td> {"players"} </td>
                 </tr>
                  {ctx
                    .players
                    .iter()
                    .map(|p| html!{<tr><td> {p} </td></tr>})
                    .collect::<Html>()}
             </tbody>
          </table>
        </div>
    }
}
