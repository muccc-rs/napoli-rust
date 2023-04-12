use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AddOrderEntryFormProps {
    pub order_id: u32,
    pub onclick: Callback<napoli_lib::napoli::AddOrderEntryRequest>,
}

#[function_component(AddOrderEntryForm)]
pub fn add_order_entry_form(props: &AddOrderEntryFormProps) -> Html {
    let order_id = props.order_id;
    let food = use_state(|| "".to_string());
    let buyer = use_state(|| "".to_string());
    let price = use_state(|| "".to_string());

    let is_food_valid = food.len() >= 2;
    let is_buyer_valid = buyer.len() >= 2;
    let is_price_valid = price.parse::<f64>().is_ok();
    let is_form_valid = is_food_valid && is_buyer_valid && is_price_valid;

    let food_str = food.to_string();
    let buyer_str = buyer.to_string();
    let price_str = price.to_string();

    html! {
        <div class="pt-8">
            <h1>{ "Add Entry To Order" }</h1>
            <form style="my-8" onsubmit={move |e: SubmitEvent| { e.prevent_default() }}>
            <div class="mb-1">
                <label for="food">{"Food:"}</label>
                <input
                    id="food"
                    class="textinput ml-2"
                    name="food"
                    type="text"
                    minlength=2
                    placeholder="Food"
                    required=true
                    value={food.to_string()}
                    oninput={move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        food.set(input.value());
                    }}/>
                </div>
                <div class="mb-1">
                    <label for="buyer">{"Buyer:"}</label>
                    <input
                        id="buyer"
                        class="textinput ml-2"
                        name="buyer"
                        type="text"
                        minlength=2
                        placeholder="Buyer"
                        required=true
                        value={buyer.to_string()}
                        oninput={move |e: InputEvent| {
                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                            buyer.set(input.value());
                        }}/>
                </div>
                <div class="mb-2">
                    <label for="price">{"Price:"}</label>
                    <input
                        id="price"
                        class="textinput ml-2"
                        name="price"
                        type="number"
                        placeholder="Price"
                        required=true
                        value={price.to_string()}
                        oninput={move |e: InputEvent| {
                            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                            price.set(input.value());
                        }}/>
                </div>
                <input
                    type="submit"
                    class="btn"
                    disabled={!is_form_valid}
                    value="Add Order Entry"
                    onclick={props.onclick.reform(move |_| napoli_lib::napoli::AddOrderEntryRequest {
                        order_id: order_id,
                        food: food_str.clone(),
                        buyer: buyer_str.clone(),
                        price: price_str.parse::<f64>().unwrap_or(0.0),
                })}/>
            </form>
        </div>
    }
}
