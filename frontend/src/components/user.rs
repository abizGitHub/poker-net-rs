use yew::prelude::*;

use crate::contexts::game_state::ContextHolder;

#[function_component(User)]
pub fn user() -> Html {
    let ctx = use_context::<ContextHolder>().unwrap();
    let user_state = match ctx.players.iter().find(|p| ctx.user_id == p.id) {
        Some(user) => html! {
            <>
             <td>{user.role_str()}</td>
             <td>{format!("{:?}", user.state)}</td>
            </>
        },
        None => html!(),
    };

    html!(
        <div>
          <td>{ctx.user_id}</td>
          {user_state}
        </div>
    )
}
