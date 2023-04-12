use crate::components::homepage::Homepage;
use crate::components::order_details::order_details::OrderDetails;
use crate::components::server_name::ServerName;
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
        <div class="m-4 font-mono">
            <ServerName name={BACKEND_URL} />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </div>
    }
}
