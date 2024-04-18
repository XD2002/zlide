use yew::{Properties, function_component, Html, use_state, MouseEvent, html};
use rand::seq::SliceRandom;
use web_sys::console;
use rand::thread_rng;

use crate::components::image_cell::ImageCell;
use crate::components::loading::Loading;
use crate::models::grid_cell_content::GridCellContent;

#[derive(Properties, PartialEq)]
pub struct GridProps {
    #[prop_or(3)]
    pub width: usize
}

#[function_component]
pub fn Grid(&GridProps {width}: &GridProps) -> Html {
    let mut img_vec: Vec<GridCellContent> = Vec::new();

    let number_of_squares = width.pow(2);
    
    // initialising img_vec with static images
    for i in 0..8 {
        //img_vec.push(GridCellContent::new("https://rustacean.net/assets/rustacean-flat-happy.svg".to_string(),i))
        img_vec.push(GridCellContent::new(format!("images/image{i}.png").to_string(),i))
    }

    // use_states
    let empty_square = use_state(|| 8);
    let solved = use_state(|| false);
    let image = use_state(|| img_vec);
    
    // shuffling the images randomly + adding the empty square
    if image.len() < number_of_squares {
        let mut image_clone = image.to_vec();
        // TODO: fix shuffling
        image_clone.shuffle(&mut thread_rng());
        image_clone.push(GridCellContent::new_empty_square(8));
        image.set(image_clone);
    }
    
    // defining the functions used in the buttons used for movement of the empty square
    let onclick_move_left = {
        let empty_square = empty_square.clone();
        let image = image.clone();
        let solved = solved.clone();
        move |_:MouseEvent| {
            if can_swap(*empty_square, (*empty_square) - 1, width as i32) {
                let mut image_vec = image.to_vec();
                image_vec.swap(*empty_square as usize, ((*empty_square) - 1) as usize);
                let new_empty_square = *empty_square - 1;
                empty_square.set(new_empty_square);
                image.set(image_vec.clone());
                let mut solved_change = *solved;
                if is_solved(image_vec.clone()) {
                    solved_change = true;
                }
                solved.set(solved_change);
            }
        }
    };

    let onclick_move_right = {
        let empty_square = empty_square.clone();
        let image = image.clone();
        let solved = solved.clone();
        move |_:MouseEvent| {
            if can_swap(*empty_square, (*empty_square) + 1, width as i32) {
                let mut image_vec = image.to_vec();
                image_vec.swap(*empty_square as usize, ((*empty_square) + 1) as usize);
                let new_empty_square = *empty_square + 1;
                empty_square.set(new_empty_square);
                image.set(image_vec.clone());
                let mut solved_change = *solved;
                if is_solved(image_vec.clone()) {
                    solved_change = true;
                }
                solved.set(solved_change);
            }
        }
    };

    let onclick_move_up = {
        let empty_square = empty_square.clone();
        let image = image.clone();
        let solved = solved.clone();
        move |_:MouseEvent| {
            if can_swap(*empty_square, (*empty_square) - width as i32, width as i32) {
                let mut image_vec = image.to_vec();
                image_vec.swap(*empty_square as usize, ((*empty_square) - width as i32) as usize);
                let new_empty_square = *empty_square - width as i32;
                empty_square.set(new_empty_square);
                image.set(image_vec.clone());
                let mut solved_change = *solved;
                if is_solved(image_vec.clone()) {
                    solved_change = true;
                }
                solved.set(solved_change);
            }
        }
    };

    let onclick_move_down = {
        let empty_square = empty_square.clone();
        let image = image.clone();
        let solved = solved.clone();
        move |_:MouseEvent| {
            if can_swap(*empty_square, (*empty_square) + width as i32, width as i32) {
                let mut image_vec = image.to_vec();
                image_vec.swap(*empty_square as usize, ((*empty_square) + width as i32) as usize);
                let new_empty_square = *empty_square + width as i32;
                empty_square.set(new_empty_square);
                image.set(image_vec.clone());
                let mut solved_change = *solved;
                if is_solved(image_vec.clone()) {
                    solved_change = true;
                }
                solved.set(solved_change);
            }
        }
    };


    if image.len() == number_of_squares {
        html! {
            <>
                <div class="container auto grid grid-cols-3">
                    {for (0..number_of_squares).into_iter().map(|nr| {
                        html! {
                            <ImageCell image={image[nr].clone().img}/>
                        }
                    })}
                </div>
                <button class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm p-2.5 text-center inline-flex items-center me-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={onclick_move_left}>{"left"}</button>
                <button class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm p-2.5 text-center inline-flex items-center me-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={onclick_move_down}>{"down"}</button>
                <button class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm p-2.5 text-center inline-flex items-center me-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={onclick_move_up}>{"up"}</button>
                <button class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm p-2.5 text-center inline-flex items-center me-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={onclick_move_right}>{"right"}</button>
                if *solved{
                    <p>{"solved"}</p>
                }
            </>
        }
    } else {
        html! {
            <Loading/>
        }
    }
}

fn can_swap(i1: i32, i2: i32, width: i32) -> bool {
    let number_of_squares = width.pow(2);
    match (i1, i2) {
        _ if i1 < 0 || i2 < 0 => {
            false
        }
        _ if i1 >= number_of_squares || i2 >= number_of_squares => {
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

fn is_solved(image: Vec<GridCellContent>) -> bool {
    for i in 0..9 {
        if image[i].correct_spot != i as i32 {
            return false
        }
    }
    return true
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
