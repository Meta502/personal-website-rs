use yew::prelude::*;

use super::components::{
    SocialMedia,
    Info,
};

#[function_component(Footer)]
pub fn footer() -> Html {    
    html! {
        <div class={classes!("w-full", "max-w-4xl", "space-y-2", "py-2")}>
            <Info />
            <SocialMedia />
        </div>
    }
}