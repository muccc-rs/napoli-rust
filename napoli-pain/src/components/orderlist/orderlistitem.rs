use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListEntryProps {
    pub order: napoli_lib::napoli::Order,
}

pub struct OrderListItem {}

impl Component for OrderListItem {
    type Message = ();
    type Properties = OrderListEntryProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let o = &ctx.props().order;
        let order_url = format!("/order/{}", o.id);
        let left_style = "padding-right: 1em; text-align: right;";
        let tr_style = "";
        let table_style = "padding-bottom: 1em;";
        html! {
                    <table style={table_style}>
                    <tr style={tr_style}><td style={left_style}>{"Order Details"}</td><td>
                        <Link<Route> to={Route::OrderListEntry { id: o.id }}>{ order_url }</Link<Route>>
                    </td></tr>
                    <tr style={tr_style}><td style={left_style}>{"Order Number"}</td><td>{o.id}</td></tr>
                    <tr style={tr_style}><td style={left_style}>{"Menu URL"}</td><td><a href={ o.menu_url.clone() }>{ o.menu_url.clone() }</a>
        </td></tr>
                    </table>
                }
    }
}
