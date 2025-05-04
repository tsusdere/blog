mod components;

use components::homepage_component::Homepage;

fn main() {
    yew::Renderer::<Homepage>::new().render();
}
