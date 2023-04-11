use crate::components::homepage::Homepage;
use crate::components::orderdetails::OrderDetails;
use crate::BACKEND_URL;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/order/:id")]
    OrderListEntry { id: u32 },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Homepage backend_url={BACKEND_URL} /> },
        Route::OrderListEntry { id } => html! {
            <OrderDetails id={id} />
        },
    }
}

#[function_component(Router)]
pub fn app() -> Html {
    html! {
        <div style="font-family: monospace">
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </div>
    }
}
