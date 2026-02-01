use yew::{Callback, Html, MouseEvent, Properties, html, prelude::*};

#[derive(Properties, PartialEq)]
pub struct FirstPageProps {
    pub set_a_table: Callback<MouseEvent>,
}

#[derive(PartialEq, Clone)]
pub struct TableCall {
    pub table_id: String,
    pub callback: Callback<MouseEvent>,
}

#[function_component(FirstPage)]
pub fn first_page(props: &FirstPageProps) -> Html {
    let set_a_table = props.set_a_table.clone();
    let tables_list = use_context::<Vec<TableCall>>().unwrap();

    html! {
        <div>
          <button class="btns" onclick={set_a_table}>{"start a game"}</button>
          <br/>{"or"}<br/>
          {"join a table"}<br/>
          {
            tables_list.into_iter().map(|t:TableCall|{
              html!{
             <div>
             <button class="btns" onclick={t.callback} >{format!("{}",t.table_id)}</button>
             </div>
                }
            }).collect::<Html>()
          }
        </div>
    }
}
