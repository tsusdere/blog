use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <h1>{"Hi!"}</h1>
        <div>
            <h3>{"I'm tsusdere!"}</h3>
        </div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
