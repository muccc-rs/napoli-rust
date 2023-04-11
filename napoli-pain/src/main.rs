mod components;
mod homepage;
mod orderlistitem;
mod router;
mod service;

pub const BACKEND_URL: &str = match option_env!("BACKEND_URL") {
    Some(url) => url,
    None => "http://[::1]:50051",
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    yew::Renderer::<router::Router>::default().render();

    Ok(())
}
