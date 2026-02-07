use yew::prelude::*;

use crate::contexts::game_state::ContextHolder;

#[function_component(Players)]
pub fn players() -> Html {
    let ctx = use_context::<ContextHolder>().unwrap();

    html! {
      <>
       <div class="game_state">
         {format!("{:?}", ctx.game_state)}
       </div>
       <div>
         <table>
             <tbody>
                 <tr class="players_header">
                    <td> {"players"} </td>
                    <td> {"status"} </td>
                 </tr>
                  {ctx
                    .players
                    .iter()
                    .filter(|p| p.id != ctx.user_id)
                    .map(|p| html!{
                      <tr>
                        <td>{p.id.clone()}</td>
                        <td>{format!("{:?}", p.state)}</td>
                      </tr>
                    })
                    .collect::<Html>()}
             </tbody>
          </table>
        </div>
        </>
    }
}
