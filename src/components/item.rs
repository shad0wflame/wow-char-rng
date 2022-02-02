use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ItemProps {
    pub text: String,
    pub icon: String,
    pub class_name: String,
    pub on_click: Option<Callback<MouseEvent>>,
}

#[function_component(Item)]
pub fn item(ItemProps { text, icon, class_name, on_click} : &ItemProps) -> Html {

    let default_fn = Callback::from(|e: MouseEvent| { e.prevent_default() });

    let callback = on_click.clone().unwrap_or_else(|| default_fn);

    html! {
         <div class="item">
            <span class={classes!("icon", class_name)}
                  style={format!("background-position: {}", icon)}
                  onclick={callback}
            />
            <p class="title"> {text} </p>
        </div>
    }
}


