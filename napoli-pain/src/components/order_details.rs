use std::collections::HashMap;

use crate::{
    components::order_details::{
        add_order_entry_form::AddOrderEntryForm,
        live_streaming_indicator::{LiveStreamingStatus, StreamingIndicator},
    },
    router::Route,
    service::{self},
};
use futures::StreamExt;
use napoli_lib::napoli::{self as npb, ObjectId, SingleOrderReply};
use yew::prelude::*;
use yew_router::prelude::Link;

mod add_order_entry_form;
mod live_streaming_indicator;

#[derive(PartialEq, Eq, Properties)]
pub struct OrderDetailsProps {
    pub id: ObjectId,
}

pub struct OrderDetails {
    order: Option<npb::Order>,
    live_streaming_status: LiveStreamingStatus,
}

pub enum OrderDetailsMsg {
    GotOrders(Vec<npb::Order>),
    OrderFetchFailed(service::ServiceError),
    GotOrderUpdated(npb::Order),
    AddOrderEntry(npb::AddOrderEntryRequest),
    SetOrderEntryPaid { entry_id: ObjectId, paid: bool },
    RemoveOrderEntry { entry_id: ObjectId },

    StreamingConnected(tonic::Streaming<npb::SingleOrderReply>),
    GotStreamingOrderUpdate(npb::Order),
    StreamingFailed(service::ServiceError),
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

        let mut svc = service::Napoli::new(crate::BACKEND_URL.to_string());
        let id = ctx.props().id;
        ctx.link().send_future(async move {
            let res = svc.stream_order_updates(id).await;
            match res {
                Ok(stream) => Self::Message::StreamingConnected(stream),
                Err(e) => Self::Message::StreamingFailed(e),
            }
        });

