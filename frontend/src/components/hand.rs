use yew::prelude::*;

#[function_component(Hand)]
pub fn hand() -> Html {
    html!(
     <section class="a-row">
             <button class="btns">{"🚀"}</button>

             <button class="btns">{"🔺"}</button>

             <button class="btns">{"👍"}</button>

             <button class="btns">{"👎"}</button>
         </section>
    )
}
