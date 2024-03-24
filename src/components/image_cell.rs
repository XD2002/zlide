use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ImageCellProps {
    pub nr: i32,
    pub image: String
}

#[function_component]
pub fn ImageCell(ImageCellProps {nr, image}: &ImageCellProps) -> Html {
    html! {
        <img src={image.clone()} height=300 width=300/>
    }
}