
use crate::service;
use napoli_lib::napoli as npb;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct OrderListItemProps {
    pub id: String,
}

pub struct OrderListItem {}

impl Component for OrderListItem {
    type Message = ();
    type Properties = OrderListItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<p> {&ctx.props().id} </p>}
        // let order_entries = ctx
        //     .props()
        //     .order
        //     .entries
        //     .iter()
        //     .cloned()
        //     .map(|entry| {
        //         html! {
        //            <li>
        //            <OrderEntry {entry} />
        //            </li>
        //         }
        //     })
        //     .collect::<Vec<_>>();
        // html! {
        //     <ul>
        //     { order_entries }
        //     </ul>
        // }
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
        html! {
            <table>
            <tr>
            <td>
            {"Name:"}
            </td>
            <td>
            {entry.buyer.clone()}
            </td>
            </tr>
            </table>

        }
    }
}
