use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "justify-center", "text-onedark-white", "w-full", "mx-auto", "space-y-4")}>
            <p>
                {"Here's a list of "}
                <span class={classes!("text-onedark-purple")}>{" PROJECTS "}</span>
                {" that I've done."}
            </p>
        </div>
    }
}
