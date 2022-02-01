use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
         <>
            <div class="container h-100">
                <div class="row h-100 align-items-center justify-content-center">
                    <div class="item-container">
                        <div class="row align-content-center h-25 justify-content-center">
                            <p class="title">
                                {"Title"}
                            </p>
                        </div>
                        <div class="row align-content-end h-50">
                            <div id="class" class="col-4">
                                {"Faction Item"}
                            </div>
                            <div id="faction" class="col-4">
                                {"Race Item"}
                            </div>
                            <div id="race" class="col-4">
                                {"Class Item"}
                            </div>
                        </div>
                        <div class="row align-content-end h-25">
                            <div class="col-4"/>
                            <div class="col-4">
                                <button type="button"
                                        class="d-block mx-auto wow-btn">
                                    {"Reset"}
                                </button>
                            </div>
                            <div class="col-4">
                                <button id="filter-btn" type="button"
                                        class="d-block mx-auto wow-btn"
                                        data-toggle="modal"
                                        data-target="#filterModal">
                                    {"Filters"}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
