use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Ticketmaster Event Scraper" }</h1>
            <h3>{ "Developed by Joseph Masson" }</h3>
        </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}