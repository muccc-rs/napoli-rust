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
        let left_classes = "pr-4 text-right";
        let timestamp = o.timestamp.clone();

        html! {
            <table class="mb-4">
                <tr><td class={left_classes}>{"Order Details"}</td><td>
                    <Link<Route>
                        to={Route::OrderListEntry { id: o.id }}
                        classes="link">
                        { order_url }
                    </Link<Route>>
                </td></tr>
                <tr><td class={left_classes}>{"Order Number"}</td><td>{o.id}</td></tr>
                <tr><td class={left_classes}>{"Timestamp"}</td><td>{timestamp}</td></tr>
                <tr><td class={left_classes}>{"Menu URL"}</td><td><a class="link" target="_blank" rel="noopener noreferrer" href={ o.menu_url.clone() }>{ o.menu_url.clone() }</a></td></tr>
                <tr><td class={left_classes}>{"# of entries"}</td><td>{o.entries.len()}</td></tr>
            </table>
        }
    }
}
