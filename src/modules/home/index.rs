use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "justify-center", "text-onedark-white", "w-full", "mx-auto", "space-y-4")}>
            <p>
                <span>{"Hello! My name is Adrian Ardizza. "}</span>
                <span>{"I'm a "}</span>
                <span class={classes!("text-onedark-purple")}>{"Full-Stack Developer and Computer Science Student "}</span>
                <span>{"currently studying at "}</span>
                <span class={classes!("text-onedark-blue")}>{"Fasilkom UI."}</span>
            </p>
            <p>
                <span>{"I have 5 years of experience in web development, with extensive knowledge in using both frontend and backend technologies to build applications."}</span>
            </p>
        </div>
    }
}
