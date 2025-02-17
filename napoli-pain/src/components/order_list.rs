use yew::prelude::*;

mod order_list_item;
use self::order_list_item::OrderListItem;
use crate::components::toast::{Toast, ToastKind};

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListProps {
    pub orders: Vec<napoli_lib::napoli::Order>,
}

pub struct OrderList {}

impl Component for OrderList {
    type Message = ();
    type Properties = OrderListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().orders.is_empty() {
            return html! {
                <p>{"No orders found"}</p>
            };
        }

        let orders = ctx
            .props()
            .orders
            .iter()
            .cloned()
            .map(|order| {
                html! {
                    <OrderListItem {order} />
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
