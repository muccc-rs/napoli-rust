mod homepage;
mod service;

const BASE_URL: &str = match option_env!("BASE_URL") {
    Some(url) => url,
    None => "http://[::1]:50051",
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    yew::Renderer::<homepage::Page>::with_props(homepage::AppConfigProps {
        base_url: BASE_URL.to_string(),
    })
    .render();

    Ok(())
}
