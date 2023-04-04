mod homepage;
mod orderlistitem;
mod service;
pub const BASE_URL: &str = match option_env!("BASE_URL") {
    Some(url) => url,
    None => "http://[::1]:50051",
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    yew::Renderer::<homepage::Main>::default().render();

    Ok(())
}
