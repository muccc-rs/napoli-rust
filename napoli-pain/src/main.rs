use yew::prelude::*;

mod service;
mod homepage;

#[function_component(App)]
fn app() -> Html {
    let svc = service::Napoli {};
    let orders = svc.get_orders().unwrap();
    html! {
        <homepage::OrderList {orders} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}