use yew::{create_portal, function_component, html, Html, Properties};

#[derive(Clone, PartialEq)]
pub enum ToastKind {
    Error,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: String,
    pub kind: ToastKind,
}

#[function_component]
pub fn Toast(props: &Props) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("toast_host")
        .expect("Expected to find a #toast_host element");

    // See: https://flowbite.com/docs/components/toast/
    create_portal(
        html!(
            <div id="toast-simple" class="flex items-center w-full max-w-xs p-4 space-x-4 text-gray-500 bg-gray-50 divide-x divide-gray-200 rounded-lg shadow-md" role="alert">
            <ErrorIcon />
            <div class="ps-4 text-sm font-normal">{props.message.clone()}</div>
            </div>
        ),
        modal_host.into(),
    )
}

#[function_component]
pub fn ErrorIcon() -> Html {
    html!(
        <div class="inline-flex items-center justify-center shrink-0 w-8 h-8 text-red-500 bg-red-100 rounded-lg dark:bg-red-800 dark:text-red-200">
        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 11.793a1 1 0 1 1-1.414 1.414L10 11.414l-2.293 2.293a1 1 0 0 1-1.414-1.414L8.586 10 6.293 7.707a1 1 0 0 1 1.414-1.414L10 8.586l2.293-2.293a1 1 0 0 1 1.414 1.414L11.414 10l2.293 2.293Z"/>
        </svg>
        <span class="sr-only">{"Error icon"}</span>
    </div>
    )
}

#[function_component]
pub fn ToastHost() -> Html {
    html! {
        <div id="toast_host" class="fixed bottom-4 right-4 z-50"></div>
    }
}
