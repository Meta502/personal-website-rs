use yew_router::prelude::*;
use yew::prelude::*;

use super::modules::Home;
use super::modules::Projects;
use super::modules::base::Navbar;
use super::modules::base::Footer;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Projects => html! {
            <Projects />
        },
        _ => html! { <p>{ "404" }</p> }
    }
}

pub fn goto(history: &AnyHistory, route: Route) {
    history.push(route);
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <div class={classes!("bg-onedark-gray", "min-h-screen", "font-mono", "py-16", "w-full")}>
            <BrowserRouter>
                <Navbar />
                <div class={classes!("max-w-3xl", "mx-auto", "w-full", "h-0.5", "my-4", "bg-white")} />
                <div class={classes!("w-full", "max-w-3xl", "mx-auto")}>
                    <Switch<Route> render={Switch::render(switch)} />
                </div>
                <div class={classes!("max-w-3xl", "mx-auto", "w-full", "h-0.5", "my-4", "bg-white")} />
                <Footer />
            </BrowserRouter>
        </div>
    }
}
