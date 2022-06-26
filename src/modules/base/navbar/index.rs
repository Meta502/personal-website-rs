use yew::prelude::*;
use yew_router::history::{History, Location};
use yew_router::{
    hooks::use_history,
};

use crate::router;
use crate::router::{Route};

use super::schemas::NavbarItem;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "justify-center", "items-center", "font-mono", "text-onedark-white")}>
            <div class={classes!("flex", "flex-col", "justify-center", "items-center", "space-y-2")}>
                <img class={classes!("w-12")} src="https://ardizza.tech/icons/logo.png" />
                <h1 class={classes!("text-2xl")}>{"Adrian Ardizza"}</h1>
            </div>
        </div>
    }
}

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    let on_navbar_click = {
        let history = use_history().unwrap();

        Callback::from(move |route: Route| {
            router::goto(&history, route)
        })
    };

    let routes = vec!(
        NavbarItem {
            name: "About".to_string(),
            href: Route::Home,
        },
        NavbarItem {
            name: "Projects".to_string(),
            href: Route::Projects,
        },
    );


    html! {
        <div class={classes!("flex", "text-onedark-white", "space-x-4")}>
            { routes.iter().map(|x| {
                let history = use_history().unwrap();

                let on_navbar_select = {
                    let on_click = on_navbar_click.clone();
                    let route = x.href.clone();
                    Callback::from(move |_| {
                        on_click.emit(route.clone())
                    })
                };

                let current = history.location().route::<Route>().unwrap() == x.href;

                html! {
                    <button class={classes!(
                        "hover:text-onedark-blue",
                        current.then(|| Some("text-onedark-blue h-full"))
                    )} onclick={on_navbar_select}>{ &x.name }</button>
                }
            }).collect::<Html>() }
        </div>
    }
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <>
            <div class={classes!("max-w-4xl", "w-full", "flex", "flex-col", "justify-center", "items-center", "space-y-2")}>
                <Header />
                <NavigationMenu />
            </div>
        </>
    }
}
