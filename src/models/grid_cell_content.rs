#[derive(Clone)]
pub struct GridCellContent {
    pub img: String,
    pub correct_spot: i32
}

impl GridCellContent {
    pub fn new(img: String, correct_spot: i32) -> GridCellContent {
        GridCellContent{img,correct_spot}
    }
}