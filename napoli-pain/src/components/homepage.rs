use crate::{components::new_order_form::NewOrderForm, service};
use napoli_lib::napoli as npb;
use yew::prelude::*;

use crate::components::orderlist::orderlist::OrderList;

pub enum Msg {
    GotOrders(Vec<npb::Order>),
    OrderFetchFailed(service::ServiceError),
    AddOrder(String),
}
#[derive(Clone)]
pub enum FetchOrdersState {
    Fetching,
    Got(Vec<npb::Order>),
    Failed(service::ServiceError),
}

pub struct Homepage {
    orders: FetchOrdersState,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct AppConfigProps {
    pub backend_url: String,
}

impl Component for Homepage {
    type Message = Msg;
    type Properties = AppConfigProps;

    fn create(ctx: &Context<Self>) -> Self {
        let svc = service::Napoli {
            backend_url: ctx.props().backend_url.clone(),
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
            Msg::AddOrder(menu_url) => {
                let svc = service::Napoli {
                    backend_url: _ctx.props().backend_url.clone(),
                };
                let orders = self.orders.clone();
                _ctx.link().send_future(async move {
                    match svc.create_order(menu_url).await {
                        Ok(order) => match orders {
                            FetchOrdersState::Got(orders) => {
                                let mut orders = orders;
                                orders.insert(0, order);
                                Msg::GotOrders(orders)
                            }
                            _ => Msg::GotOrders(vec![order]),
                        },
                        Err(e) => Msg::OrderFetchFailed(e),
                    }
                });
                self.orders = FetchOrdersState::Fetching;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_create_new_order = ctx.link().callback(|menu_url| Msg::AddOrder(menu_url));

        match &self.orders {
            FetchOrdersState::Fetching => html! {
                <h1>{ "hold on to your butts" }</h1>
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
                    <>
                    <NewOrderForm onclick={on_create_new_order} />
                    <OrderList {orders} />
                    </>
                }
            }
        }
    }
}
