use yew::prelude::*;

use crate::components::image_cell::ImageCell;
use crate::components::loading::Loading;

#[derive(Properties, PartialEq)]
pub struct GridProps {
    pub width: usize
}

#[function_component]
pub fn Grid(&GridProps {width}: &GridProps) -> Html {
    let image = use_state(|| vec!["https://rustacean.net/assets/rustacean-flat-happy.svg"; 8]);
    let empty_square = use_state(|| 8);

    if image.len() < 9 {
        let mut image_clone = image.to_vec();
        image_clone.push("");
        image.set(image_clone);
    }

    /*let onclick = {
        let empty_square = empty_square.clone();
        let mut image_clone = image.to_vec();
        move |_:Event| {
            if is_valid_index(*empty_square, width as i32) {
                image_clone.swap(*empty_square as usize, (*empty_square - 1) as usize);
                let new_empty_square = *empty_square - 1;
                empty_square.set(new_empty_square);
                image.set(image_clone);
            }
        }
    };*/

    if image.len() == 9 {
    html! {
        <>
            {for (0..9).into_iter().map(|nr| {
                html! {
                    <>
                        <ImageCell _nr={nr} image={image[nr as usize]}/>
                        if (nr + 1) % width as i32 == 0 {
                            <br/>
                        }
                    </>
                }
            })}
            <button>{"hj"}</button>
        </>
    }}
    else {
        html! {
            <Loading/>
        }
    }
}

fn can_swap(i1: i32, i2: i32, width: i32) -> bool {
    match (i1, i2) {
        _ if i1 < 0 || i2 < 0 => {
            false
        }
        _ if i1 > 2_i32.pow(width as u32) || i2 > 2_i32.pow(width as u32) => {
            false
        }
        _ if i1 % width + 1 != i2 % width || i1 % width != i2 % width + 1 => {
            false
        }
        (_,_) => {
            true
        }
    }
}