use crate::models::grid_cell_content::GridCellContent;

pub fn can_swap(i1: i32, i2: i32, width: i32) -> bool {
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

pub fn is_solved(image: Vec<GridCellContent>) -> bool {
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
