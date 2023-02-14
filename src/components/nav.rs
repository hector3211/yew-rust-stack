use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component]
pub fn Nav() -> Html {
    html! {
        <div>
            <ul>
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                <Link<Route> to={Route::About}>{"About"}</Link<Route>>
            </ul>
        </div>
    }
}
