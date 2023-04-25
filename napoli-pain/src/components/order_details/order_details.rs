use crate::{
    components::order_details::add_order_entry_form::AddOrderEntryForm,
    router::Route,
    service::{self},
};

use napoli_lib::napoli::{self as npb, ObjectId};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(PartialEq, Eq, Properties)]
pub struct OrderDetailsProps {
    pub id: ObjectId,
}

pub struct OrderDetails {
    order: Option<npb::Order>,
}

pub enum OrderDetailsMsg {
    GotOrders(Vec<npb::Order>),
    OrderFetchFailed(service::ServiceError),
    GotOrderUpdated(npb::Order),
    AddOrderEntry(npb::AddOrderEntryRequest),
    SetOrderEntryPaid { entry_id: ObjectId, paid: bool },
    RemoveOrderEntry { entry_id: ObjectId },
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

            let id = order.id;
            let menu_url = order.menu_url.clone();
            let menu_url_text = menu_url.clone();

            html! {
                <div class="my-8">
                    <Link<Route> to={Route::Home} classes="btn"> {"< Back"} </Link<Route>>
                    <h1 class="mt-8">{"Order #"}{id}</h1>
                    <p>{"Menu URL: "}<a class="link" href={menu_url}>{menu_url_text}</a></p>

                    <ul class="mt-4">
                    { order_entries }
                    </ul>
                    <AddOrderEntryForm order_id={order.id} onclick={on_add_new_order_request} />
                    <OrderSummary order_entries={order.entries.clone()} />
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

        let price_in_euros = entry.price_in_millicents as f64 / 1000.0;

        html! {
            <table class="mb-4">
                <tr style={tr_style}><td style={left_style}>{"Person"}</td><td>{&entry.buyer}</td></tr>
                <tr style={tr_style}><td style={left_style}>{"Price"}</td><td>{format!("{:.2}\u{00a0}€", price_in_euros)}</td></tr>
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

#[function_component(OrderSummary)]
pub fn order_summary_props(props: &OrderSummaryProps) -> Html {
    // summarize by food name

    let mut food_to_order_entries: std::collections::HashMap<String, Vec<npb::OrderEntry>> =
        std::collections::HashMap::new();

    for order_entry in &props.order_entries {
        let food = order_entry.food.clone().to_ascii_lowercase();
        let order_entries = food_to_order_entries.entry(food).or_insert(vec![]);
        order_entries.push(order_entry.clone());
    }

    let food_to_order_entries_str = food_to_order_entries
        .iter()
        .map(|(food, order_entries)| {
            html! {
                <div>
                    <p>{food}{"("}{order_entries.len()}{")"}</p>
                    <ul>
                        {order_entries.iter().map(|order_entry| html! {
                            <li>{format!("{}", order_entry.buyer)}</li>
                        }).collect::<Vec<_>>()}
                    </ul>
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
