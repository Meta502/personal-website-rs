use yew::prelude::*;

use super::data::Data;

#[function_component(SocialMedia)]
pub fn social_media() -> Html {
    let data = Data::new();
    let social_media = data.get_social_items();

    html! {
        <div class={classes!("flex", "justify-center", "items-center", "space-x-4")}>
            { social_media.iter().map(|x| {
                let data = x.clone();
                html! {
                    <a href={data.href.clone()} target="_blank" class={classes!("flex", "space-x-2", "text-onedark-white", "font-mono", "hover:underline")}>
                        {data.image.clone()}     
                        <p>{data.name.clone()}</p>
                    </a>
                }
            }).collect::<Html>() }
        </div>
    }
}

#[function_component(Info)]
pub fn info() -> Html {
    html! {
        <div>
            <p class={classes!("text-onedark-white", "text-center")}>
                {"This website is entirely written entirely* in "}
                <span class={classes!("text-onedark-yellow")}>{"Rust"}</span>
                {"!"}
            </p>
            <p class={classes!("text-onedark-white", "text-center", "text-[0.8rem]")}>
                <small>{r"*Technically it still builds to WASM + JS, but ah well ¯\_(ツ)_/¯"}</small>
            </p>
        </div>
    }
}