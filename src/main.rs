#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod components;
mod context;
mod home;
mod reducer;

use context::InvoiceProvider;
use home::Home;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <InvoiceProvider>
            <Home />
        </InvoiceProvider>
    }
}

fn main() {
    yew::start_app::<App>();
}
