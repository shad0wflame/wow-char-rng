mod components;
mod data;
mod state;

use crate::data::{get_random_class, get_random_icon, get_random_race, get_random_spec};
use crate::state::{Action, State};
use components::item::Item;
use gloo::console::__macro::JsValue;
use gloo_utils::body;
use rand::{thread_rng, Rng};
use web_sys::CssStyleDeclaration;
use yew::prelude::*;

fn set_rand_background(style: &CssStyleDeclaration) -> Result<(), JsValue> {
    let mut rng = thread_rng();
    let image_index: u8 = rng.gen_range(0..=4);

    style.set_property(
        "background-image",
        &format!("url(assets/images/backgrounds/{}.jpg)", image_index),
    )?;
    style.set_property("background-size", "cover")?;

    Ok(())
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(State::default);

    use_effect_with_deps(
        move |_| {
            match set_rand_background(&body().style()) {
                Err(e) => eprintln!("{:?}", e),
                _ => (),
            }

            // use_effect has to compulsorily return a closure, even if it's a no-op.
            || {}
        },
        ()
    );

    let reset_onclick = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::Reset))
    };

    html! {
        <div class="container h-100">
            <div class="row h-100 align-items-center justify-content-center">
                <div class="item-container">
                    <div class="row align-content-center h-25 justify-content-center">
                        <p class="title">
                            {"WoW Character Randomizer"}
                        </p>
                    </div>
                    <div class="row align-content-end h-50">
                        <div id="class" class="col-4">
                            <Item
                                text={state.selected_race.faction.clone()}
                                icon={"2px"}
                                class_name={format!(
                                    "faction {}",
                                    &state.selected_race.faction.to_lowercase()
                                )}
                            />
                        </div>
                        <div id="faction" class="col-4">
                            <Item
                                text={state.selected_race.name.clone()}
                                icon={get_random_icon(&state.selected_race)}
                                class_name={"race"}
                            />
                        </div>
                        <div id="race" class="col-4">
                            <Item
                                text={format!(
                                    "{} {}",
                                    get_random_spec(&state.selected_class),
                                    &state.selected_class.name
                                )}
                                icon={state.selected_class.icon.clone()}
                                class_name={"class"}
                            />
                        </div>
                    </div>
                    <div class="row align-content-end h-25">
                        <div class="offset-md-4 col-4">
                            <button type="button"
                                    class="d-block mx-auto wow-btn"
                                    onclick={reset_onclick}
                            >
                                {"Repeat!"}
                            </button>
                        </div>
                        <div class="col-4">
                            /*
                            <button id="filter-btn"
                                    type="button"
                                    class="d-block mx-auto wow-btn"
                                    data-toggle="modal"
                                    data-target="#filterModal"
                            >
                                {"Filters"}
                            </button>
                            */
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
