use yew::{Properties, function_component, Html, html};

#[derive(Properties, PartialEq)]
pub struct ImageCellProps {
    #[prop_or_default]
    pub _nr: i32,
    #[prop_or_default]
    pub image: String
}

#[function_component]
pub fn ImageCell(ImageCellProps {_nr, image}: &ImageCellProps) -> Html {
    html! {
        <img src={image.clone()} height=300 width=300/>
    }
}