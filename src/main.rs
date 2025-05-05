mod app;
mod components;

use app::App;
use yew::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <App />
    }
}

fn main() {
    //yew::Renderer::<Homepage>::new().render();
    yew::Renderer::<Root>::new().render();
}
