use yew::prelude::*;

mod components;

use crate::components::grid::Grid;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Grid width=3/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
