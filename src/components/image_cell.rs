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
        <img class="inset-0 w-full h-full object-cover" src={image.clone()}/>
    }
}
