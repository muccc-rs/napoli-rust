
use crate::service;
use napoli_lib::napoli as npb;
use yew::prelude::*;
use crate::homepage::Msg;

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListItemProps {
    pub id: String,
}

pub struct OrderListItem {
    order: Option<npb::Order>,
}

impl Component for OrderListItem {
    type Message = Msg;
    type Properties = OrderListItemProps;

    fn create(ctx: &Context<Self>) -> Self {
         let svc = service::Napoli {
            base_url: crate::BASE_URL.to_string(),
        };
        ctx.link().send_future(async move {
            match svc.get_orders().await {
                Ok(orders) => Msg::GotOrders(orders),
                Err(e) => Msg::OrderFetchFailed(e),
            }
        });
        Self {
            order: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotOrders(o) => {
                self.order = o.into_iter().find(|order| order.id.to_string() == ctx.props().id);
                true
            }
            Msg::OrderFetchFailed(_e) => {
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(order) = &self.order {
            let order_entries = order
                .entries
                .iter()
                .cloned()
                .map(|entry| {
                    html! {
                       <li style="list-style: none">
                       <OrderEntry order_entry={entry} />
                       </li>
                    }
                })
                .collect::<Vec<_>>();
            html! {
                <ul>
                { order_entries }
                </ul>
            }
        } else {
            html! {
                <p> {"\u{21ba}"} </p>
            }
        }
    }
}

pub struct OrderEntry {}
#[derive(PartialEq, Eq, Properties)]
pub struct OrderEntryProps {
    pub order_entry: npb::OrderEntry,

}
impl Component for OrderEntry {
    type Message = ();
    type Properties = OrderEntryProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
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
            <tr style={tr_style}><td style={left_style}>{"Paid"}</td><td>{if entry.paid {"\u{2705}"} else {"\u{274c}"}}</td></tr>
            <tr style={tr_style}><td style={left_style}>{"Id"}</td><td>{&entry.id}</td></tr>
            </table>
        }
    }
}
