use yew::prelude::*;
use crate::service;

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListEntryProps {
    pub order: service::Order,
}

pub struct OrderListEntry {
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
    pub orders: Vec<service::Order>,
}

pub struct OrderList {
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
