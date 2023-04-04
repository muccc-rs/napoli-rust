use crate::service;
use napoli_lib::napoli as npb;
use yew::prelude::*;

pub enum Msg {
    GotOrders(Vec<npb::Order>),
    OrderFetchFailed(service::ServiceError),
}

pub enum FetchOrdersState {
    Fetching,
    Got(Vec<npb::Order>),
    Failed(service::ServiceError),
}

pub struct Page {
    orders: FetchOrdersState,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct AppConfigProps {
    pub base_url: String,
}

impl Component for Page {
    type Message = Msg;
    type Properties = AppConfigProps;

    fn create(ctx: &Context<Self>) -> Self {
        let svc = service::Napoli {
            base_url: ctx.props().base_url.clone(),
        };
        ctx.link().send_future(async move {
            match svc.get_orders().await {
                Ok(orders) => Msg::GotOrders(orders),
                Err(e) => Msg::OrderFetchFailed(e),
            }
        });
        Self {
            orders: FetchOrdersState::Fetching,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotOrders(o) => {
                self.orders = FetchOrdersState::Got(o);
                true
            }
            Msg::OrderFetchFailed(e) => {
                self.orders = FetchOrdersState::Failed(e);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.orders {
            FetchOrdersState::Fetching => html! {
                { "hold on to your butts" }
            },
            FetchOrdersState::Failed(e) => html! {
                <>
                    <h1>{ "oh shit oh fuck" }</h1>
                    { e.html() }
                </>
            },
            FetchOrdersState::Got(orders) => {
                let orders = orders.clone();
                html! {
                    <OrderList {orders} />
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListEntryProps {
    pub order: npb::Order,
}

pub struct OrderListEntry {}

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
                <a href={ o.menu_url.clone() }>{ o.menu_url.clone() }</a>
            </li>
        }
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListProps {
    pub orders: Vec<npb::Order>,
}

pub struct OrderList {}

impl Component for OrderList {
    type Message = ();
    type Properties = OrderListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let orders = ctx
            .props()
            .orders
            .iter()
            .cloned()
            .map(|order| {
                html! {
                    <OrderListEntry {order} />
                }
            })
            .collect::<Vec<_>>();
        html! {
            <ul>
            { orders }
            </ul>
        }
    }
}
