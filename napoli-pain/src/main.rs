use yew::prelude::*;

mod service;

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListEntryProps {
    order: service::Order,
}

struct OrderListEntry {
}

impl Component for OrderListEntry {
    type Message = ();
    type Properties = OrderListEntryProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let o = &ctx.props().order;
        html! {
            <li>
                { "Order number " }
                { o.id }
                { ", menu: " }
                <a href={ o.menu_url }>{ o.menu_url }</a>
            </li>
        }
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListProps {
    orders: Vec<service::Order>,
}

struct OrderList {
}

impl Component for OrderList {
    type Message = ();
    type Properties = OrderListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let orders = ctx.props().orders.iter().cloned().map(|order| {
            html! {
                <OrderListEntry {order} />
            }
        }).collect::<Vec<_>>();
        html! {
            <ul>
            { orders }
            </ul>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let svc = service::Napoli {};
    let orders = svc.get_orders().unwrap();
    html! {
        <OrderList {orders} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}