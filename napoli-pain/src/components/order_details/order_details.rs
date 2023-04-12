use crate::{
    components::order_details::add_order_entry_form::AddOrderEntryForm,
    router::Route,
    service::{self},
};

use napoli_lib::napoli as npb;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(PartialEq, Eq, Properties)]
pub struct OrderDetailsProps {
    pub id: u32,
}

pub struct OrderDetails {
    order: Option<npb::Order>,
}

pub enum OrderDetailsMsg {
    GotOrders(Vec<npb::Order>),
    OrderFetchFailed(service::ServiceError),
    SetOrderEntryPaid { entry_id: u32, paid: bool },
    GotOrderUpdated(npb::Order),
    AddOrderEntry(npb::AddOrderEntryRequest),
}

impl Component for OrderDetails {
    type Message = OrderDetailsMsg;
    type Properties = OrderDetailsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut svc = service::Napoli::new(crate::BACKEND_URL.to_string());
        ctx.link().send_future(async move {
            match svc.get_orders().await {
                Ok(orders) => Self::Message::GotOrders(orders),
                Err(e) => Self::Message::OrderFetchFailed(e),
            }
        });
        Self { order: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::GotOrders(o) => {
                self.order = o.into_iter().find(|order| order.id == ctx.props().id);
                true
            }
            Self::Message::OrderFetchFailed(_e) => false,
            Self::Message::SetOrderEntryPaid { entry_id, paid } => {
                let mut svc = service::Napoli::new(crate::BACKEND_URL.to_string());
                let order_id = ctx.props().id;
                ctx.link().send_future(async move {
                    match svc.set_order_entry_paid(order_id, entry_id, paid).await {
                        Ok(order) => Self::Message::GotOrderUpdated(order),
                        Err(e) => Self::Message::OrderFetchFailed(e), // This is fine 🔥
                    }
                });
                false
            }
            OrderDetailsMsg::AddOrderEntry(add_order_entry_request) => {
                let mut svc = service::Napoli::new(crate::BACKEND_URL.to_string());
                ctx.link().send_future(async move {
                    match svc.add_order_entry(add_order_entry_request).await {
                        Ok(order) => Self::Message::GotOrderUpdated(order),
                        Err(e) => Self::Message::OrderFetchFailed(e),
                    }
                });
                false
            }
            Self::Message::GotOrderUpdated(order) => {
                self.order = Some(order);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(order) = &self.order {
            let on_add_new_order_request = ctx.link().callback(
                |add_order_entry_request: napoli_lib::napoli::AddOrderEntryRequest| {
                    Self::Message::AddOrderEntry(add_order_entry_request)
                },
            );

            let order_entries =
                order
                    .entries
                    .iter()
                    .cloned()
                    .map(|order_entry| {
                        let paid_callback = ctx.link().callback(|(entry_id, paid)| {
                            Self::Message::SetOrderEntryPaid { entry_id, paid }
                        });
                        html! {
                           <li style="list-style: none">
                           <OrderEntry {order_entry} {paid_callback}/>
                           </li>
                        }
                    })
                    .collect::<Vec<_>>();

            html! {
                <div class="my-8">
                    <Link<Route> to={Route::Home} classes="btn"> {"< Back"} </Link<Route>>
                    <ul class="mt-4">
                    { order_entries }
                    </ul>
                    <AddOrderEntryForm order_id={order.id} onclick={on_add_new_order_request} />
                </div>
            }
        } else {
            html! {
                <p> {"\u{21ba}"} </p>
            }
        }
    }
}

pub enum OrderEntryMsg {
    PayOrderEntry,
}

pub struct OrderEntry {}
#[derive(PartialEq, Properties)]
pub struct OrderEntryProps {
    pub order_entry: npb::OrderEntry,
    pub paid_callback: Callback<(u32, bool)>,
}
impl Component for OrderEntry {
    type Message = OrderEntryMsg;
    type Properties = OrderEntryProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let order_entry = ctx.props().order_entry.clone();
        match msg {
            Self::Message::PayOrderEntry => ctx
                .props()
                .paid_callback
                .emit((order_entry.id, !order_entry.paid)),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let entry = &ctx.props().order_entry;
        let left_style = "padding-right: 1em; text-align: right;";
        let tr_style = "";
        let table_style = "padding-bottom: 1em;";
        html! {
            <table style={table_style}>
            <tr style={tr_style}><td style={left_style}>{"Person"}</td><td>{&entry.buyer}</td></tr>
            <tr style={tr_style}><td style={left_style}>{"Price"}</td><td>{format!("{:.2}\u{00a0}€", entry.price)}</td></tr>
            <tr style={tr_style}><td style={left_style}>{"Food"}</td><td>{&entry.food}</td></tr>
            <tr style={tr_style}>
                <td style={left_style}>{"Paid"}</td>
                <td>
                    <button onclick={ctx.link().callback(|_| OrderEntryMsg::PayOrderEntry)}>
                        {if entry.paid {"\u{2705}"} else {"\u{274c}"}}
                    </button>
                </td>
            </tr>
            <tr style={tr_style}><td style={left_style}>{"Id"}</td><td>{&entry.id}</td></tr>
            </table>
        }
    }
}
