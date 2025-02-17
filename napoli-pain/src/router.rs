use crate::components::homepage::Homepage;
use crate::components::order_details::OrderDetails;
use crate::components::server_name::ServerName;
use crate::components::toast::ToastHost;
use crate::BACKEND_URL;

use napoli_lib::napoli::ObjectId;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/order/:id")]
    OrderListEntry { id: ObjectId },
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
            <ToastHost />
            <ServerName name={BACKEND_URL} />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </div>
    }
}
