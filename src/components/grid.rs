use yew::prelude::*;

use crate::components::image_cell::ImageCell;
#[function_component]
pub fn Grid() -> Html {
    html! {
        <>
            { for (0..9).into_iter().map(|nr| {
                html! {
                    <>
                        <ImageCell nr={nr} image={"https://rustacean.net/assets/rustacean-flat-happy.svg".clone()}/>
                        if (nr + 1) % 3 == 0 {
                            <br/>
                        }
                    </>
                }
            })}
        </>
    }
}