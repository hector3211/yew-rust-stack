use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about::About;
use crate::layout::Layout;

#[function_component(Home)]
fn counter() ->Html {
    let state = use_state(|| 0);

    let increment = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 2))
    };

    let decrement = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 2))
    };

    html! {
    <Layout>
        <div class="text-3xl">
            <p> {"current counter: "}{* state}</p>
            <button class="p-1 border border-solid rounded-md" onclick={increment}>{"+"}</button>
            <button class="p-1 border border-solid rounded-md" onclick={decrement}>{"-"}</button>
        </div>
    </Layout>
    }
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component]
fn HelloUser(props: &Props) -> Html {
    html!{
    <div>{"hello User: "} {props.name.clone()}</div>
    }
}

#[function_component]
fn Nav() -> Html {
    html!{
        <div>
            <ul>
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                <Link<Route> to={Route::About}>{"About"}</Link<Route>>
            </ul>
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{ <Home/>},
        Route::About => html!{<About/>},
        Route::NotFound => html!{ <h1> { "404"} </h1> },
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
