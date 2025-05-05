use crate::components::homepage_component::Homepage;
use yew::prelude::*;

#[function_component(App)]
pub(crate) fn app() -> Html {
    html! {
        <Homepage/>
    }
}
