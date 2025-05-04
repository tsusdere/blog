use crate::components::homepage;
use yew::prelude::*;

pub struct Homepage;

impl Component for Homepage {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        homepage::app()
    }
}
