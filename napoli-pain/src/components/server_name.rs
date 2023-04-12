use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ServerNameProps {
    pub name: String,
}

#[function_component(ServerName)]
pub fn server_name(props: &ServerNameProps) -> Html {
    html! {
        <p>{"Server Name: "}{&props.name}</p>
    }
}
