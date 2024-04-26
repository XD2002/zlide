use crate::models::grid_cell_content::GridCellContent;
use rand::{Rng, thread_rng};
use web_sys::js_sys::Math::sqrt;
use crate::tools::grid_checks::can_swap;

pub fn shuffle_grid(image: &mut Vec<GridCellContent>, empty_square: &mut i32){
    let number_of_squares = image.len() as i32;
    let width: i32 = sqrt(number_of_squares as f64) as i32;
    let mut number_of_swaps = 0;
    while number_of_swaps < 100 {
        let possible_tiles = [*empty_square-width, *empty_square-1, *empty_square+1, *empty_square+width];
        let tile_to_swap = possible_tiles[thread_rng().gen_range(0..4)];
        if can_swap(tile_to_swap, *empty_square, width) {
            image.swap(tile_to_swap as usize, *empty_square as usize);
            *empty_square = tile_to_swap;
            number_of_swaps += 1;
        }
    }
}