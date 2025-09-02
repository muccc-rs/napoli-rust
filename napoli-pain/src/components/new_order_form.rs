use time::{
    format_description::well_known::{iso8601, Iso8601},
    formatting, OffsetDateTime,
};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NewOrderFormProps {
    pub onclick: Callback<(String, Option<OffsetDateTime>)>,
}

#[function_component(NewOrderForm)]
pub fn new_order_form(props: &NewOrderFormProps) -> Html {
    let menu_url = use_state(|| "".to_string());
    let cutoff_time = use_state(|| "".to_string());

    let mu_clone = menu_url.trim().to_owned();
    let (cutoff_time_clone, is_valid_cutoff_time) = if cutoff_time.len() == 0 {
        (None, true)
    } else {
        let res = OffsetDateTime::parse(&cutoff_time.trim().to_owned(), &Iso8601::DEFAULT);
        (res.ok(), res.is_ok())
    };

    let onclick = props
        .onclick
        .reform(move |_| (mu_clone.clone(), cutoff_time_clone));
    html! {
        <form class="my-8" onsubmit={move |e: SubmitEvent| { e.prevent_default() }}>
            <label for="menu_url" class="mr-4">{"Menu URL:"}</label>
            <input
                id="menu_url"
                name="menu_url"
                type="url"
                placeholder="https://..."
                value={menu_url.to_string()}
                maxlength="210"
                oninput={move |e: InputEvent| {
                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    menu_url.set(input.value());
                }}
                class="textinput"
                />
            <input
                id="cutoff_time"
                name="cutoff_time"
                type="datetime-local"
                oninput={move |e: InputEvent| {
                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    cutoff_time.set(input.value());
                }}
                />
            <input
                type="submit"
                value="Open new order"
                class="btn"
                {onclick}/>
        </form>
    }
}
