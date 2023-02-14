use yew::prelude::*;
use crate::layout::Layout;

#[function_component]
pub fn About() -> Html {
    html! {
    <Layout>
        <div>
            {"about page"} 
        </div>
    </Layout>
    }
}
