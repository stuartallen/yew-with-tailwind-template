mod pages {
    pub mod tailwind_page;
}
mod components {
    pub mod logo;
}

use pages::tailwind_page::TailwindPage;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! { <TailwindPage /> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
