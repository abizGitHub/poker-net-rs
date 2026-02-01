use yew::prelude::*;

use crate::components::room::Props;

#[function_component(Hand)]
pub fn hand(props: &Props) -> Html {
    let all_in = props.click_types.all_in.clone();
    let check = props.click_types.check.clone();
    let fold = props.click_types.fold.clone();
    let raise = props.click_types.raise.clone();
    html!(<section class="a-row">
             <button class="btns" onclick={all_in}>{"All-In🚀"}</button>

             <button class="btns" onclick={raise}>{"🔺"}</button>

             <button class="btns" onclick={check}>{"👍"}</button>

             <button class="btns" onclick={fold}>{"👎"}</button>
         </section>
    )
}
