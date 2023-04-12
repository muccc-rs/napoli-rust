use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NewOrderFormProps {
    pub onclick: Callback<String>,
}

#[function_component(NewOrderForm)]
pub fn new_order_form(props: &NewOrderFormProps) -> Html {
    let menu_url = use_state(|| "".to_string());

    let mu_clone = menu_url.clone();

    let onclick = props.onclick.reform(move |_| mu_clone.to_string());
    html! {
        <form class="my-8" onsubmit={move |e: SubmitEvent| { e.prevent_default() }}>
            <label for="menu_url" class="mr-4">{"Menu URL:"}</label>
            <input
                id="menu_url"
                name="menu_url"
                type="url"
                placeholder="https://..."
                value={menu_url.to_string()}
                oninput={move |e: InputEvent| {
                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    menu_url.set(input.value());
                }}
                class="textinput"
                />
            <input
                type="submit"
                value="Open new order"
                class="btn"
                {onclick}/>
        </form>
    }
}
