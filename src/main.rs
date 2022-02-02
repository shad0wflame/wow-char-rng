mod state;
mod components;

use gloo_utils::body;
use web_sys::CssStyleDeclaration;
use yew::prelude::*;
use components::item::Item;

#[function_component(App)]
fn app() -> Html {
    let body_style: CssStyleDeclaration  = body().style();

    // TODO: Get the number randomly in a range between 0 and 4.
    // TODO: Handle Result gracefully.
    body_style.set_property("background-image", &format!("url(assets/images/backgrounds/{}.jpg)", 1));
    body_style.set_property("background-size", "cover");

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
