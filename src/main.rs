use yew::prelude::*;

mod modules;
mod router;

use router::*;

enum Msg {}

struct Model;

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Main />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}