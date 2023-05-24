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
    let (dot, message, hover) = match &props.status {
        LiveStreamingStatus::Error(msg) => (
            "🔴",
            "Live updates inactive".to_string(),
            Some(format!("Error: {}", msg)),
        ),
        LiveStreamingStatus::Connecting => ("🟡", "Live updates connecting".to_string(), None),
        LiveStreamingStatus::Connected => ("🟢", "Live updates active".to_string(), None),
    };

    yew::html!(
        <div class="fixed top-4 right-4" title={hover.unwrap_or(String::new())}>
            {message}{" "}{dot}
        </div>
    )
}
