mod state;
mod components;

use gloo::console::__macro::JsValue;
use gloo_utils::body;
use rand::{Rng, thread_rng};
use web_sys::CssStyleDeclaration;
use yew::prelude::*;
use components::item::Item;

fn set_rand_background(style: &CssStyleDeclaration) -> Result<(), JsValue> {
    let mut rng = thread_rng();
    let image_index: u8 = rng.gen_range(0..=4);

    style.set_property("background-image", &format!("url(assets/images/backgrounds/{}.jpg)", image_index))?;
    style.set_property("background-size", "cover")?;

    Ok(())
}

#[function_component(App)]
fn app() -> Html {

    match set_rand_background(&body().style()) {
        Err(e) => eprintln!("{:?}", e),
        _ => ()
    }

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
                            <Item text={"Horde"} icon={"2px"} class_name={"faction horde"} />
                        </div>
                        <div id="faction" class="col-4">
                            <Item text={"Dark Iron Dwarf"} icon={"-485px -160px"} class_name={"race"} />
                        </div>
                        <div id="race" class="col-4">
                            <Item text={"Brewmaster Monk"} icon={"0px -75px"} class_name={"class"} />
                        </div>
                    </div>
                    <div class="row align-content-end h-25">
                        <div class="offset-md-4 col-4">
                            <button type="button"
                                    class="d-block mx-auto wow-btn"
                            >
                                {"Repeat!"}
                            </button>
                        </div>
                        <div class="col-4">
                            <button id="filter-btn"
                                    type="button"
                                    class="d-block mx-auto wow-btn"
                                    data-toggle="modal"
                                    data-target="#filterModal"
                            >
                                {"Filters"}
                            </button>
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
