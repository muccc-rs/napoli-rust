#[derive(PartialEq, Clone)]
pub enum LiveStreamingStatus {
    Error(String),
    Connecting,
    Connected,
}

#[derive(yew::Properties, PartialEq, Clone)]
pub struct StreamingIndicatorProps {
    pub status: LiveStreamingStatus,
}

#[yew::function_component(StreamingIndicator)]
pub fn streaming_indicator(props: &StreamingIndicatorProps) -> yew::Html {
    let (status_symbol, status_message, hover_text) = match &props.status {
        LiveStreamingStatus::Error(msg) => {
            ("🔴", "Live updates inactive", format!("Error: {}", msg))
        }
        LiveStreamingStatus::Connecting => ("🟡", "Live updates connecting", String::new()),
        LiveStreamingStatus::Connected => ("🟢", "Live updates active", String::new()),
    };

    yew::html!(
        <div class="fixed top-4 right-4" title={hover_text}>
            {status_message}{" "}{status_symbol}
        </div>
    )
}
