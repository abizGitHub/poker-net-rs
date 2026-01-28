use yew::prelude::*;

use crate::contexts::game_state::ContextHolder;

#[function_component(User)]
pub fn user() -> Html {
    let user_id = use_context::<ContextHolder>().unwrap().user_id;
    html!(
        <div> {user_id} </div>
    )
}