        Self {
            order: None,
            live_streaming_status: LiveStreamingStatus::Connecting,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::GotOrders(o) => {
                self.order = o.into_iter().find(|order| order.id == ctx.props().id);
                true
            }
            Self::Message::GotStreamingOrderUpdate(o) => {
                self.order = Some(o);
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
            Self::Message::RemoveOrderEntry { entry_id } => {
                let mut svc = service::Napoli::new(crate::BACKEND_URL.to_string());
                let order_id = ctx.props().id;
                ctx.link().send_future(async move {
                    match svc
                        .remove_order_entry(npb::OrderEntryRequest {
                            order_id,
                            order_entry_id: entry_id,
                        })
                        .await
                    {
                        Ok(order) => Self::Message::GotOrderUpdated(order),
                        Err(e) => Self::Message::OrderFetchFailed(e),
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
            Self::Message::StreamingConnected(stream) => {
                self.live_streaming_status = LiveStreamingStatus::Connected;
                ctx.link()
                    .send_stream(stream.map(|single_order_reply_result| {
                        match single_order_reply_result {
                            Ok(SingleOrderReply { order: Some(o) }) => {
                                Self::Message::GotStreamingOrderUpdate(o)
                            }
                            Ok(SingleOrderReply { order: None }) => Self::Message::StreamingFailed(
                                service::ServiceError::from("Got empty order"),
                            ),
                            Err(e) => {
                                Self::Message::StreamingFailed(service::ServiceError::from(e))
                            }
                        }
                    }));
                true
            }
            Self::Message::StreamingFailed(e) => {
                self.live_streaming_status = LiveStreamingStatus::Error(format!("{:?}", e));
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
                        let on_paid_clicked = ctx.link().callback(|(entry_id, paid)| {
                            Self::Message::SetOrderEntryPaid { entry_id, paid }
                        });
                        let on_remove_clicked = ctx
                            .link()
                            .callback(|entry_id| Self::Message::RemoveOrderEntry { entry_id });
                        html! {
                            <li style="list-style: none">
                                <OrderEntry {order_entry} {on_paid_clicked} {on_remove_clicked}/>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>();

            let total_millicents: i64 = order
                .entries
                .iter()
                .map(|order| order.price_in_millicents)
                .sum();

            let total_str = match napoli_lib::Millicents::from_raw(total_millicents) {
                Ok(price) => {
                    let (euros, cents) = price.to_euro_tuple();
                    format!("{}.{:02}\u{00a0}€", euros, cents)
                }
                Err(e) => format!("Invalid price value: {}; Error: {:?}", total_millicents, e),
            };

            let id = order.id;
            let menu_url = order.menu_url.clone();
            let menu_url_text = menu_url.clone();

            html! {
                <div class="my-8">
                    <Link<Route> to={Route::Home} classes="btn"> {"< Back"} </Link<Route>>
                    <h1 class="mt-8">{"Order #"}{id}</h1>
                    <p>{"Menu URL: "}<a class="link" href={menu_url} target="_blank" rel="noopener noreferrer">{menu_url_text}</a></p>

                    <ul class="mt-4">
                    { order_entries }
                    </ul>
                    <AddOrderEntryForm order_id={order.id} onclick={on_add_new_order_request} />
                    <OrderSummary order_entries={order.entries.clone()} />
                    <StreamingIndicator status={self.live_streaming_status.clone()} />
                    <p>{"Total: "}{total_str}</p>
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
    RemoveOrderEntry,
}

pub struct OrderEntry {}
#[derive(PartialEq, Properties)]
pub struct OrderEntryProps {
    pub order_entry: npb::OrderEntry,
    pub on_paid_clicked: Callback<(ObjectId, bool)>,
    pub on_remove_clicked: Callback<ObjectId>,
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
                .on_paid_clicked
                .emit((order_entry.id, !order_entry.paid)),
            Self::Message::RemoveOrderEntry => ctx.props().on_remove_clicked.emit(order_entry.id),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let entry = &ctx.props().order_entry;
        let left_style = "padding-right: 1em; text-align: right;";
        let tr_style = "";

        let price_str = match napoli_lib::Millicents::from_raw(entry.price_in_millicents) {
            Ok(price) => {
                let (euros, cents) = price.to_euro_tuple();
                format!("{}.{:02}\u{00a0}€", euros, cents)
            }
            Err(e) => format!(
                "Invalid price value: {}; Error: {:?}",
                entry.price_in_millicents, e
            ),
        };

        html! {
            <table class="mb-4">
                <tr style={tr_style}><td style={left_style}>{"Person"}</td><td>{&entry.buyer}</td></tr>
                <tr style={tr_style}><td style={left_style}>{"Price"}</td><td>{price_str}</td></tr>
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
                <tr>
                    <td></td>
                    <td>
                        <button class="btn mt-2 mb-2" onclick={ctx.link().callback(|_| OrderEntryMsg::RemoveOrderEntry)}>
                            {"Remove Entry"}
                        </button>
                    </td>
                </tr>
            </table>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct OrderSummaryProps {
    pub order_entries: Vec<npb::OrderEntry>,
}

pub fn group_by(order_entries: &[npb::OrderEntry]) -> HashMap<String, usize> {
    let mut group_by = std::collections::HashMap::new();

    for order_entry in order_entries {
        let food = order_entry.food.to_ascii_lowercase().trim().to_owned();
        let group_by_entry = group_by.entry(food).or_insert(0);
        *group_by_entry += 1;
    }

    group_by
}

#[function_component(OrderSummary)]
pub fn order_summary_props(props: &OrderSummaryProps) -> Html {
    /*
    // summarize by food name
    let mut food_to_order_entries: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for order_entry in &props.order_entries {
        let food = order_entry.food.clone().to_ascii_lowercase();
        let order_entries = food_to_order_entries.entry(food).or_insert(vec![]);
        order_entries.push(order_entry.clone());
    }
    */

    let mut grouped_entries: Vec<_> = group_by(&props.order_entries).into_iter().collect();

    // call group_by
    grouped_entries.sort_by(|a, b| human_sort::compare(&a.0, &b.0));

    let food_to_order_entries_str = grouped_entries
        .iter()
        .map(|(food, count)| {
            html! {
                <div>
                    <p>{food}{" ("}{count}{")"}</p>
                </div>
            }
        })
        .collect::<Vec<_>>();

    let number_of_pizzas = props.order_entries.len();

    html! {
        <div>
            <h1>{"Summary"}</h1>
            <br />
            <p>{food_to_order_entries_str}</p>
            <p>{"Name: Hans Acker"}</p>
            <br />
            <h1>{"Number of Pizzas"}</h1>
            <p>{number_of_pizzas}</p>
        </div>
    }
}
