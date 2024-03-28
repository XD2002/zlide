use yew::{Properties, function_component, Html, use_state, MouseEvent, html};

use crate::components::image_cell::ImageCell;
use crate::components::loading::Loading;

#[derive(Properties, PartialEq)]
pub struct GridProps {
    #[prop_or(3)]
    pub width: usize
}

#[function_component]
pub fn Grid(&GridProps {width}: &GridProps) -> Html {
    let image = use_state(|| vec!["https://rustacean.net/assets/rustacean-flat-happy.svg"; 8]);
    let empty_square = use_state(|| 8);

    if image.len() < width.pow(2) {
        let mut image_clone = image.to_vec();
        image_clone.push("");
        image.set(image_clone);
    }

    let onclick_move_left = {
        let empty_square = empty_square.clone();
        let image = image.clone();
        move |_:MouseEvent| {
            if can_swap(*empty_square, (*empty_square) - 1, width as i32) {
                let mut image_vec = image.to_vec();
                image_vec.swap(*empty_square as usize, ((*empty_square) - 1) as usize);
                let new_empty_square = *empty_square - 1;
                empty_square.set(new_empty_square);
                image.set(image_vec);
            }
        }
    };

    let onclick_move_right = {
        let empty_square = empty_square.clone();
        let image = image.clone();
        move |_:MouseEvent| {
            if can_swap(*empty_square, (*empty_square) + 1, width as i32) {
                let mut image_vec = image.to_vec();
                image_vec.swap(*empty_square as usize, ((*empty_square) + 1) as usize);
                let new_empty_square = *empty_square + 1;
                empty_square.set(new_empty_square);
                image.set(image_vec);
            }
        }
    };

    let onclick_move_up = {
        let empty_square = empty_square.clone();
        let image = image.clone();
        move |_:MouseEvent| {
            if can_swap(*empty_square, (*empty_square) - width as i32, width as i32) {
                let mut image_vec = image.to_vec();
                image_vec.swap(*empty_square as usize, ((*empty_square) - width as i32) as usize);
                let new_empty_square = *empty_square - width as i32;
                empty_square.set(new_empty_square);
                image.set(image_vec);
            }
        }
    };

    let onclick_move_down = {
        let empty_square = empty_square.clone();
        let image = image.clone();
        move |_:MouseEvent| {
            if can_swap(*empty_square, (*empty_square) + width as i32, width as i32) {
                let mut image_vec = image.to_vec();
                image_vec.swap(*empty_square as usize, ((*empty_square) + width as i32) as usize);
                let new_empty_square = *empty_square + width as i32;
                empty_square.set(new_empty_square);
                image.set(image_vec);
            }
        }
    };

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
            <button onclick={onclick_move_left}>{"h"}</button>
            <button onclick={onclick_move_down}>{"j"}</button>
            <button onclick={onclick_move_up}>{"k"}</button>
            <button onclick={onclick_move_right}>{"l"}</button>
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
        _ if i1 >= width.pow(2) || i2 >= width.pow(2) => {
            false
        }
        _ if (i1 % width == width - 1 && i2 % width == 0) || (i2 % width == width - 1 && i1 % width == 0) => {
            false
        }
        (_,_) => {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_swap_same_row() {
        assert_eq!(can_swap(1,2,3), true);
    }

    #[test]
    fn test_can_swap_crossing_borders() {
        assert_eq!(can_swap(2,3,3), false);
    }

    #[test]
    fn test_can_swap_same_row_2() {
        assert_eq!(can_swap(8,7, 3), true);
    }
}