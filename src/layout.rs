use yew::prelude::*;
use crate::components::nav::Nav;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component]
pub fn Layout(props: &LayoutProps) -> Html {
    html!{
        <div class="min-h-screen max-w-full bg-zinc-700">
            <Nav/>
            {for props.children.iter()}
        </div>
    }
}

